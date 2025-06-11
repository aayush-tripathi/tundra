# Operators

Tundra’s operators fall into these categories:

| Category        | Symbols                                         |
|-----------------|-------------------------------------------------|
| Arithmetic      | `+`, `-`, `*`, `/`, `//` (floor div), `%`, `**` |
| Comparison      | `==`, `!=`, `<`, `<=`, `>`, `>=`               |
| Boolean         | `and`, `or`, `not`                             |
| Bitwise         | `&`, `|`, `^`, `~` (via `!` on booleans)        |
| Assignment      | `=`, `+=`, `-=`, `*=`, `/=`, `//=`, `%=`, `**=` |
| Increment/Dec   | `++x`, `--x`                                    |
| Member access   | `obj.field *`, `arr[index]`                      |
| Function call   | `fn(arg1, arg2)`                               |

**Precedence** (highest → lowest):

1. **Primary**  
   - identifiers, literals, parenthesized expressions  
   - array indexing `arr[i]`, field access `obj.f`, argument-less calls `fn()`

2. **Call**  
   - function calls: `fn(arg1, arg2)`

3. **Exponentiation**  
   - `**`

4. **Unary**  
   - prefix `-`, `not`

5. **Factor**  
   - `*`, `/`, `//`, `%`

6. **Term**  
   - `+`, `-`

7. **Bitwise AND**  
   - `&`

8. **Bitwise OR**  
   - `|`

9. **Bitwise XOR**  
   - `^`

10. **Comparison**  
    - `<`, `<=`, `>`, `>=`

11. **Equality**  
    - `==`, `!=`

12. **Logical AND**  
    - `and`

13. **Logical OR**  
    - `or`

14. **Assignment**  
    - `=`, `+=`, `-=`, …, `**=`

---

## Example:

```tundra
var x = 2 + 3 * 4         # 14
var y = (2 + 3) * 4       # 20
var z = not true or false # false
print(x)
print(y)
print(z)
```
### Bytecode
```bytecode
== == Disassembled Bytecode == ==
0000 Line    2    LoadConstant(0, Value { value: Int(2) })
0001 Line    2    Move(1, 0)
0002 Line    2    LoadConstant(2, Value { value: Int(3) })
0003 Line    2    Move(3, 2)
0004 Line    2    LoadConstant(4, Value { value: Int(4) })
0005 Line    2    Multiply(5, 3, 4)
0006 Line    2    Add(4, 1, 5)
0007 Line    2    DefineGlobal(4, "x")
0008 Line    3    LoadConstant(4, Value { value: Int(2) })
0009 Line    3    Move(5, 4)
0010 Line    3    LoadConstant(1, Value { value: Int(3) })
0011 Line    3    Add(3, 5, 1)
0012 Line    3    Move(1, 3)
0013 Line    3    LoadConstant(5, Value { value: Int(4) })
0014 Line    3    Multiply(6, 1, 5)
0015 Line    3    DefineGlobal(6, "y")
0016 Line    4    LoadConstant(6, Value { value: Bool(true) })
0017 Line    4    LoadConstant(1, Value { value: Bool(false) })
0018 Line    4    Equal(5, 6, 1)
0019 Line    4    Move(1, 5)
0020 Line    4    LoadConstant(6, Value { value: Bool(false) })
0021 Line    4    BitwiseOr(7, 1, 6)
0022 Line    4    DefineGlobal(7, "z")
0023 Line    5    GetGlobal(7, "x")
0024 Line    5    Print(7)
0025 Line    6    GetGlobal(7, "y")
0026 Line    6    Print(7)
0027 Line    7    GetGlobal(7, "z")
0028 Line    7    Print(7)
0029 Line    8    Return(7)
```

### Output
14
20
false


## Note
\* Classes are not yet implemented in Tundra