# Popper-lang

![Popper-lang Logo](https://example.com/popper-lang-logo.png)

**Popper-lang** is a versatile programming language written in Rust that simplifies complex tasks. This repository serves as the command-line interface (CLI) for Popper-lang, providing users with a seamless way to interact with the language. Additionally, it acts as a central hub for the Popper-lang ecosystem, bringing together the core components of the project: **Popper-compiler** and **Popper-cpu**.

## Features

- Easy-to-use command-line interface for Popper-lang.
- Integration with the Popper-compiler project, written in Rust, for code compilation.
- Support for running Popper-lang code on the Popper-cpu emulator, also implemented in Rust.

## Installation

To get started with Popper-lang, you can install the CLI using Cargo, Rust's package manager:

```bash
cargo install popper-lang
```

For detailed installation instructions and system requirements, please refer to the Installation Guide.

# Usage
Once installed, you can use the Popper-lang CLI, written in Rust, to perform various tasks, such as:

 - Compiling Popper-lang source code with the Popper-compiler.
 - Running Popper-lang programs on the Popper-cpu emulator.
 - Managing your Popper-lang projects and dependencies.
 - For detailed usage instructions, please refer to the User Guide.

# Syntax
Popper-lang features a clear and concise syntax that makes it easy for developers to write and understand code. Here are some key elements of the Popper-lang syntax:

Variables are declared using the let keyword, followed by the variable name and optional type annotation.

```
let x: int = 42;
```
Conditional statements are written using if, else if, and else.
```
if condition {
    // Code to execute if the condition is true
} else if another_condition {
    // Code to execute if another condition is true
} else {
    // Code to execute if none of the conditions are true
}
```
Loops can be created using for and while constructs.

```
for i in 0..10 {
    // Code to repeat for each value of i from 0 to 9
}
```

```
while condition {
    // Code to execute while the condition is true
}
```
Functions are defined using the `fun` keyword.

```
fun add(a: int, b: int): int {
    return a + b;
}
```
For a more comprehensive guide on the Popper-lang syntax and language features, please refer to the Language Documentation.

# Popper-compiler
[The Popper-compiler](https://github.com/popper-lang/popper-compiler) is a core component of the Popper-lang project, written in Rust, responsible for compiling Popper-lang source code into executable programs. It offers features like code optimization and error checking, ensuring your Popper-lang programs run efficiently and reliably.

# Popper-cpu
[The Popper-cpu project](https://github.com/popper-lang/popper-cpu/), also implemented in Rust, provides a software-based emulator for running Popper-lang programs. Please note that Popper-cpu does not currently support debugging.

# Contributing
We welcome contributions from the Rust and Popper-lang community! If you'd like to contribute to the Popper-lang project, please follow our Contribution Guidelines.

# License
Popper-lang is open-source software released under [the MIT License](/LICENSE). Feel free to use, modify, and distribute it in accordance with the terms of the license.

# Contact
If you have any questions, issues, or suggestions regarding Popper-lang, please feel free to open an [issue](https://github.com/popper-lang/popper-lang/issues) on this repository

Thank you for using Popper-lang!

