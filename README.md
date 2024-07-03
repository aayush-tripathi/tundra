# Tundra Programming Language

Tundra is a dynamically typed, register-based programming language designed for simplicity and efficiency written completely in Rust. It features a Pythonic syntax , making it ideal for scripting and rapid development. The language includes common data types such as boolean, string, integer, float, and character. Tundra supports basic arithmetic operations, comparison operators, bitwise operations, and control flow statements including if-else blocks and loops. It compiles to bytecode executed on a register-based virtual machine with a fixed-size register set of 128. 


*Inspired by [CraftingInterpreters](https://craftinginterpreters.com/) (where a Stack Based C Implementation of the same exists , I am trying to challenge myself by adding support for more functionalities and using a register based VM)*
## Example

```tundra
var x = 42
var y = x + 3.14
print(x, y)
