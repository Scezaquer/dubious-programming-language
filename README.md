# Dubious-Programming-Language

Based on https://norasandler.com/2017/11/29/Write-a-Compiler.html

- TODO: Finish all basic operators
- TODO: %include
- TODO: %define
- TODO: Function calls
- TODO: String literals
- TODO: Structs and enums?
- TODO: Checker
- TODO: binary and hex literals
- TODO: make generator write comments in asm file
- TODO: register allocation
- TODO: testing assignment operators
- TODO: default function parameters ?
- TODO: kwargs ?


A simple compiler for the Dubious programming language (DPL).

# Usage

To use this compiler, you need to provide an input file (typically with a `.dpl` extension) and specify an output file. 
The compiler can also print the Abstract Syntax Tree (AST) and tokens for debugging purposes, and it can output either 
an x86_64 assembly file or an elf64 binary.

## Command Line Arguments

- `input_file`: The input file to read (required).
- `--output-file`, `-o`: The output file to write (default is `out`).
- `--ast`, `-a`: Print the AST (default is `false`).
- `--tokens`, `-t`: Print the tokens (default is `false`).
- `--output-asm`, `-S`: Output an assembly file instead of a binary (default is `false`).

## Example

```sh
# Compile a DPL file to a binary
./dubious input.dpl -o output

# Compile a DPL file to an assembly file
./dubious input.dpl -S -o output

# Print the tokens and AST
./dubious input.dpl --tokens --ast
```

## Error Handling

- If the input file does not exist, the compiler will print an error message and exit.
- If there are issues reading the input file, the compiler will panic with a message.


## Operator Precedence Table

1. **Member access**: `.`
2. **Unary operators**: `++a`, `--a`, `+a`, `-a`, `!a`, `~a`, `*a`, `&a`
3. **Exponentiation**: `a ** b`
4. **Multiplicative operators**: `a * b`, `a / b`, `a % b`
5. **Additive operators**: `a + b`, `a - b`
6. **Bitwise shift operators**: `a << b`, `a >> b`
7. **Relational operators**: `a < b`, `a > b`, `a <= b`, `a >= b`
8. **Equality operators**: `a == b`, `a != b`
9. **Bitwise AND**: `a & b`
10. **Bitwise XOR**: `a ^ b`
11. **Bitwise OR**: `a | b`
12. **Logical AND**: `a && b`V
13. **Logical XOR**: `a ^^ b`
14. **Logical OR**: `a || b`
15. **Assignment operators**: `a = b`, `a += b`, `a -= b`, `a *= b`, `a /= b`, `a %= b`, `a <<= b`, `a >>= b`, `a &= b`, `a ^= b`, `a |= b`

Note: Assignment operators have a return value equal to the expression being assigned.

If the expression in if statements evaluates to anything other than 0, then the if
statement executes. Otherwise else (if present).

Uninitialized variables default to 0.

++ and -- are NOT assignment operators (i.e. ++a will evaluate to a+1 but the value of a will be unchanged).

For loop iterator variables can't be declared inside the loop itself they have to be declared before. i.e.
```
for (let i: int = 0; i < 10; i += 1){
	...
}
```
isn't legal syntax and should be replaced by
```
let i: int;
for (i = 0; i < 10; i += 1){
	...
}
```
Furthermore all 3 fields have to contain an expression, although the expression may have no side effects, i.e
```
for (;;){
	...
}
```
Isn't legal but
```
for (0;1;0){
	...
}
```
Is, and is equivalent to `for (;;)` in C

Undefined variables default to 0.

Constants may be defined anywhere in the global scope and are valid everywhere in the file, even
before they are declared. Constants may only be declared once, and must be initialized at the time
they are declared. They are essentially just aliases for literals, so expressions cannot be assigned to them,
only literals. If I implement expression pre-processing then maybe it'll become possible to have
conbinations of literals instead.