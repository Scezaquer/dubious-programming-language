# Dubious-Programming-Language

Partially based on https://norasandler.com/2017/11/29/Write-a-Compiler.html.

A simple compiler for the Dubious programming language (DPL).

### Priority features

- TODO: Helpful compiler error messages at the code generation stage (make &lt;T>TokenWithDebugInfo generic?)
- TODO: Structs, enums, unions
- TODO: better checker error messages
- TODO: heap memory stuff
- TODO: std library
- TODO: inline asm
- TODO: vscode syntax highlighting
- TODO: floating point arithmetic
- TODO: Wiki
- TODO: support escape characters
- TODO: namespaces for functions, constants, structs, enums, unions

### Optional

- TODO: register allocation ?
- TODO: default function parameters ?
- TODO: kwargs ?
- TODO: Give the option to generate LLVM IR instead of x86_64
- TODO: improve #include

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
3. **Multiplicative operators**: `a * b`, `a / b`, `a % b`
4. **Additive operators**: `a + b`, `a - b`
5. **Bitwise shift operators**: `a << b`, `a >> b`
6. **Relational operators**: `a < b`, `a > b`, `a <= b`, `a >= b`
7. **Equality operators**: `a == b`, `a != b`
8. **Bitwise AND**: `a & b`
9. **Bitwise XOR**: `a ^ b`
10. **Bitwise OR**: `a | b`
11. **Logical AND**: `a && b`V
12. **Logical XOR**: `a ^^ b`
13. **Logical OR**: `a || b`
14. **Type cast**: `a : b`
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

Constants may be defined anywhere in the global scope and are valid everywhere in the project, even
before they are declared or in different files. Constants may only be declared once, and must be initialized at the time
they are declared. They are essentially just aliases for literals, so expressions cannot be assigned to them,
only literals. If I implement expression pre-processing then maybe it'll become possible to have
combinations of literals instead. Right now #define is more versatile.

true/false booleans evaluate to 1 and 0 respectively.

The compiler is pretty wasteful in terms of space, making everything 64 bits alined even when that's excessive,
but on modern hardware that should not matter at all, and it makes my life easier.
Maybe i'll need some byte-level fine grain control on memory at some point tho?
I can work around it by combining some binary operations to get that but it's a little
impractical (and requires more cpu cycles of course, but at this point, does it even matter). I'll see.

TODO: N dimensional arrays (on stack):

```
let arr[3]: list[int] = [0, 1, 2];
let arr2d[3, 4]: list[int] = [[0, 1, 2, 3], [0, 1, 2, 3], [0, 1, 2, 3]];
let arr2d[3, 4]: list[int] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3]; // TODO: Do I make this legal? Is it equivalent to the array above? Probably shouldn't because it opens a whole can of worms with mismatched dimensions, like assigning a [4, 3] array to a [3, 4] array pointer which could lead to some sneaky bugs. Probably best to enfore strict dim match.

let arr2d[2, 2]: list[int] = [[0], [1, 2]];	// Arrays are cast to the smallest rectangular array that can contain them. Undefined entries default to 0, so this is equivalent to [[0, 0], [1, 2]]
arr2d[3]; // returns 1: Treat arrays as flat when indexed like this
arr2d[1, 0]; // returns 1 as well

arr2d[3, 4] = 3; // TODO: Special syntax that initializes all entries to 3
```

The compiler should treat all arrays as flat and substitute dimensional indexing by it's equivalent flat indexing,
so that the dimensions of the array don't have to be saved at runtime, only a value containing the size (in words).

In memory, there should be a variable on the stack containing the length of the array, and additional variables with the array's dimensions if needed. Then elements of the array one after the other in a continuous block of memory on the stack.

At runtime, check before accessing an array's element that it's within the array's length,
otherwise throw some error (need an error routine? Otherwise just do something that segfaults
but that's pretty dirty). It's technically an extra instruction (and a branch so extra bad)
but useful safety to have, should save me from a bunch of bugs. NOTE: Branch prediction: Modern
CPUs generally predict forward conditional jumps as "unlikely" and backward conditional jumps as
"likely," which is a natural heuristic based on typical loop behavior.

make sure n >= 1 in `let arr[n] = ...` ?

only allow array literals in let statements? meaning considering them as separate from expressions
If I don't but still implement the "uninitialized entries default to 0" thing then
the same expression could mean 2 different things in 2 different contexts. Unless
I enforce all arrays being rectangular? I guess I could make a more flexible data
structure later for cases where not all entries in a list are arrays of the same size.
Alternatively arrays of pointers would be fine.

should arr2d[2, 2] be list[int] or list[list[int]]? I think list[int] since
it's just syntactic sugar for that. Technically arr2d is a pointer to the
beginning of the list, while list[list[int]] would be a pointer to a list which
itself contains pointers to int lists. Useful to keep distinct.

I explicitely don't allow C nonsense like arr[3] being equivalent to 3[arr].
Array indexation must be of the form `identifier[exp1, exp2, ...]`

TODO: test array implementation thoroughly. 
- Test 0 autofill
- Test up to high dim, both indexing ways
- Test/implement assignment to array
- Test using expressions as array elements, array indices, array dimensions
- Assigning an array of the wrong dim should crash

TODO: element-wise operations on arrays?
TODO: array literals are glitched as fuck if you do weird dimension things. So
don't. Ideally stick to either rectangular or 1/2d arrays. If you do something
else, you're on your own, and expect fucked up indexing behavior.
TODO: make arr.len a variable accessible in code

Type `str` is an alias of type `array[char]`. Type `bool` is an alias of type `int`

The `char` type is different from usual. Since all data types are 64 bits, it
would be very wasteful to have individual characters take 64 bits, especially
when manipulating long strings or text files. So `char` can actually contain
up to 8 characters. This means that `'a'` is a char, but `'abcdefgh'` is also a char.
However, `'abcdefghi'` is not.

In memory, characters in the char are stored backwards, such that `'abcd'`
corresponds to hex `0x64636261`, which naïvely translates to `dcba`. This is so
that chars with less than 8 letters behave as expected (meaning, `'a'` corresponds
to hex `0x61` instead of `0x6100000000000000`)

This also means that `str` are actually chunked into groups of 8 characters,
which is important to keep in mind when indexing them. I.e., `let a: str = "abcdefghij"`
will give you `a[0] == 'abcdefgh'` and `a[1] == 'ij'`. Finer access is obtained through
casting to int and bitwise manipulation.

Physically in memory, structs are just like arrays where each entry can have
a different type. When instantiating a struct, we get a pointer to the first
attribute. This means that array-style indexing should work for structs (should it?
currently it does but breaks typechecking. TODO: Fix)

```
struct S {
	first_attribute: int;
	second_attribute: char;
}

fn main(): int {
	let a: S = S{ 1, 'a' };
	return a.first_attribute; // This should be the same as a[0]. TODO: a[1] return 97 but should actually complain because char != int
}
```

TODO: should I support this syntax for instantiation ? :
```
let a: S = S{ 
	first_attribute: 1,
	second_attribute: 'a'
	};
```

- TODO: mixing member access and array access should be broken (something.attribute[0] doesn't work)
- TODO: Can't change struct members after initialization
- TODO: make tests for structs


expr.identifier is an expression
expr[expr, expr, ...] is valid array access syntax