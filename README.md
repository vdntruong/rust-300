# rust-300

<img src="/images/rustacean-orig-noshadow.png" width="100"/>

300 bài code thiếu nhi tả xung hữu đột

## Rust toolchain

`rustc` the Rust compiler

- `rustc main.rs` build a binary `main` file from `main.rs`, for small projects

`cargo` Rust's package manager and build system

- `cargo --version` to show `cargo` version
- `cargo new <project-name> --vcs=None` to create new project
- `cargo build` to build ours project, result of the build in the `targe/debug` folder
- `cargo run` is more frequenly used than `cargo build`, to build and run ours project
- `cargo check` quickly checks the code
- `cargo build --release` to compile project with optimizations, `target/release`
- `cargo update` to update a crate

`cargo fmt` to invoke `rustfmt` and format the code automatically

`rustdoc` Rust's documentation tool

`rustup` a command line tool for managing Rust versions and associated tools

- `rustup update` to update a newly released version
- `rustup self uninstall` to uninstall Rust and `rustup`
- `rustup doc` to open the local documentaion

## Ownership Rules

- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## Syntax

- `extern crate <crateRef asClause>;`
An extern crate declaration specifies a dependency on an external crate.
