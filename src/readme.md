// Variable
// Control flow
// shadowing re assigning the value e.g let a =2, let a=3;
let mut height
// no semi colon mean its return line
// strongly typed language

1. Introduction to Rust: Learn the basics of Rust and its key features.
2. Choosing Rust: Discover the reasons why developers prefer Rust over other languages.
3. Essential Components and Tools: Understand the fundamental components and tools necessary for Rust development.
4. Hands-on Experience: Gain practical experience by working with code in the Rust playground.
   https://learn.microsoft.com/en-us/training/modules/rust-introduction/

Rust doesn't have a large runtime or garbage collector

1. No Large Runtime
   A runtime is a set of code or functionality that runs alongside your program to manage things like memory allocation, garbage collection, and error handling.
   Many programming languages (e.g., Java, Python) have large runtimes that handle features like:
   Automatic memory management (garbage collection).
   Just-in-time (JIT) compilation.
   Dynamic type checking.
   Rust, in contrast:

Doesn't include a large runtime.

Provides minimal runtime features for program execution (e.g., stack unwinding for panics).
Leaves memory and resource management to the developer or compiler, rather than including extensive background processes.
Type safe: The compiler assures that no operation will be applied to a variable of a wrong type.
Memory safe: Rust pointers (known as references) always refer to valid memory.
Data race free: Rust's borrow checker guarantees thread-safety by ensuring that multiple parts of a program can't mutate the same value at the same time.
Zero-cost abstractions: Rust allows the use of high-level concepts, like iteration, interfaces, and functional programming, with minimal to no performance costs. The abstractions perform as well, as if you wrote the underlying code by hand.
Minimal runtime: Rust has a minimal and optional runtime. The language also has no garbage collector to manage memory efficiently. In this way, Rust is most similar to languages like C and C++.
Targets bare metal: Rust can target embedded and "bare metal" programming, making it suitable to write an operating system kernel or device drivers.

Create new project templates with the cargo new command.
Build a project with the cargo build command.
Build and run a project with the cargo run command.
Test a project with the cargo test command.
Check project types with the cargo check command.
Build documentation for a project with the cargo doc command.
Publish a library to crates.io with the cargo publish command.
Add dependent crates to a project by adding the crate name to the Cargo.toml file.
The Cargo.toml file is the manifest file for Rust. It's where you keep metadata for your project and also any dependencies.

oop: A keyword used for an infinite loop, which can be exited using a break statement.

while: A conditional loop that continues as long as its condition is true.

for: A loop that iterates through elements of a collection or range.

break: A control flow keyword to exit the current innermost loop early.

mutability: The ability for a variable to have its value changed during runtime by marking it as mutable with the mut keyword

option: A Rust enum type that can be either Some(value) or None, used to represent optional values.

continue: A control flow keyword to skip an iteration and move on to the next one in the same loop.

if let: A pattern matching construct that allows you to conditionally bind variables based on their match against a given value or pattern.

sum: An enum type wrapper around Option<T> which can be either Some(value) or None.

range: Represents a sequence of numbers, often used in loops for iteration purposes.

shadowing: A variable redeclaration with the same name but different value and/or scope within the same context.

////

Key Terms
loop: A keyword used for an infinite loop, which can be exited using a break statement.

while: A conditional loop that continues as long as its condition is true.

for: A loop that iterates through elements of a collection or range.

break: A control flow keyword to exit the current innermost loop early.

mutability: The ability for a variable to have its value changed during runtime by marking it as mutable with the mut keyword

option: A Rust enum type that can be either Some(value) or None, used to represent optional values.

continue: A control flow keyword to skip an iteration and move on to the next one in the same loop.

if let: A pattern matching construct that allows you to conditionally bind variables based on their match against a given value or pattern.

sum: An enum type wrapper around Option<T> which can be either Some(value) or None.

range: Represents a sequence of numbers, often used in loops for iteration purposes.

shadowing: A variable redeclaration with the same name but different value and/or scope within the same context.

// options
let user_2 = Student { name: "Dyson Tan".to_string(), level: 5, remote: false };
let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
// adt resouces
https://en.wikipedia.org/wiki/Algebraic_data_type
The #[derive(Debug)] syntax lets us see certain values during the code execution that aren't otherwise viewable in standard output. To view debug data with the println! macro, we use the syntax {:#?} to format the data in a readable manner.
https://learn.microsoft.com/en-us/training/modules/rust-create-program/8-summary#completion
