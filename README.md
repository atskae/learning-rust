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
* Create a project (a new package)
    * Go to project directory, then run: `cargo new <project name>`
* Build a project: `cd <project-name; cargo build`
* Run a Cargo project: `cd <project-name>; cargo run`
    * This compiles and runs the project
    * `Cargo.toml` contains instructions on how to build the crate
* Import from another file
    * In `my_module.rs` define importable functions with `pub fn`:
    ```
    pub fn my_test_function() { ... }
    ```
    * Have the caller import `my_module`  at the top with `mod`:
    ```
    mod my_module;
    ```
    * Call the module's function: `my_module::my_test_function()`
* Central Rust package (Crates) registrty: https://crates.io/
    * Put dependencies in `Cargo.toml`. `cargo build` will pull Crates from the registry
    ```
    [dependencies]
    regex = "1.4.2"
    ```
    * Check if a package is available `cargo search <package name>`

## Common Crates
* `serde` serialization and deserialization of data
    * `serde_json`


## Language Notes
* Variables are immutable by default. Need `mut` for mutability.
    * `let mut level = 10;`
    * The term "variable" and "binding" are interchangeable
        * "binding" is more appropriate in Rust since immutable by default
* Immutable: `str`, mutable: `String` (allocated on the heap!)
    * Can't do `let name: str = "atskae"`, need to use `&str`.
* **Only one thing can ever *own* a piece of data at a time**

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

* Syntactic sugar for catching errors: `some_func()?;`
Equivalent snippets:
```
fn function_1() -> Result(Success, Failure) {
    match operation_that_might_fail() {
        Ok(success) => success,
        Err(failure) => return Err(failure),
    }
}
```
```
fn function_2() -> Result(Success, Failure) {
    operation_that_might_fail()?
}
```

* Printing stuff
    * Add `#[derive(Debug)]` before struct definitions to be able to print them
    * `{:?}` let's us print vectors
        * Programmer-facing, used for debugging
    * `{:#}` is this pretty print??
        * Using this with `String` removes the quotations from printed result `"Pika!" vs. pika!`

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

* Scope
    * Denoted with `{ }`
        * `if`, `else`, `match`, etc.
    * Going out of scope = variable is "dropped"
        * Releasing any resources tied to that variable
            * Frees memory, closes files
    * Reminder: The terms "binding" and "variable" are interchangable in Rust
    * Bindings can have this associated with them, which are then freed when the bindings are "dropped"
        * The bindings *own* those things when in scope
        * `mascot` *owns* the `String` data associated with it
        ```
        {
            let mascot = String::from("baxter");
        }
        ```
        And the `String` itself owns the heap memory that holds the string's characters

* Move Semantics and Ownership
    * Transfer ownership from one binding to another
        * **Default behavior in Rust**
    * Use when we don't want variables to be "dropped" at end of scope
    * Old binding becomes invalid
    ```
    let mascot = String::from("baxter");
    let bearcat = mascot; // now mascot cannot be used
    ```
    * *Transferring ownership* == *moving*
        * `String` value *moved* from `mascot` to `bearcat`
    * Compile error if try to use a variable after move
    * Passing arguments to function *moves* the variable
    * The compiler tracks ownership

* Copying instead of moving
    * Variables must have the `Copy` trait for implicit copies to occur (instead of moving ownership)
        * These variables can be re-used
        * They don't have owners
        * Good for simple values, like integers
        * `u32` has copy trait
    * `String` and `vectors` do not have copy trait, so are moved
        * These can be expensive to copy
    * If no copy trait is present, we can explicity copy: `var.clone()`
        * Can slow down code since this duplicates memory

* References allow to borrow values without owning them
    * `&` indicates *borrow*
    ```
    let lotto = String::from("If I won the lotto, tomorrow, well I know");
    let in_the_heights_reference = &lotto;
    println!("{}", lotto); // still usable
    some_func(in_the_heights_reference); // usable
    ```

* Borrowing vs. Full-ownership
    * By default cannot change reference values. `&` = immutable borrow. Read-only.
    * Must declare with `mut`: `let mut greeting = ...`
        * Then function signature: `some_func(var: &mut String)`
    * `&mut` = mutable borrow. Read and write allowed

* Borrowing and mutable references, can only have *either or*:
    * One or more immutable references: `&T`
    * *Exactly one* mutable reference: `&mut T`
* Compiler will complain if this ^ is not followed!

* *Lifetimes*: Rust guarantees that all references point to valid items
    * In C/C++, it was possible that pointer can point to already freed memory, *dangling pointer*
    * A variable "lives longer" if its scope is longer
* **Borrow-checker** compares variable lifetimes and checks whether a borrow was valid
    * If a variable borrows from another variable whose lifetime is shorter, the program fails to compile
    * Must annotate if the compiler cannot determine lifetime
    ```
    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
    Annotation here is: `<'a>` and `&'a`, `a` can be anything
    * Generic lifetime is in angle brackets `<>`
    * Annotation above says "all parameters will live at least as long as the lifetime associated with each variable"
    * Can also apply lifetime annotation in structs 
    ```
    struct S<'a> {
        first: &'a str,
        last: &'a str,
    }
    ```

* Generic types
    * Like C++ templates (what a throwback)
    ```
    struct Point<T> {
       x: T, // holds any type T
       y: T, // must match the type of x
    }

    fn main() {
       let boolean = Point { x: true, y: false };
       let integer = Point { x: 1, y: 9 };
       let float = Point { x: 1.7, y: 4.3 };
       let string_slice = Point { x: "high", y: "low" };
    }
    ```

* Traits: common interface that types can implement
    * Abtract base class then?
    * `impl Trait for Type`
        * `Trait` is the interface, `Type` is the struct/enum
        * `impl Area for Circle`, `impl Area for Rectangle`
        * The `Trait` gets applied to the class `Type` after the class is defined... interesting
    * Can implement a single function
    ```
    use std::fmt;
    impl fmt::Display for Point {
        fn fmt(&self, ...) { ... }
    }
    ```
    * Function arguments can inidicates Traits in the function signature `&impl`:
    ```
    trait AsJson { ... }
    fn send_data(value: &impl AsJson) { ... }
    ```
    Function accepts any parameter that use the trait `AsJson`

    Another way ^ using generic types `T`:
    ```
    fn send_data<T: AsJson>(value: &T) { ... }
    ```
    * `impl<T> Container<T> { ... }`

* Deriving traits `derive`: have traits automatically implemented
    * Example: `#[derive(Debug, PartialEq)]`
        * Automatically implement debug printing and equality

* `Iterator` trait
    ```
    trait Iterator {
        type Item; // the type of the item that the container holds
        fn next(&mut self) -> Option<Self::Item>; // return either Some(Type) or None
    }
    ```

* Etsy
    * `== 0.0` seems to work
        * Equality between floats was with caution in C/C++ I thought
    * Define a constructor for a struct by implementing `new()`
        ```
        struct MyStruct { ... }
        impl MyStruct {
            fn new(...) -> MyStruct { 
                return MyStruct {
                    ...
                };
            }
        }
        ```
    * Seems very common not to use `return` keyword in examples... :/
        * Better get used to it I guess
    * Struct fields defined in a module (`mod my_module { ... }`) are private and cannot be accessed
        * Need to write *public* getter and setter methods
        * Don't need `mod my_module { ... }` if the module contents are in a separate file?
    * Inner modules are private by default, use `pub mod` to expose them
    ```
    mod my_module { // public
        mod my_sub_module_one { // inner sub module, private

        }

        pub mod my_sub_module_two { // inner sub module, public
            // my_module::my_sub_module_two
        }
    }
    ```


## Testing
* Mark tests with `#[test]`
    * Rust will only compile these when asked `cargo test`
* Tests typically contain `assert!`, `assert_eq!` macros
* Add `#[should_panic]` above a test that expects a panic to be raised
* Skip tests `#[ignore = "Some reason"]`
    * Skipped tests are still type checked and compiled
* Tests are usually in their own submodule, mark the module with `#[cfg(test)]`
    ```
    #[cfg(test)]
    mod add_function_tests {
        #[test]
        fn test_add_works() {
            ...
        }
    
        #[test]
        #[should_panic]
        fn test_add_fails() {
            ...
        }
    
        #[test]
        #[ignore]
        fn add_negatives() {
            ...
        }
            
    }
    ```
    * `cfg` = conditional compilation
        * `cfg(predicate)`, compile if `predicate` is true
        * `cfg(test)` means if the test flag is set when calling `cargo`, compile and run these


### Documentation Testing
* Doc tests are tests inside *documentation comments*
* Triple slashes `///` in source code indicate documentation comments
    * Format is Markdown
* Only work for library projects, so need to start a project with `cargo new --lib my_lib`


### Integration Tests
Test how others use the code, if modules work together
* Exist in a separate directory and file to externally test the library code
    * Put in `my_lib/tests/` directory
    ```
    my_lib/
        src/
        tests/
    ```
* Only library crates can be integration tested
    * Binary crates don't expose functions that others can use
        * To test binary crates, include `src/lib.rs` with most functinality in `src/main.rs`?
        * Then can import the library


## Rust Project

### Vocab
* *Package* a collection of one or more crates, contains information on how to build the crates
* *Crate* smallest unit of code that can be compiled
    * Compile result is a binary (executalbe) or a library
    * Has a module name
* *Module* code inside a Crate
* *Item privacy*: either public (can be used outside of the module) or private (internal-module use only; internal implementation details)
    * Controlled by modules

### Project Structure
Get basic structure with `cargo new ...`.
```
rust-project/
    Cargo.toml <-- contains metadata and dependencies
    src/
        main.rs <--- function main() is the entry point
        lib.rs <--- library file // not generated from `cargo new`
        ...
    bin/ <--- contains executables, "Crates" // not generated from `cargo new`
```
To get `lib.rs` file (instead of a `main.rs` file): `cargo new --lib my-library`
* Compilation result: `libmy_library.rlib`
    * This can be published and linked to other project
* The module name is automatically taken from the library name passed in (`my_library`, for example)
    * Access: `my_library::my_function`

## Tutorials 
* https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/

