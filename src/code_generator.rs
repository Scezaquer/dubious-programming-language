use crate::ast_build::{
    AssignmentIdentifier, AssignmentOp, Ast, Atom, BinOp, Constant, Expression, Function, Literal,
    Program, ReassignmentIdentifier, Statement, Struct, UnOp, Typed, Type
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path;
use std::sync::atomic::AtomicUsize;
use lazy_static::lazy_static;
use std::sync::Mutex;

// Separate counters for branch and loop labels for break and continue statements
/// Branch label counter for if statements
static BRANCH_LABEL: AtomicUsize = AtomicUsize::new(0);
/// Loop label counter for while, loop, and for statements
static LOOP_LABEL: AtomicUsize = AtomicUsize::new(0);
/// Loop label counter for floating point literals
static FLOAT_LABEL: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    static ref FLOAT_LABEL_MAP: Mutex<HashMap<String, f64>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

// TODO: test all of these one by one (pain)

/// Generates the assembly code for an atom
fn generate_atom(file: &mut File, atom: &Typed<Atom>, context: &mut Context) {
    match atom {
        Typed{expr: Atom::Literal(constant), ..} => {
			if let Literal::Float(f) = constant.expr {
                let label = format!(".float.{}", FLOAT_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst));

                let mut float_label_map = FLOAT_LABEL_MAP.lock().unwrap();
                float_label_map.insert(label.clone(), f);

				writeln!(file, "    movsd xmm0, [{}]	; Load float into xmm0", label).unwrap();
			} else if let Literal::Char(c) = &constant.expr {
				// Turn the character into the corresponding ASCII
				// We need this encoding otherwise weird characters may cause
				// malformed asm to be generated

				let mut display_chars = String::new();
				let mut encoding: i64 = 0;
				for character in c.chars() {
					if !character.is_ascii() { // Continue checking each character
						panic!("Non ASCII character");
					}
					
					// Build the display string for comments
					match character {
						'\n' => display_chars.push_str("\\n"),
						'\r' => display_chars.push_str("\\r"),
						'\t' => display_chars.push_str("\\t"),
						'\0' => display_chars.push_str("\\0"),
						c if c.is_ascii() && c.is_ascii_graphic() => display_chars.push(c),
						_ => display_chars.push('?'),
					};
					
					// Stack each byte into the encoding
					encoding = (encoding << 8) | (character as i64);
				}
				
				writeln!(file, "    mov rax, 0x{:x}	;{}", encoding, display_chars).unwrap();
			}
			
			else {
				// NOTE: change the std::fmt::Display trait for Constant in build_ast.rs in case it doesn't print the asm correctly
				writeln!(file, "    mov rax, {}", constant).unwrap();
			}
        }
        Typed{expr: Atom::Variable(variable), type_} => {
            let var_address = context.var_map.get(variable);

			let mut instruction = "mov";
			let mut register = "rax";
			if type_ == &Type::Float {
				instruction = "movsd";
				register = "xmm0";
			}

            if let Some(var_address) = var_address {
                writeln!(
                    file,
                    "    {} {}, [rbp{}{}]",
					instruction,
					register,
                    if *var_address < 0 { "" } else { "+" },
                    var_address
                )
                .unwrap();
            } else {
				let constant = context
                	.constants
                	.get(variable)
                	.expect(format!("Undeclared variable {}", variable).as_str());
            	writeln!(file, "    {} {}, [.{}]", instruction, register, constant).unwrap();
			}
        }
        Typed{expr: Atom::Expression(expression), ..} => {
            generate_expression(file, expression, context);
        }
        Typed{expr: Atom::FunctionCall(name, args), ..} => {
            writeln!(
                file,
                "	;push function arguments to the stack in reverse order"
            )
            .unwrap();
            for arg in args.iter().rev() {
                // Push arguments in reverse order (C convention)
                generate_expression(file, arg, context);

				if arg.type_ == Type::Float {
					writeln!(file, "    movq rax, xmm0").unwrap();
				}

                writeln!(file, "    push rax").unwrap();
            }
            writeln!(file, "    call {}", name).unwrap();
            writeln!(file, "    add rsp, {}	;pop arguments", args.len() * 8).unwrap();
            // Pop arguments from stack
        }
        Typed{expr: Atom::Array(expressions, _), ..} | Typed{expr: Atom::StructInstance(_, expressions), ..} => {
			// Careful, this section of the code is technical because we want
			// to be able to define arrays of structs such that
			//  [
			// 		S{a, b},
			// 		S{c, d}
			//	]
			// is a valid array of structs. Typically, the expressions in the
			// array would be evaluated and pushed on the stack in reverse order
			// such that the stack for [a, b, c] would look like
			// 	| c |
			// 	| b |
			// 	| a | <- rsp, we get a ptr to this
			//
			// However, when evaluating structs, we push their values on the stack
			// as we evaluate them. This is a big problem, because now, the stack
			// for [S{a, b}, S{c, d}] would actually look like
			// 	| d |
			// 	| c |
			//  | ptr(S{c, d}) |
			// 	| b |
			// 	| a |
			//  | ptr(S{a, b}) | <- rsp, we get a ptr to this
			// But if we try to access arr[1], we would get rsp-8, which is the
			// address of the first attribute of the first struct, not the address
			// of the second struct.

			// To solve this, we need to push the values of the structs first,
			// and then the ptrs to the structs. This way, we get a stack that
			// looks like
			// 	| d |
			// 	| c |
			// 	| b |
			// 	| a |
			//  | ptr(S{c, d}) |
			//  | ptr(S{a, b}) | <- rsp, we get a ptr to this
			// Where array indexing works as expected.

			// Now aditionally, we note that since structs work in the same way,
			// we must correctly handle the cases where some expressions push
			// to the stack, and some don't, all in the same array-like structure.
			// This is what leads to the complexity of this section of the code.

			let mut stack_index = context.stack_index;
			let mut stack_indices = vec![];
			for expr in expressions.iter().rev() {
				// If the expression is a struct or an array, we need to push the
				// values on the stack first
				if matches!(expr, Typed{expr: Expression::Atom(Typed{expr: Atom::StructInstance(_, _) | Atom::Array(_, _), ..}), ..}) {
					generate_expression(file, expr, context);
					stack_indices.push(stack_index - context.stack_index);
					stack_index = context.stack_index;
				}
			}

			let sum = stack_indices.iter().sum::<i64>(); // Size of all the structs values together

			if sum != 0 {
				writeln!(file, "    mov rax, rsp").unwrap();
				writeln!(file, "    add rax, {}", sum).unwrap();

				if !matches!(expressions[0], Typed{expr: Expression::Atom(Typed{expr: Atom::StructInstance(_, _) | Atom::Array(_, _), ..}), ..}){
					writeln!(file, "    push rax").unwrap();
					context.stack_index -= 8;
					context.len += 1;
				}
			}

			let mut iterator = stack_indices.iter();
			for expr in expressions.iter().rev() {
				if matches!(expr, Typed{expr: Expression::Atom(Typed{expr: Atom::StructInstance(_, _) | Atom::Array(_, _), ..}), ..}) {
					let i = iterator.next().unwrap();
					writeln!(file, "    sub rax, {}", i).unwrap();
					writeln!(file, "    push rax").unwrap(); // If generate_expression pushes stuff, this is broken. Same for StructInstance
					context.stack_index -= 8;
					context.len += 1;
				} else {
					// Here we assume that generate_expression doesn't push anything
					// If it does, this is broken
					generate_expression(file, expr, context);
					if sum != 0 {	// If structs and non structs are mixed, we need to save the last rax so that we don't lose count of the structs
						writeln!(file, "    pop rcx").unwrap();
						writeln!(file, "    sub rsp, 8").unwrap();
					}
					if expr.type_ == Type::Float {
						writeln!(file, "    movq rax, xmm0").unwrap();
					}
					writeln!(file, "    push rax").unwrap(); // If generate_expression pushes stuff, this is broken. Same for StructInstance
					if sum != 0 {
						writeln!(file, "    mov rax, rcx").unwrap();
					}
					context.stack_index -= 8;
					context.len += 1;
				}
			}

            // Move the address of the array/struct to rax
            writeln!(
                file,
                "    mov rax, rsp	; Move the address of the array to rax"
            )
            .unwrap();
        }
    }
}

/// Generates the assembly code for an assignment operation
fn generate_assignment(
    op: &AssignmentOp,
    file: &mut File,
    var_address: &i64,
    pointer_dereference: bool,
	type_: &Type,
) {
    let write_address;
    if pointer_dereference {
        write_address = format!("[r8]");
    } else {
        write_address = format!(
            "[rbp{}{}]",
            if *var_address < 0 { "" } else { "+" },
            var_address
        );
    }

	if type_ == &Type::Float {
		match op {
			AssignmentOp::Assign => {
				writeln!(file, "    movq {}, xmm0", write_address).unwrap();
			}
			AssignmentOp::AddAssign => {
				writeln!(file, "    addsd xmm0, {}", write_address).unwrap();
				writeln!(file, "    movq {}, xmm0", write_address).unwrap();
			}
			AssignmentOp::SubtractAssign => {
				writeln!(file, "    movq xmm1, xmm0").unwrap();
				writeln!(file, "    movq xmm0, {}", write_address).unwrap();
				writeln!(file, "    subsd xmm0, xmm1").unwrap();
				writeln!(file, "    movq {}, xmm0", write_address).unwrap();
			}
			AssignmentOp::MultiplyAssign => {
				writeln!(file, "    mulsd xmm0, {}", write_address).unwrap();
				writeln!(file, "    movq {}, xmm0", write_address).unwrap();
			}
			AssignmentOp::DivideAssign => {
				writeln!(file, "    movq xmm1, xmm0").unwrap();
				writeln!(file, "    movq xmm0, {}", write_address).unwrap();
				writeln!(file, "    divsd xmm0, xmm1").unwrap();
				writeln!(file, "    movq {}, xmm0", write_address).unwrap();
			}
			AssignmentOp::ModulusAssign => {
				panic!("Modulus not implemented for floats");
			}
			AssignmentOp::LeftShiftAssign => {
				panic!("Left shift not implemented for floats");
			}
			AssignmentOp::RightShiftAssign => {
				panic!("Right shift not implemented for floats");
			}
			AssignmentOp::BitwiseAndAssign => {
				panic!("Bitwise and not implemented for floats");
			}
			AssignmentOp::BitwiseXorAssign => {
				panic!("Bitwise xor not implemented for floats");
			}
			AssignmentOp::BitwiseOrAssign => {
				panic!("Bitwise or not implemented for floats");
			}
			AssignmentOp::NotAnAssignmentOp => {
				panic!("Not an assignment op");
			}
		}
	} else {
		match op {
			AssignmentOp::Assign => {
				writeln!(file, "    mov {}, rax", write_address).unwrap();
			}
			AssignmentOp::AddAssign => {
				writeln!(file, "    add {}, rax", write_address).unwrap();
			}
			AssignmentOp::SubtractAssign => {
				writeln!(file, "    sub {}, rax", write_address).unwrap();
			}
			AssignmentOp::MultiplyAssign => {
				writeln!(file, "    mov rcx, {}", write_address).unwrap();
				writeln!(file, "    imul rax, rcx").unwrap();
				writeln!(file, "    mov {}, rax", write_address).unwrap();
			}
			AssignmentOp::DivideAssign => {
				writeln!(file, "    mov rcx, rax").unwrap();
				writeln!(file, "    mov rax, {}", write_address).unwrap();
				writeln!(file, "    cqo").unwrap();
				writeln!(file, "    idiv rcx").unwrap();
				writeln!(file, "    mov {}, rax", write_address).unwrap();
			}
			AssignmentOp::ModulusAssign => {
				writeln!(file, "    mov rcx, rax").unwrap();
				writeln!(file, "    mov rax, {}", write_address).unwrap();
				writeln!(file, "    cqo").unwrap();
				writeln!(file, "    idiv rcx").unwrap();
				writeln!(file, "    mov {}, rdx", write_address).unwrap();
			}
			AssignmentOp::LeftShiftAssign => {
				writeln!(file, "    mov rcx, rax").unwrap();
				writeln!(file, "    mov rax, {}", write_address).unwrap();
				writeln!(file, "    shl rax, cl").unwrap();
				writeln!(file, "    mov {}, rax", write_address).unwrap();
			}
			AssignmentOp::RightShiftAssign => {
				writeln!(file, "    mov rcx, rax").unwrap();
				writeln!(file, "    mov rax, {}", write_address).unwrap();
				writeln!(file, "    shr rax, cl").unwrap();
				writeln!(file, "    mov {}, rax", write_address).unwrap();
			}
			AssignmentOp::BitwiseAndAssign => {
				writeln!(file, "    and {}, rax", write_address).unwrap();
			}
			AssignmentOp::BitwiseXorAssign => {
				writeln!(file, "    xor {}, rax", write_address).unwrap();
			}
			AssignmentOp::BitwiseOrAssign => {
				writeln!(file, "    or {}, rax", write_address).unwrap();
			}
			AssignmentOp::NotAnAssignmentOp => {
				panic!("Not an assignment op");
			}
		}
	}
}

/// Generates the assembly code for an expression
fn generate_expression(file: &mut File, expression: &Typed<Expression>, context: &mut Context) {
    match expression {
        Typed{expr: Expression::Atom(atom), ..} => {
            generate_atom(file, atom, context);
        }
        Typed{expr: Expression::UnaryOp(expr, unop), type_} => {
            generate_expression(file, expr, context);
			if type_ == &Type::Float {
				match unop {
					UnOp::UnaryMinus => {
						writeln!(file, "    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)").unwrap();
						writeln!(file, "    psllq xmm1, 63		; xmm1 = 0x8000000000000000 (sign bit mask)").unwrap();
						writeln!(file, "    xorpd xmm0, xmm1	; Flip the sign bit of xmm0").unwrap();
					}
					UnOp::BitwiseNot => {
						writeln!(file, "    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)").unwrap();
						writeln!(file, "    xorpd xmm0, xmm1	; Flip all the bits of xmm0").unwrap();
					}
					UnOp::LogicalNot => {
						writeln!(file, "    pcmpeqd xmm1, xmm1	; xmm1 = all ones (0xFFFFFFFFFFFFFFFF)").unwrap();
						writeln!(file, "    xorpd xmm0, xmm1	; Flip all the bits of xmm0").unwrap();
					}
					UnOp::PreIncrement => {
						writeln!(file, "	movq xmm1, 0x3FF0000000000000  ; Bit pattern of 1.0 in IEEE 754").unwrap();
						writeln!(file, "	addsd xmm0, xmm1").unwrap();
					}
					UnOp::PreDecrement => {
						writeln!(file, "	movq xmm1, 0x3FF0000000000000  ; Bit pattern of 1.0 in IEEE 754").unwrap();
						writeln!(file, "	subsd xmm0, xmm1").unwrap();
					}
					UnOp::UnaryPlus => {
						// Do nothing
					}
					UnOp::Dereference => {
						panic!("Cannot dereference a float");
					}
					UnOp::AddressOf => {
						panic!("Cannot take the address of a float");
					}
					UnOp::NotAUnaryOp => {
						panic!("Not a unary op");
					}
				}
			} else {
				match unop {
					UnOp::UnaryMinus => {
						writeln!(file, "    neg rax").unwrap();
					}
					UnOp::BitwiseNot => {
						writeln!(file, "    not rax").unwrap();
					}
					UnOp::LogicalNot => {
						writeln!(file, "    not rax").unwrap();
					}
					UnOp::PreIncrement => {
						writeln!(file, "    inc rax").unwrap();
					}
					UnOp::PreDecrement => {
						writeln!(file, "    dec rax").unwrap();
					}
					UnOp::UnaryPlus => {
						// Do nothing
					}
					UnOp::Dereference => {
						writeln!(file, "    mov rax, [rax]").unwrap();
					}
					UnOp::AddressOf => {
						// Do nothing
					}
					UnOp::NotAUnaryOp => {
						panic!("Not a unary op");
					}
				}
			}
        }
        Typed{expr: Expression::BinaryOp(left, right, bin_op), type_} => {
			let left_type = left.type_.clone();
            if bin_op != &BinOp::MemberAccess {
				generate_expression(file, right, context);
				if left_type == Type::Float {
					writeln!(file, "    movq rax, xmm0").unwrap();
				}
                writeln!(file, "    push rax").unwrap();
				generate_expression(file, left, context);
            } else {
                if let Expression::Atom(Typed{expr: Atom::Literal(Typed{expr: Literal::Int(i), ..}), ..}) = right.expr {
					generate_expression(file, left, context);
                    writeln!(file, "    mov rcx, {}", i).unwrap();
					if type_ == &Type::Float {
						writeln!(file, "    movq xmm0, [rax + rcx * 8]").unwrap();
					} else {
                    	writeln!(file, "    mov rax, [rax + rcx * 8]").unwrap();
					}
                    return;
                } else {
                    panic!("Unreachable code, something went very wrong");
                }
            }

			if type_ == &Type::Float {
				writeln!(file, "	pop rcx").unwrap();
				writeln!(file, "	movq xmm1, rcx").unwrap();

				match bin_op {
					BinOp::Add => {
						writeln!(file, "	addsd xmm0, xmm1").unwrap();
					}
					BinOp::Subtract => {
						writeln!(file, "	subsd xmm0, xmm1").unwrap();
					}
					BinOp::Multiply => {
						writeln!(file, "	mulsd xmm0, xmm1").unwrap();
					}
					BinOp::Divide => {
						writeln!(file, "	divsd xmm0, xmm1").unwrap();
					}
					BinOp::LessThan => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	setb al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::GreaterThan => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	seta al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::Equal => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	sete al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::NotEqual => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	setne al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::LessOrEqualThan => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	setbe al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::GreaterOrEqualThan => {
						writeln!(file, "	ucomisd xmm0, xmm1").unwrap();
						writeln!(file, "	setae al").unwrap();
						writeln!(file, "	movzx rax, al").unwrap();
					}
					BinOp::BitwiseAnd => {
						writeln!(file, "	andpd xmm0, xmm1").unwrap();
					}
					BinOp::BitwiseXor => {
						writeln!(file, "	xorpd xmm0, xmm1").unwrap();
					}
					BinOp::BitwiseOr => {
						writeln!(file, "	orpd xmm0, xmm1").unwrap();
					}
					
					BinOp::Modulus => {
						panic!("Modulus not implemented for floats");
					}
					BinOp::LogicalAnd => {
						panic!("Logical and not implemented for floats");
					}
					BinOp::LogicalOr => {
						panic!("Logical or not implemented for floats");
					}
					BinOp::LogicalXor => {
						panic!("Logical xor not implemented for floats");
					}
					BinOp::LeftShift => {
						panic!("Left shift not implemented for floats");
					}
					BinOp::RightShift => {
						panic!("Right shift not implemented for floats");
					}
					BinOp::MemberAccess => {
						panic!("Member access not implemented for floats");
					}
					BinOp::NotABinaryOp => {
						panic!("Not a binary op");
					}
				}
			} else {
				writeln!(file, "    pop rcx").unwrap();
				match bin_op {
					BinOp::Add => {
						writeln!(file, "    add rax, rcx").unwrap();
					}
					BinOp::Subtract => {
						writeln!(file, "    sub rax, rcx").unwrap();
					}
					BinOp::Multiply => {
						writeln!(file, "    imul rax, rcx").unwrap();
					}
					BinOp::Divide => {
						writeln!(file, "    cqo").unwrap();
						writeln!(file, "    idiv rcx").unwrap();
					}
					BinOp::Modulus => {
						writeln!(file, "    cqo").unwrap();
						writeln!(file, "    idiv rcx").unwrap();
						writeln!(file, "    mov rax, rdx").unwrap();
					}
					BinOp::LessThan => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    setl al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::GreaterThan => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    setg al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::Equal => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    sete al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::NotEqual => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    setne al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::LessOrEqualThan => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    setle al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::GreaterOrEqualThan => {
						writeln!(file, "    cmp rax, rcx").unwrap();
						writeln!(file, "    setge al").unwrap();
						writeln!(file, "    movzx rax, al").unwrap();
					}
					BinOp::BitwiseAnd => {
						writeln!(file, "    and rax, rcx").unwrap();
					}
					BinOp::BitwiseXor => {
						writeln!(file, "    xor rax, rcx").unwrap();
					}
					BinOp::BitwiseOr => {
						writeln!(file, "    or rax, rcx").unwrap();
					}
					BinOp::LogicalAnd => {
						writeln!(file, "    and rax, rcx").unwrap();
					}
					BinOp::LogicalOr => {
						writeln!(file, "    or rax, rcx").unwrap();
					}
					BinOp::LogicalXor => {
						writeln!(file, "    xor rax, rcx").unwrap();
					}
					BinOp::LeftShift => {
						writeln!(file, "    shl rax, cl").unwrap();
					}
					BinOp::RightShift => {
						writeln!(file, "    shr rax, cl").unwrap();
					}
					BinOp::MemberAccess => {
						panic!("Unreachable code, member access is turned into array access much earlier");
					}
					BinOp::NotABinaryOp => {
						panic!("Not a binary op");
					}
				}
			}
        }
        Typed{expr: Expression::Assignment(lvalue, expr, op), type_} => {
            generate_expression(file, expr, context);

            match lvalue {
                Typed{expr: ReassignmentIdentifier::Variable(s), ..} => {
                    let var_address = context.var_map.get(s);
                    if let Some(var_address) = var_address {
                        generate_assignment(op, file, var_address, false, type_);
                        return;
                    }

                    if let Some(_) = context.constants.get(s) {
                        panic!("Cannot assign to a constant variable");
                    }
                    panic!("Undeclared variable {:?}", s);
                }
                Typed{expr: ReassignmentIdentifier::Array(variable, indices), ..} => {
					if type_ == &Type::Float {
						writeln!(file, "    movq rax, xmm0").unwrap();
					}
                    writeln!(file, "    push rax").unwrap();

                    generate_expression(file, variable, context);
                    writeln!(file, "    push rax").unwrap();
                    generate_expression(file, &indices[0], context);
                    writeln!(file, "    mov r8, rax").unwrap();
                    writeln!(file, "    imul r8, 8").unwrap();
                    writeln!(file, "    pop rax").unwrap();
                    writeln!(file, "    add r8, rax").unwrap();

                    writeln!(file, "    pop rax").unwrap();
					if type_ == &Type::Float {
						writeln!(file, "    movq xmm0, rax").unwrap();
					}
                    generate_assignment(op, file, &0, true, type_);
                }
                Typed{expr: ReassignmentIdentifier::MemberAccess(_, _), ..} => {
                    panic!(
                        "Unreachable code, member access is turned into array access much earlier"
                    );
                }
                Typed{expr: ReassignmentIdentifier::Dereference(v), ..} => {
					if type_ == &Type::Float {
						writeln!(file, "    movq rax, xmm0").unwrap();
					}
					writeln!(file, "    push rax").unwrap();
                    
                    generate_expression(file, v, context);
                    writeln!(file, "    mov r8, rax").unwrap();

                    writeln!(file, "    pop rax").unwrap();
					if type_ == &Type::Float {
						writeln!(file, "    movq xmm0, rax").unwrap();
					}

                    generate_assignment(op, file, &0, true, type_);
                }
            }
        }
        Typed{expr: Expression::TypeCast(expr, _), type_: cast_type} => {
			let expr_type = expr.type_.clone();
            generate_expression(file, expr, context);

			if expr_type == Type::Float && cast_type != &Type::Float {
				writeln!(file, "    movq rax, xmm0").unwrap();
			} else if expr_type != Type::Float && cast_type == &Type::Float {
				writeln!(file, "    movq xmm0, rax").unwrap();
			}

			// TODO: casting literally interprets the bits of the float as an int.
			// I should have a different method that actually converts the float to an int
			// and vice versa. Maybe in std?
        }
        Typed{expr: Expression::ArrayAccess(variable, indices), type_} => {
            generate_expression(file, variable, context);
            let index = indices.get(0).expect("Array access without index");

            writeln!(file, "    push rax").unwrap();
            generate_expression(file, index, context);
            writeln!(file, "    pop rcx").unwrap();

			if type_ == &Type::Float {
				writeln!(file, "    movq xmm0, [rcx + rax * 8]").unwrap();
			} else {
            	writeln!(file, "    mov rax, [rcx + rax * 8]").unwrap();
			}
        }
    }
}

/// Context struct to keep track of local variables and stack index
struct Context {
    /// HashMap to store the variables and their addresses
    var_map: HashMap<String, i64>,
    /// HashMap to store the var types
    var_types: HashMap<String, String>,
    /// Set of constant variables
    constants: HashSet<String>,
    /// Structs
    structs: HashMap<String, HashMap<String, i64>>,
    /// Stack index to keep track of the stack pointer
    stack_index: i64,
    /// HashSet to store the variables that were declared in the current block
    local_variables: HashSet<String>,
    /// Label a continue statement should jump to
    continue_label: Option<String>,
    /// Label a break statement should jump to
    break_label: Option<String>,
    /// Length of the context (in number of 64 bit words)
    len: usize,
    /// Arrays number of dimensions
    dimensions: HashMap<String, usize>,
}

impl Context {
    fn new() -> Context {
        Context {
            var_map: HashMap::new(),
            var_types: HashMap::new(),
            constants: HashSet::new(),
            structs: HashMap::new(),
            stack_index: 0,
            local_variables: HashSet::new(),
            continue_label: None,
            break_label: None,
            len: 0,
            dimensions: HashMap::new(),
        }
    }

    /// Create a new context from the last context
    ///
    /// This clones all the info from the last context except the local variables
    fn from_last_context(context: &Context) -> Context {
        Context {
            var_map: context.var_map.clone(),
            var_types: context.var_types.clone(),
            constants: context.constants.clone(),
            structs: context.structs.clone(),
            stack_index: context.stack_index.clone(),
            local_variables: HashSet::new(),
            continue_label: context.continue_label.clone(),
            break_label: context.break_label.clone(),
            len: 0,
            dimensions: context.dimensions.clone(),
        }
    }

    /// Insert a variable into the context
    fn insert(&mut self, key: String, value: i64, var_type: String) {
        self.var_map.insert(key.clone(), value);
        self.var_types.insert(key.clone(), var_type);
        self.local_variables.insert(key);
        self.len += 1;
    }
}

/// Generates the assembly code for a compound statement
fn generate_compound_statement(file: &mut File, cmp_statement: &Typed<Statement>, last_context: &Context) {
    let mut context = Context::from_last_context(last_context);

    // Yes I do need to generate compound statements like that otherwise
    // local variables can't be a thing

    if let Typed{expr: Statement::Compound(statements), ..} = cmp_statement {
        for statement in statements.iter() {
            match statement {
                Typed{expr: Statement::Return(expression), ..} => {
                    generate_expression(file, expression, &mut context);
                    // Pop all the variables from the stack
                    writeln!(
                        file,
                        "    add rsp, {}		;pop local variables before return",
                        context.len * 8
                    )
                    .unwrap();
                    writeln!(file, "    pop rbx		;restore rbx for caller function").unwrap();
                    writeln!(file, "    pop rbp		;restore base pointer").unwrap();
                    writeln!(file, "    ret").unwrap();
                }
                Typed{expr: Statement::Expression(expression), ..} => {
                    generate_expression(file, expression, &mut context);
                }
                Typed{expr: Statement::Let(variable, expr, _), type_: var_type} => {
                    if let Some(expr) = expr {
                        generate_expression(file, expr, &mut context);
                    }

                    // Dereference the variable if it is a pointer
                    let mut number_of_dereferences = 0;
                    let mut variable = variable;
					let mut type_ = var_type;
                    while let AssignmentIdentifier::Dereference(new_expr) = variable {
                        variable = &new_expr.expr;
                        number_of_dereferences += 1;
						type_ = &new_expr.type_;
                    }

                    if let Some(_) = expr {
						if Type::Float == *type_ {
							writeln!(file, "    movq rax, xmm0").unwrap();
						}
                        writeln!(file, "    push rax").unwrap();
                    } else {
                        writeln!(file, "    push 0").unwrap(); // Undefined variables default to 0
                    }
                    context.stack_index -= 8;

                    while number_of_dereferences > 0 {
                        writeln!(file, "    push rsp").unwrap();
                        number_of_dereferences -= 1;
                        context.stack_index -= 8;
                        context.len += 1;
                    }

                    if let AssignmentIdentifier::Variable(variable) = variable {
                        context.insert(variable.clone(), context.stack_index, var_type.to_string());
                    } else if let AssignmentIdentifier::Array(variable, dimensions) = variable {
                        context.insert(variable.clone(), context.stack_index, var_type.to_string());

                        let mut dimensions = dimensions.clone();
                        context
                            .dimensions
                            .insert(variable.clone(), dimensions.len());
                        // Push the dimensions on the stack
                        while let Some(expr) = dimensions.pop() {
                            generate_expression(file, &expr, &mut context);
                            writeln!(file, "    push rax	;pushing array dimensions onto stack")
                                .unwrap();
                            context.stack_index -= 8;
                            context.insert(
                                format!("{}:dim{}", variable, dimensions.len()),
                                context.stack_index,
                                "int".to_string(),
                            );
                        }
                    } else {
                        panic!("Expected a variable or an array, got {:?}", variable);
                    }
                }
                Typed{expr: Statement::Compound(_), ..} => {
                    generate_compound_statement(file, statement, &context);
                }
                Typed{expr: Statement::If(condition, if_statement, else_statement), ..} => {
                    let branch_label =
                        BRANCH_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let else_label = format!("else_{}", branch_label);
                    let end_label = format!("end_{}", branch_label);

                    writeln!(file, "    ;if statement").unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", else_label).unwrap();
                    generate_compound_statement(file, if_statement, &context);
                    writeln!(file, "    jmp {}", end_label).unwrap();
                    writeln!(file, "{}:", else_label).unwrap();
                    if let Some(else_statement) = else_statement {
                        generate_compound_statement(file, else_statement, &context);
                    }
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Typed{expr: Statement::While(condition, statement), ..} => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("while_start_{}", loop_label);
                    let end_label = format!("while_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;while statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", end_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Typed{expr: Statement::Loop(statement), ..} => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("loop_start_{}", loop_label);
                    let end_label = format!("loop_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;loop statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Typed{expr: Statement::Dowhile(condition, statement), ..} => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("dowhile_start_{}", loop_label);
                    let end_label = format!("dowhile_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;do while statement").unwrap();
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    jne {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Typed{expr: Statement::For(init, condition, update, statement), ..} => {
                    let loop_label = LOOP_LABEL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let start_label = format!("for_start_{}", loop_label);
                    let end_label = format!("for_end_{}", loop_label);

                    context.continue_label = Some(start_label.clone());
                    context.break_label = Some(end_label.clone());

                    writeln!(file, "    ;for statement").unwrap();
                    generate_expression(file, init, &mut context);
                    writeln!(file, "{}:", start_label).unwrap();
                    generate_expression(file, condition, &mut context);
                    writeln!(file, "    cmp rax, 0").unwrap();
                    writeln!(file, "    je {}", end_label).unwrap();
                    generate_compound_statement(file, statement, &context);
                    generate_expression(file, update, &mut context);
                    writeln!(file, "    jmp {}", start_label).unwrap();
                    writeln!(file, "{}:", end_label).unwrap();
                }
                Typed{expr: Statement::Break, ..} => {
                    if let Some(label) = &context.break_label {
                        writeln!(file, "    jmp {}	;break statement", label).unwrap();
                    } else {
                        panic!("Break statement outside of loop");
                    }
                }
                Typed{expr: Statement::Continue, ..} => {
                    if let Some(label) = &context.continue_label {
                        writeln!(file, "    jmp {}	;continue statement", label).unwrap();
                    } else {
                        panic!("Continue statement outside of loop");
                    }
                }
				Typed{expr: Statement::Asm(asm, _), ..} => {
					writeln!(file, "{}", asm).unwrap();
				}
            }
        }
    } else {
        panic!("Expected a compound statement, got {:?}", cmp_statement);
    }

    // Pop all the variables from the stack
    writeln!(
        file,
        "    add rsp, {}		;end of block, pop local variables",
        context.local_variables.len() * 8
    )
    .unwrap();
}

/// Generates the assembly code for a function
fn generate_function(file: &mut File, function: &Typed<Function>, context: &Context) {
    let mut context = Context::from_last_context(context);

    let Typed{expr: Function::Function(name, params, statement, _), ..} = function;

    context.stack_index = 24; // Skip rbp, rbx and the return address
    for param in params.iter() {
        context.insert(param.0.clone(), context.stack_index, param.1.to_string());
        context.stack_index += 8;
    }
    context.stack_index = 0;

    writeln!(file, "").unwrap();
    writeln!(file, "global {}", name).unwrap();
    writeln!(file, "{}:", name).unwrap();
    writeln!(file, "    push rbp		;save previous base pointer").unwrap();
    writeln!(file, "    push rbx		;functions should preserve rbx").unwrap();
    writeln!(file, "    mov rbp, rsp	;set base pointer").unwrap();
    generate_compound_statement(file, statement, &context);
    writeln!(file, "    pop rbx			;restore rbx for caller function").unwrap();
    writeln!(file, "    pop rbp			;restore base pointer").unwrap();
    writeln!(
        file,
        "    ret				;return by default if no return statement was reached"
    )
    .unwrap();
}

fn generate_constants(file: &mut File, const_vector: &Vec<Typed<Constant>>) {
	for constant in const_vector.iter(){
		let Typed{expr: Constant::Constant(name, constant, _), ..} = constant;
    	writeln!(file, "    .{}: dq {}", name, constant).unwrap();
	}
}

fn generate_float_literals(file: &mut File) {
	let float_label_map = FLOAT_LABEL_MAP.lock().unwrap();
	for (label, value) in float_label_map.iter() {
		// Important to use debug trait, otherwise floats with no decimals will be interpreted as int
		writeln!(file, "	{}: dq {:?}", label, value).unwrap();
	}
}

/// Generates the assembly code for the given AST
pub fn generate(ast: &Ast, out_path: &str) {
    let path = path::Path::new(out_path);
    let mut file = File::create(path).unwrap();

    // Post-order traversal of the AST to generate x86_64 (+nasm pseudo-instructions)

    let Program::Program(function_vector, const_vector, structs, _enums) = &ast.program;
    
	writeln!(file, "[BITS 64]").unwrap();
    writeln!(file, "section .text").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "global _start").unwrap();
    writeln!(file, "_start:").unwrap(); // Entry point
    writeln!(file, "    call main").unwrap(); // Call main
    writeln!(file, "    mov rdi, rax").unwrap(); // Return value of main
    writeln!(file, "    mov rax, 60").unwrap(); // Exit syscall
    writeln!(file, "    syscall").unwrap();

    let mut context = Context::new();

    for constant in const_vector.iter() {
        let Typed{expr: Constant::Constant(name, _, _), ..} = constant;
        if let Some(_) = context.var_map.get(name) {
            panic!(
                "Variable {} already declared as a constant. Constants cannot be declared twice",
                name
            );
        }
        context.constants.insert(name.clone());
    }

    for struct_ in structs.iter() {
        let Struct { id, members } = struct_;
        let mut member_names = HashMap::new();
        for (i, member) in members.iter().enumerate() {
            let (name, _) = member;
            member_names.insert(name.clone(), i as i64);
        }
        context.structs.insert(id.clone(), member_names);
    }

    for function in function_vector.iter() {
        generate_function(&mut file, function, &context);
    }

    writeln!(file, "").unwrap();
    writeln!(file, "section .data").unwrap();

	generate_constants(&mut file, const_vector);
	generate_float_literals(&mut file);
}
