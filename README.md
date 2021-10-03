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
* Immutable: `str`, mutable: `String` (allocated on the heap!)
    * Can't do `let name: str = "atskae"`, need to use `&str`.

* Structs
    * Classic C structs: `struct Student { name: String, level: u8 }`
        * Doesn't need a semicolon at the end
    * Tuple struct: `struct Animal(char, f32);`
        * Defines the field types of the tuple to be instantiated later
        * But this needs a semicolon at the end???
    * Union struct: `struct Unit`
    * To print structs with `println!()`, add `#[derive(Debug)]` before the struct definition

* String funkiness
    * When instantiating structs with string fields, need: `String::from("my string")`, convert `String` to `&str`...

* Functions
    * Can return values two ways:
        * `return <val>;` <-- Note semicolon here
        * `<val>` <-- Note *no* semicolon here
        * I don't like that...

* Printing stuff
    * Add `#[derive(Debug)]` before struct definitions to be able to print them
    * `{:?}` let's us print vectors
    * `{:#}` is this pretty print??

* Arrays and Vectors
    * Vectors are *not* mutable by default...
        * Need: `let mut fruits = Vec::new();`
    * Normal vector: `vec!["stuff", "stuff", "more stuff"]`
        * How is a immutable vector different from normal array then?

* HashMaps `let mut reviews: HashMap<String, String> = HashMap::new();`
    * `my_hash_map.get()` return `Some(value)`

* Loops
    * Can return value of a loop with `break`:
    ```
    let my_num = loop {
        counter += 1;
        if counter > 10 {
            break counter; // return counter value
        }
    }
    ```

* Error handling
    * `panic!("message")` terminates the program


* Optional arguments `Option<type>`
    * Rust is explicit when arguments can be empty (`None`)
    * `None` and `Some` are variants of `Option<T>`
        ```
        enum Option<T> {
            None,     // The value doesn't exist
            Some(T),  // The value exists
        }
        ```
        * Returning `Option<T>` returns `Option::Some(value)` or `Option::None`
        * So functions that take an argument `Option<T>` cannot take `None` or `Some` as a replacement to `Option<T>` (?)
    * Print `Option<T>` values with `{:?}`
        * `println!("{:?}", my_vector.get(0));`
    * View the value of `Some(<value>` with `Some(<value>).unwrap()`:
        * This will `panic!` if the value is `None`
            * Use `unwrap_or(<default value>)` to return a default value if the value is not found (and avoid a panic)
        * `Some(<value>).expect("Custom panic message")`
    * Check if an `Option` is `None`: `Some(value).is_none()`

* `Result` type for propagating errors
    ```
    enum Result<T, E> {
        Ok(T):  // A value T was obtained.
        Err(E): // An error of type E was encountered instead.
    }
    ```
    * `Option` indicates *absence* of a value
        * `Result` indicates if a failure occurred
    * `Result.unwrap()`, `Result.expect()`
    * If returning `Result`, then either return:
        * `Ok(value)` or `Err(value)`

* Pattern matching
    * `match`
    ```
    let pokemon = vec![
        "raichu",
        "golem",
        "quilava",
        "muk",
        "wurmple"
    ];
    for &index in [0, 2, 3, 99].iter() {
        match pokemon.get(index) {
            // Comma-separated values of each pattern
            // ^ each is called a "match arm"
            Some(&"muk") => println!("Sludge!!!!"),
            Some(pokemon_name) => println!("I choose {}!", pokemon_name), // pokemon_name gets created and scoped only here
            None => println!("Empty pokéball...")
        }
    }
    ```
        * Match arms are evaluated top to bottom
            * Specific match arms should go to the top, more generic toward bottom, otherwise bottom match arms will not get reached
        * Every possible pattern *must* be accounted for, or else will get a compiler error
    * `if let`: match a single pattern
        * Equivalent:
        ```
        let num: Option<u8> = Some(7);
        match num {
            // comma-separated values!! I keep forgetting
            Some(7) => println!("Wahoo!"),
            _ => {},
        }
        ```
        `if let` is a shortcut for the above.
        ```
        let num: Option<u8> = Some(7);
        if let Some(7) = num {
            println!("Wahoo!"); // semicolon here
        } 
        ```

* Etsy
    * `== 0.0` seems to work
        * Equality between floats was with caution in C/C++ I thought


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

