<div align="center">

  <img src="https://github.com/user-attachments/assets/3d8e9998-a738-40d6-b3fc-b16c38406081" alt="tundra_base_logo" width="300"/>

</div>


# Tundra Programming Language

Tundra is a Python-inspired, high-performance programming language implemented in Rust. It features a clean syntax designed for readability and a register-based virtual machine architecture that ensures efficient execution.

## Features

* **Pythonic Syntax:** Write code in a style that feels natural, clean, and intuitive.
* **High-Performance VM:** Tundra uses a register-based virtual machine built in Rust for maximum performance.
* **Efficient Memory Use:** Designed to handle modern workloads with low memory overhead.
* **Powerful Compiler:** Includes a robust compiler that generates optimized bytecode for the VM.

## Quick Example

```tundra
#fib.tundra

fun fib(n):


    if (n<=1):
        return n

    return fib(n-1) + fib(n-2)

var n = input()
var i = parseInt(n)
print(fib(i))

```

## Getting Started

To build and run your first Tundra program:

1. Clone the repository:

   ```bash
   git clone https://github.com/aayush-tripathi/tundra.git
   ```

2. Build the compiler and VM:

   ```bash
   cargo build --release
   ```

3. Run your program:

   ```bash
   ./target/release/tundra-cli path/to/your_program.tundra
   ```

## Future Plans for Tundra

* [ ] **JIT Compilation:** Integrate Just-In-Time compilation for dynamic performance boosts.
* [ ] **Standard Library:** Expand built-in functions and modules.
* [ ] **Interactive REPL:** Provide an interactive mode for faster experimentation.
* [ ] **Package Manager:** Simplify sharing and using Tundra libraries.

## Contributing

We welcome contributions! Whether you want to help with bug fixes, propose new features, or improve documentation, feel free to open a pull request or an issue.


## Acknowledgements

Built with love in Rust, inspired by Python, and optimized for speed.

---

**Happy coding with Tundra!**
