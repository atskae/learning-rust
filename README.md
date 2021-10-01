# Learning Rust
Documenting my learning process with the Rust programming language.

## Installation and Setup
* https://www.rust-lang.org/tools/install
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Check that installation worked:
```
rustc --version
cargo --version
```
Shows release number, commit hash and commit date.

## Development Notes
* Compile: `rustc <file>.rs`
* Create a project
    * Go to project directory, then run: `cargo new <project name>`
* Build a project: `cd <project-name; cargo build`
* Run a Cargo project: `cd <project-name>; cargo run`
    * This compiles and runs the project


## Language Notes
* Variables are immutable by default. Need `mut` for mutability.
    * `let mut level = 10;`


## Rust Project Structure
Result of `cargo new ...`:
```
rust-project/
    Cargo.toml <-- contains metadata and dependencies
    src/
        main.rs <--- function main() is the entry point
        ...
```

## Tutorials 
* https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/

