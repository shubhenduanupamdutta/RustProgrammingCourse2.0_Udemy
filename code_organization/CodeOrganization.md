# Code Organization
<strong><h3>
Three basic components we need to know about code organization in Rust are:
<h4>
<li> Packages
<li> Crates
<li> Modules
</h4>
</h3></strong>

## Packages
- A `package` is highest level of code organization in Rust.
- A package is created using the command `cargo new <package_name>`.
- A package may contain one or more `crates`.
- Each crate provides a set of functionality and allows you to build tests and share crates, all these is managed through `cargo` commands such as `cargo build`, `cargo test`, `cargo run`, etc.
- Inside a package, there is a `Cargo.toml` file which contains metadata about the package.
- `Cargo.toml` serves as central configuration file, and manages,
    - dependencies
    - package metadata
    - How to build the crates
    - Information on optional features

## Crates
- A `crate` is lower level organization than a package.
- A `crate` is a compilation unit in Rust that houses a set of `modules`, and their associated items, such as `functions` and `structs`.
- From code organization perspective, a `crate` is essentially a `tree of modules`, representing a hierarchial structure of the code.
- A `crate` can be compiled into a `binary` or a `library`.
- A `library crate` is a piece of code that produces a library, which can be used by other crates. It is for sharing purpose and not for execution.
- A `binary crate` is a piece of code that produces an executable, which can be executed.
- A `binary crate` may contain code from `library create` as well.

## Modules
- A `module` is a way to organize code within a crate.
- A `module` allows you to group items at a finer level.
- A `module` can also control:
    - visibility of items
    - privacy of items
    - structure of the code


## Rules for Packages
- **A package must contain at least one crate.**
- **A package can contain at most 1 library crate.**
- **A package can contain any number of binary crates.**
```
// Visual representation of code organization in Rust 
package
    |
    |--- crate
            |
            |--- module
                    |
                    |--- items
```
- `Binary Crate` is a crate that produces an executable, with root being `main.rs`.
- `Library Crate` is a crate that produces a library, with root being `lib.rs`.
- Each `crate` contains a `root` module, which is the starting point of the crate, and may contain nested modules.

#### - Additional binary `crate` than the default `src/main.rs`, can be created inside the `src/bin` directory.
- Each `binary crate` must have a main function as an entry point.
- `cargo run` will not work with this additional binary crate, as it is not the default binary crate.
- To run the additional binary crate, use `cargo run --bin <binary_crate_name>`.

- We don't have any binary crate by the name of `code_organization` but `cargo` shows this, this is because of naming convention in cargo.
- `Cargo` adheres to the rule that if `src/main.rs` is present, then the binary crate name is the name of the package.
- Similarly if `src/lib.rs` is present, then the library crate name is the name of the package.
