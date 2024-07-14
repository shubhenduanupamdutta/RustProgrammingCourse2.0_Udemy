# Publishing Your Crates
#### We can also publish our own crates to `crates.io` for others to use.

## To publish a crate, follow these steps:

### Step 1: Create an account on `crates.io`
- Go to [crates.io](https://crates.io/) and create an account.
- Account creation requires a GitHub account.
- Go to account settings, add and verify your email address.
- Generate your API Token
    - Go to the API Tokens tab in your account settings.
    - Click on the New Token button.
    - Fill/Modify the properties/description as required.
    - Finally click on the `Generate Token` button.
    - Make sure to copy your token.

### Step 2: Login to `crates.io` using `cargo login` in the local terminal.
- Run the following command in the terminal:
```bash
cargo login <your-api-token>
```
- Replace `<your-api-token>` with the token generated in the previous step.
- Upon successful login, you will see a message,
```bash
Login token for `crates-io` saved
```
- This command will store the token in the `~/.cargo/credentials` file.

### Step 3: Create a `doc` to see how the published version will look like on `crates.io`
- Run the following command in the terminal:
```bash
cargo doc --open
```
- This will generate the documentation for the crate and open it in the browser.
- You can also run it using optional `no-deps` flag to generate documentation only for the current crate.
```bash
cargo doc --open --no-deps
```
- This will generate the documentation for the current crate only.

### Optional (But Recommended) Step 4: Properly document your code.
- Make sure to properly document your code using `///` comments.
- Let's try to add documentation for `Product` struct and `Category` enum.
- And recreate `doc` to see changes.
- Any documentation comments added to private items will not be included in the generated documentation.
- We can add sections like `Examples`, `Panics`, `Errors`, `Safety`, etc. to the documentation.
- They are used to provide additional information about the item, and can be added using `/// # Examples` or `/// # Panics`.
- We can use ` ``` ` to include code blocks inside the documentation comments.
```rust
/// # Examples
/// ```
/// let harry_potter_1 = Product::new( 1, "Harry Potter and the Philosopher's Stone".to_string(), 879.0, Category::Books);
/// assert_eq!(harry_potter_1.id, 1);
/// assert_eq!(harry_potter_1.name, "Harry Potter and the Philosopher's Stone".to_string());
/// ```
```

- We can also add `doc` tests to the code.
- And run `cargo test` to run the `doc` tests.
- To create documentation at crate root, you start the documentation with `//!` instead of `///`, and it will be included in the crate root documentation.
```rust
//! # <Section Name>
//! This is a rust library for online store.
//! 
```

### Step 5: Add necessary package metadata to `Cargo.toml`
- Add description field to the `Cargo.toml` file.
- Add `repository` field to the `Cargo.toml` file.
- Add `license` field to the `Cargo.toml` file.
- Add `documentation` field to the `Cargo.toml` file.
- Add `homepage` field to the `Cargo.toml` file.
```rust
[package]
name = "online_store"
version = "0.1.0"
authors = ["Your Name <"]
edition = "2021"
description = "A rust library for online store."
license = "MIT"
```
- You need to commit the newest changes to git before publishing the crate.
- Or you can pass `--allow-dirty` flag to `cargo publish` to publish the crate without committing the changes.

### Step 6: Publish the crate to `crates.io`
- Run the following command in the terminal:
```bash
cargo publish --allow-dirty
```
- This will publish the crate to `crates.io`.
- You can go to your dashboard on `crates.io` to see the published crate.
- You can't remove a crate once it's published.
- You can yank a version of the crate if you want to remove it from the index.
- Yanking a version will prevent new projects from using that version, but existing projects will still be able to use it.
