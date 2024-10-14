# Dubious-Programming-Language

Based on https://norasandler.com/2017/11/29/Write-a-Compiler.html

TODO: Finish all basic operators
TODO: %include
TODO: %define
TODO: Variables
TODO: Function calls
TODO: String literals
TODO: Structs and enums?
TODO: Checker
TODO: binary and hex literals
TODO: make generator write comments in asm file

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
12. **Logical AND**: `a && b`
13. **Logical OR**: `a ^^ b`
14. **Logical XOR**: `a || b`
15. **Assignment operators**: `a = b`, `a += b`, `a -= b`, `a *= b`, `a /= b`, `a %= b`, `a <<= b`, `a >>= b`, `a &= b`, `a ^= b`, `a |= b`