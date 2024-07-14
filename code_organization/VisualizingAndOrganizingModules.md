# Visualizing and Organizing Modules


## `cargo modules`
- `cargo modules` is a tool that can be used to visualize and organize the modules in a Rust project.
- It can be installed using `cargo install cargo-modules`.
- It can be used to generate a tree/hierarchy of the modules in a project.
- Command for generating the tree is: `cargo modules structure --<options>`. For examples
```bash
cargo modules structure --all-features
cargo modules structure --lib
```
- You can also run only `cargo modules structure` to get all the options.


## Organizing Modules into Files and Folders
```
crate code_organization
├── mod customer: pub(crate)
│   └── struct Customer: pub
├── mod order: pub(crate)
│   └── struct Order: pub(self)
│       ├── fn calculate_discount: pub(self)
│       └── fn total_bill: pub(self)
└── mod product: pub(crate)
    ├── struct Product: pub
    │   ├── fn calculate_tax: pub(self)
    │   └── fn product_price: pub
    └── mod category: pub(self)
        └── enum Category: pub
```
![Output of `cargo modules structure --lib` in PowerShell for current project](image.png)

- Right now, modules are not mapped to the file system, instead modules are explicitly defined in the `lib.rs` file.
- For example, the `customer` module is defined in the `lib.rs` file as `mod customer`, not in a separate file.
- **Files themselves do not define module borders in Rust.**
- Rust, However, allows us to organize the codes by using typical system.

### Organizing Modules into Files
#### Method 1:
- **Make a new file with the same name as that of the module, and include the contents of the module there.**
- After separating the content of a module into file, like using `customer.rs` for the `customer` module, you can include the file in the `lib.rs` file using `mod customer;`.
- This is done to maintain the tree structure, and hierarchy of the modules.
- When Rust sees `mod customer;` in the `lib.rs` file, it will look for a file named `customer.rs` in the same directory.
- About modules containing sub-modules, you can create a folder with the same name as the module, and then create a file with the same name as the sub-module in that folder.

#### In Summary
- **For Simple Modules**, which have no sub modules, we can separate them in a file with the same name as the module, in the `src` directory itself.

- **For Modules with Sub Modules**, we can create a folder with the same name as the module, and then create a file with the same name as the sub-module in that folder, i.e. follow convention of typical file system.

- **This way, implementation details of modules can be hidden away, and code in lib.rs can be kept clean.**

#### Method 2:
- **Use a `mod.rs` file in the folder to include all the modules in that folder.**
- There is a downside to the second approach, there will be multiple files with the same name `mod.rs` in different folders.

#### In our code
- `customer` module is example of `Method 2`.
- `order` and `product` modules are example of `Method 1`.