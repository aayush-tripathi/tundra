# Standard Library

Tundra currently provides these built-ins:

| Name        | Arguments | Description                         |
|-------------|-----------|-------------------------------------|
| `print(x)`  | 1         | Print a value to stdout with newline |
| `input()`   | 0         | Read a line from stdin as string     |
| `parseInt(s)` | 1       | Convert string `s` to integer       |
| `parseFloat(s)` | 1     | Convert string `s` to float         |
| `len(arr)`  | 1         | Length of array                     |
| `Array(n)`  | 1         | Create new array of length `n`      |

## Example:

```tundra
var name = input()
print("Hello, " + name)
var nums = Array(5)
print(len(nums))  # → 5
```

### Bytecode

```bytecode
== == Disassembled Bytecode == ==
0000 Line    2    GetGlobal(0, "input")
0001 Line    2    Call(1, 0, 0)
0002 Line    2    DefineGlobal(1, "name")
0003 Line    3    LoadConstant(1, Value { value: String("\"Hello, \"") })
0004 Line    3    Move(0, 1)
0005 Line    3    GetGlobal(2, "name")
0006 Line    3    Add(3, 0, 2)
0007 Line    3    Print(3)
0008 Line    4    GetGlobal(3, "Array")
0009 Line    4    LoadConstant(2, Value { value: Int(5) })
0010 Line    4    Move(4, 2)
0011 Line    4    Call(0, 3, 1)
0012 Line    4    DefineGlobal(0, "nums")
0013 Line    5    GetGlobal(0, "len")
0014 Line    5    GetGlobal(3, "nums")
0015 Line    5    Move(1, 3)
0016 Line    5    Call(2, 0, 1)
0017 Line    5    Print(2)
0018 Line    6    Return(2)
```
### Output 
Dexter Morgan <-stdin
""Hello, "Dexter Morgan"
5