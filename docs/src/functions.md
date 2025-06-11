# Functions & Classes

## Function Declaration

```tundra
fun add(a, b):
    return a + b
```
The functions in tundra follow a very pythonic syntax
- Need to begin with `fun` keyword
- Followed by function name and parameters 
- Colon to begin definition
- Indent and followed by a regular code 

### Example
```tundra
fun fib(n):
    if (n < 2):
        return n
    return fib(n - 1) + fib(n - 2)
print(fib(10))

```
#### Bytecode
```bytecode
== == Disassembled Bytecode == ==
0000 Line    6    LoadConstant(0, Value { value: Function(RefCell { value: FunctionObject { name: "fib", arity: 1, chunk: RefCell { value: Chunk { code: [GetLocal(1, 0), Move(2, 1), LoadConstant(3, Value { value: Int(2) }), Less(4, 2, 3), JumpIfFalse(4, 2), GetLocal(3, 0), Return(3), GetGlobal(4, "fib"), GetLocal(3, 0), Move(2, 3), LoadConstant(5, Value { value: Int(1) }), Subtract(6, 2, 5), Move(5, 6), Call(5, 4, 1), Move(4, 5), GetGlobal(6, "fib"), GetLocal(2, 0), Move(7, 2), LoadConstant(8, Value { value: Int(2) }), Subtract(9, 7, 8), Move(7, 9), Call(8, 6, 1), Add(6, 4, 8), Return(6), Return(0)], constants: [], lines: [3, 3, 3, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6], max_register: 10 } }, jitted: None } }) })
0001 Line    6    DefineGlobal(0, "fib")
0002 Line    6    GetGlobal(0, "fib")
0003 Line    6    LoadConstant(1, Value { value: Int(10) })
0004 Line    6    Call(2, 0, 1)
0005 Line    6    Print(2)
0006 Line    7    Return(2)
```
#### Output 
    55
---
## Class Declaration

- To be implemented