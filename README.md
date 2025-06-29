# Dubious-Programming-Language

Partially based on https://norasandler.com/2017/11/29/Write-a-Compiler.html.

A simple compiler for the Dubious programming language (DPL).

### Priority features

- TODO: fix problem where structs in function prototypes don't get the path added during typechecking
- TODO: std library
- TODO: Wiki
- TODO: let strings be defined over multiple lines like "hello "\n"world" in code would evaluate to the literal "hello world"
- TODO: circular imports when the two files are in different namespaces cause a compilation crash (infinite import loop)

### Optional

- TODO: optimize multiple files importing the same methods (only generate code for one of them instead of having redundant asm)
- TODO: optimized register allocation ?
- TODO: default function parameters ?
- TODO: kwargs ?
- TODO: Give the option to generate LLVM IR instead of x86_64
- TODO: exclude functions that are never called from code generation ?
- TODO: ellipses in function args ? you could always just pass an array or a struct
- TODO: Void pointers?  Im not entirely sure I need it as I can already freely cast anything to anything but that would make for more explicit code. This may be an alternative/complementary to generics, but I feel like it would be worse

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
- If the code to compiled is incorrect, The compiler will panic and print a message pointing out the line and the nature of the error.


## Operator Precedence Table

1. **Member access**: `.`, `::`
2. **Unary operators**: `++a`, `--a`, `+a`, `-a`, `!a`, `~a`, `*a`, `&a`
3. **Multiplicative operators**: `a * b`, `a / b`, `a % b`
4. **Additive operators**: `a + b`, `a - b`
5. **Bitwise shift operators**: `a << b`, `a >> b`
6. **Relational operators**: `a < b`, `a > b`, `a <= b`, `a >= b`
7. **Equality operators**: `a == b`, `a != b`
8. **Bitwise AND**: `a & b`
9. **Bitwise XOR**: `a ^ b`
10. **Bitwise OR**: `a | b`
11. **Logical AND**: `a && b`
12. **Logical XOR**: `a ^^ b`
13. **Logical OR**: `a || b`
14. **Type cast**: `a : b`
15. **Assignment operators**: `a = b`, `a += b`, `a -= b`, `a *= b`, `a /= b`, `a %= b`, `a <<= b`, `a >>= b`, `a &= b`, `a ^= b`, `a |= b`

Note: Assignment operators have a return value equal to the expression being assigned.

If the expression in if statements evaluates to anything other than 0, then the if
statement executes. Otherwise else (if present).

Uninitialized variables default to 0.

++ and -- are NOT assignment operators (i.e. `++a` will evaluate to `a+1` but the value of `a` will be unchanged).

For loop iterator variables can be declared inside the loop itself, i.e. the following is legal syntax.
```
for (let i: int = 0; i < 10; i += 1){
	...
}
```
Strictly speaking it is equivalent to
```
let i: int = 0;
for (0; i < 10; i += 1){
	...
}
```
Therefore `i` remains accessible after the loop in both cases.

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

true/false booleans evaluate to 0xFFFFFFFFFFFFFFFF and 0 respectively.

The compiler is pretty wasteful in terms of space, making everything 64 bits alined even when that's excessive,
but on modern hardware that should not matter at all, and it makes my life easier.
Maybe i'll need some byte-level fine grain control on memory at some point tho?
I can work around it by combining some binary operations to get that but it's a little
impractical (and requires more cpu cycles of course, but at this point, does it even matter). I'll see.

## Arrays

TODO: N dimensional arrays (on stack):

```
let arr[3]: list[int] = [0, 1, 2];
let arr2d[3, 4]: list[int] = [[0, 1, 2, 3], [0, 1, 2, 3], [0, 1, 2, 3]];
let arr2d[3, 4]: list[int] = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3];

let arr2d[2, 2]: list[int] = [[0], [1, 2]];	// Arrays are cast to the smallest rectangular array that can contain them. Undefined entries default to 0, so this is equivalent to [[0, 0], [1, 2]]
arr2d[3]; // returns 1: Treat arrays as flat when indexed like this
arr2d[1, 0]; // returns 1 as well

arr2d.len; // The length of arr2d (when flattened). Here, it's 4.
```

The compiler should treat all arrays as flat and substitute dimensional indexing by it's equivalent flat indexing,
so that the dimensions of the array don't have to be saved at runtime, only a value containing the size (in words).

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

Array indexation must be of the form `expr[exp1, exp2, ...]`. If `expr` is
anything other than a variable (like the return value of a function, or a struct
member), it may only be indexed through flat indexing. To use multidimensional
indexing, create a variable, give it the array, and index that instead. This
is because all arrays are internally flat, so the compiler needs the programmer
to explicitely say what he wants the dimensions to be in a let statement.

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

The length of an array is stored in the 64 bits right before the start of the array.
This means that `arr[-1]` is the length of the array. 


## Strings

Type `str` is an alias of type `array[char]`.

The `char` type is different from usual. Since all data types are 64 bits, it
would be very wasteful to have individual characters take 64 bits, especially
when manipulating long strings or text files. So `char` can actually contain
up to 8 characters. This means that `'a'` is a char, but `'abcdefgh'` is also a char.
However, `'abcdefghi'` is not.

In memory, characters in the char are in reverse order, padded to the left, such that
`'abcd'` corresponds to hex `0x64636261`. Char `'a'` corresponds to hex `0x61`
(instead of `0x6100000000000000`). This is the order that lets the print syscall
works properly on strings.

This also means that `str` are actually chunked into groups of 8 characters,
which is important to keep in mind when indexing them. I.e., `let a: str = "abcdefghij"`
will give you `a[0] == 'abcdefgh'` and `a[1] == 'ij'`. Finer access is obtained through
casting to int and bitwise manipulation. The standard library should eventually
implement nicer ways to manipulate strings.

The following escape characters are supported: `\n \r \t \\ \' \" \0`

## Structs

Physically in memory, structs are just like arrays where each entry can have
a different type. When instantiating a struct, we get a pointer to the first
attribute.

```
struct S {
	first_attribute: int;
	second_attribute: char;
}

fn main(): int {
	let a: S = S{ 1, 'a' };
	return a.first_attribute;
}
```

TODO: should I support this syntax for instantiation ? :
```
let a: S = S{ 
	first_attribute: 1,
	second_attribute: 'a'
	};
```

Reassignments can only have a variable, a dereferenced address, an array element
or a struct member as left hand side.

Similarly to functions, structs and enums are both defined everywhere, even before
they are declared. This allows circular definitions and recursive structures.

The 'len' attribute is reserved, and s.len refers to the number of attributes in
the struct. In memory, the length of a struct is stored in the 64 bits that precede
the first element of the struct, similarly to how array length is stored.

## Enums

Enums are discrete types whose value can be one in a list of user-defiend values.

```
enum E { // Define the enum and give it 3 possible values
	LOW,	// internally, this is 0
	MEDIUM,	// 1
	HIGH	// 2
}

fn main(): int{
	let var : E = E.LOW;
	var = E.MEDIUM;
	return var : int;	// returns E.MEDIUM casted as an int, so 1.
}
```


## Inline asm

You can inline asm with the `asm [string literal]` statement. It will simply take the string literal and paste it in the asm at the corresponding spot. This is a compile-time operation, so the string literal has to be fully defined at compile time, meaning a variable containing a string doesn't work, and neither do hypothetical string formatting operations.

```
fn main(): void {
	asm "
	mov rdi, 3 ; hello :) this is inline asm
    mov rax, 60
    syscall";
}
```

Asm statements can be typed. For example

```
fn ftoint(x : float) : int {
	x;	// Move x into xmm0
	asm "	cvtsd2si rax, xmm0  ; Convert double in xmm0 to 64-bit integer" : int; // Cast asm to int
}
```

In the example above, the asm is cast to int, which allows us to pass the typechecker by
promising what the asm does just resolves to int, which in turn means the body of the
function matches it's definition. I'd suggest you get REAL familiar with how the
compiler works before using that feature, but in short: If the value your asm returns
is a float, then the compiler will assume it is located in xmm0. If it is anything
else; it will assume it is in rax. So write your asm such that the right stuff ends
up in the right place.

## Floating point arithmetic

Floating point arithmetic is performed as follows

```
#include<cast.dpl>

fn main() : int {
	let a : float = 1.5;
	let b : float = 0.5;
	let c : float = a + b;
	let i : int = 2;
	c += inttof(i);
	return ftoint(c);
}
```

We note that in order to perform operations involving both ints and floats,
we first need some type conversion, as mixed type operations with implicit type
conversion such as `1.5 + 2` are not supported.

IMPORTANT: Casting a float to an int (or conversely) will NOT give the result
you might expect. The typecasting operator interprets the bits contained in the
memory location of the variable as being bits that represent the type you cast
the variable to. What this means is that if you cast `1.0` to int, you will NOT get `1`.
Instead, we have 1.0 being represented in binary according to IEEE 754 as
`0x3FF0000000000000`, which will simply be read as if it was an int, which means
you'll get `4607182418800017408` in decimal. Similarly, casting the integer `1`
to float will read `0x1` as if it was the IEEE 754 representation of a float,
which is about `5e-324`.

In order to convert from int to float, or float to int, you should use the
`inttof(x: float)` and `ftoint(x: float)` functions in cast.dpl. These will actually
give the correct number, unlike type casting. Note that ftoint rounds to the
closest integer. If the decimal is .5, see x86-64 `cvtsd2si` doc to know if
it will round up or down. TODO: implement in stdlib, make .5 behavior reliable.

## Type casting

Type casting is an extremely powerful tool, but with great power comes great
responsibility. Basically, you're allowed to cast anything to anything else,
and the compiler won't complain about it. This lets you essentially entirely
bypass the type system, if you want to. However, this does mean that you have
the ability to feed nonsensical data anywhere you want, which may result in
less than ideal scenarios, and if you're lucky, a crash.

TODO: explan how type casting works, what you could theoretically do with it,
and how you can typecast asm statements.

## Include

The `#include <relative_path>` preprocessor macro is what allows you to split
code into multiple files. The way it works is extremely simple: the include will
essentially be substituted with the code from the included file.

It is also possible to include a folder, in which case the preprocessor will
look for an `include.dpl` file in that folder, and include this file.

## Namespaces

Namespaces lets you avoid conflicts when compartemetalizing code. The point should be
clear with an example: Assume you have two libraries, each of which implements
it's own `add` method. Without namespaces, including both will lead to a conflict
since `add` is defined twice, which is not allowed and will cause a compilation
error.

The solution: namespaces! Now, instead of having two `add` functions, you will
get `lib_a::add` and `lib_b::add`. Both are clearly distinct from eachother,
any ambiguity is removed and your code compiles!

Here is how to use namespaces in practice

lib_a.dpl
```
#namespace LIB_A

fn add(a: int, b: int): int {
	return a + b;
}
```

main.dpl
```
#include <lib_a.dpl>

fn main(): int {
	return LIB_A::add(1, 5);
}
```

A namespace can contain functions, constants, structs, enums, and nested namespaces.
All are accessed using the same `::` syntax. NOTE: In case the access path gets
too long, you can always use a #define macro to get a shorthand.

It is possible to close a namespace using the #spacename macro. The following
code is strictly equivalent to the one in the two files above

```
#namespace LIB_A

fn add(a: int, b: int): int {
	return a + b;
}

#spacename

fn main(): int {
	return LIB_A::add(1, 5);
}
```

In fact, what the preprocessor does is turn the code in multiple files
into the code in one single file.

NOTE: It is also possible to define namespaces with the `namespace [name];` and
`spacename;` keywords rather than the `#namespace` and `#spacename` macros.
However, this is not recommanded unless you're sure that you know what you're
doing. The entire point of the `#namespace` macro is that the preprocessor
will automatically add `#spacename` at the end of your file if you forget to
do it. If you instead use the `namespace` keyword, you could forget to add
`spacename` at the end, which could have very unpleasant to debug unintended
consequences. If your file with a missing `spacename` is included somewhere
else, the namespace will "spill over", and everything after that include will
also end up in the namespace. In principle, this should cause a compilation
error because a namespace won't have been properly closed by the time EOF is
reached, but a stray `spacename` keyword lost somewhere could remove this
safeguard. Spare yourself the pain and just use the macros, the preprocessor
translates them into the correct keywords for you.

All elements in code are considered as part of the `toplevel` namespace. This
means that you can use an "absolute" path to access anything by doing
`toplevel::some_namespace::some_function`. This also implies that `toplevel` is
a reserved namespace that can't be used elsewhere, to avoid ambiguities.

## Memory management

Just as in C, the DPL standard library comes with it's own `malloc` and `free`,
as well as a few other useful fonctions to manipulate the heap effectively.

It may come to the attention of DPL users that there is no way of
procedurally generating arrays. Something like `[0] * 8` generating an array of 0s
of size 8. The reason is simple: array literals are on the stack, and stack-based
structures must have a known size at compile time.

Instead, in order to generate structures whose size may be unknown at compile
time, the heap is made available by the standard library.

```
#include <std>
let arr: array[int] = std::mem::malloc(10);	// This returns an array[int] of the requested size

// ... do stuff with arr

std::mem::free(arr);	// Free the memory once it is no longer needed
```

There are a few things to note:
- The signature of malloc is `malloc(size: int): array[int]`. You may then cast
the `array[int]` you get to whatever type you desire.
- The values in the array returned by malloc are uninitialized and may contain
garbage. See `std::mem::calloc` or `std::mem::arrset` to initialize the memory.
- The argument is NOT a number of bytes, but a number of dwords, as
every type in DPL takes one dword.

Once you are done using the memory, you should free it so that it may be recycled.
Keep in mind: there is no garbage collector in DPL.

Other useful methods include the following
```
#include <std>

// This returns an array[int] of the requested size
let arr: array[int] = std::mem::malloc(10);

// Same as malloc, but initializes all the values in the array to 0
let arr2: array[int] = std::mem::calloc(10);

// Copies the values in arr to a new block of memory of the specified size and
// frees the old block. If The new size is less than the old, the memory block will be truncated.
let arr: array[int] = std::mem::realloc(arr, 15);  

// Set all the values in arr to 1. Note that the function signature is arrset:<T>(arr: array[T], value: T): T
std::mem::arrset:<int>(arr, 1);

// Free the memory once it is no longer needed
std::mem::free(arr);	

// Return the number of times malloc has been called in total
std::mem::malloc_call_count();

// Print the heap layout, for debugging purposes
std::mem::print_heap_layout();
```

## Generic types

The type system in DPL can be quite strict, but that is a problem if you want
to create generic functions or structures. For example, it would be terrible
having to write two different implementations for a linked list of ints and a
linked list of floats. This is where generics come in handy.

```
fn return2:<T, U, V, W>(i: T, j: U, k: V, l: W): T {
	let tmp: T = i;
	let tmp2: U = tmp + j;
	let tmp3: V = tmp2 + k;
	let tmp4: W = tmp3 + l;
	return tmp4;
}

fn main(): int {
	return return2:<int, int, int, int>(2, 3, 4, 5);
}
```

```
struct S:<T> {
	first_attribute: int;
	second_attribute: T;
	third_attribute: char;
	fourth_attribute: T;
}

fn main(): int {
	let a: S:<int> = S:<int>{ 1, 2, 'a', 3 };
	let b: S:<float> = S:<float>{ 1, 2.0, 'b', 3.0 };
	return a.second_attribute + a.fourth_attribute + a.first_attribute;
}
```

Generics can be used in functions or structs. Each one can have arbitrarily
many generics defined by writing `:<>` after the name of the function or struct.
These generics can then be used in the struct/function body, arguments and return types.

To use a generic function or struct, you bind a concrete type to each generic.
This concrete type will then substitute the generic in all the function/struct,
allowing you to only write a single function or struct for different types.