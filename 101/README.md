# 101: Hello World and Cargo

## Installation

Before installation, please make sure the c compiler is install.

For linux/Mac OSX, please run:
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## A Compiler or An Interpreter?

Like C and C++, Rust uses a compiler to translate human-readable code into machine code. Since a compiler goes through all the program before compiling, it can't provide an interactive coding-session as programming languages like Python, javascript, and etc.

When we compile a C program, we call *gcc*. Similarily, we call *rustc* to compile a Rust program. Entering *rustc filename* will build the executable.

## main.rs and main()

Conventially, a Rust file ends with *rs* and the first execution takes place at entrypoint **main()** (Just similar to C/C++):

```rust
fn main(){
	// Do something
	println!("Hello, world!");
}
```

Note: *fn* is a reserved keyword for function declaration. In addition, println! is *not a function* but a *marco*. **!** is the marco identifier in the language.

## Cargo

Cargo is the built-in system for building Rust-app and package management in Rust. The following are commands are the most essential:

1. *cargo new \<project_name\>*: Cargo will create a new Rust project. A folder name as project_name is created. Under the folder, there are two directories: *src* is where we put Rust code and *target* is where the compiled binary is put. Inside the *src*, a *main.rs* is ready for use. If the folder is not inside a git repository, a git repository is created.
2. *cargo build*: Cargo will build the program from the code in the *src* folder. In default, the compiled result will be found in the *target/debug* folder. To create a release version, we can simply add *--release* flag. The output of the binary is in *target/release*.
3. *cargo run*: The command will build a program and execute it. It executes the binary in *target/debug* directory. We can execute the released version by adding *--release* flag.
4. *cargo check*: Instead of building a executable, the command will only check if the code is compilable. This method can reduce the debugging time dramatically.

## TOML

TOML is created when *cargo new* command is call. The file is the meta information about the project and the dependenices that are need for execution.

## References
1. [Differences between a compiler and an interpreter By GeeksForGeeks](https://www.geeksforgeeks.org/difference-between-compiler-and-interpreter/)
2. [Differences between a compiler and an interpreter By Business Insider India](https://www.businessinsider.in/difference-between-compiler-and-interpreter/articleshow/69523408.cms)
3. [Keywords - The Rust Reference](https://doc.rust-lang.org/reference/keywords.html)
4. 