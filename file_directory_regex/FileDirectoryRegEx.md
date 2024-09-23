# Text Processing (RegEx), File and Directory Handling
-------------------------------------------------------
-------------------------------------------------------
## Basic File Handling
-------------------------------------------------------
- Some basic standard modules for file handling are
```rust
use std::fs::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
```
- `std::fs` module contains basic methods to manipulate the contents of local file system. The methods in this module represents cross-platform file system operations. It provides functions like
    - copying the contents of one file to another, 
    - creating a new empty directory
    - read the contents of a file
    - removes an empty directory
    - write contents to a file and other related operations.
- `std::io` module contains a number of common things that will be needed when doing input and output, or read or write to textual files.
- `std::path` module provides a number of operations for inspecting a path, including but not limited to
    - breaking the paths into its components
    - extracting the file name
    - determining whether the path is absolute or relative
    - joining paths together
=======================================================
- `std::io::Result` is a specialized `Result` type for io operations. It is broadly used across the standard io module for representing the output of any operation which may produce an error.
- `std::fs::File::create(path: &Path)` creates a new file at the given path. If the file already exists, it will delete the content of the file and return a brand new empty file, so be careful with this method.
- `file = std::fs::File::create(some_path)` from here on.
- `file.write(buf: &[u8])` writes the given buffer to the file. It returns the number of bytes written inside `Ok` value `Ok(n: usize)` or an error if the write operation fails. Please note tha this function needs a buffer, which is nothing but bytes in the form of a `u8 array`.
- We can also use `.as_bytes()` function to convert a string to a byte array.
- Usually `create()` function will delete the content of the file if it already exists. If you want to append to the file, you can use `std::fs::OpenOptions` to open the file in append mode, and then use `write()` function to write to the file.
- We can also store program outputs in file, idea behind is to convert any variable/type to string then convert it to buffer of `&[u8]` using `as_bytes()` function. Then write this buffer to file.
```rust
use std::fs::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

fn file_handling() -> std::io::Result<()> {
    let path_location = r"D:\my_text.txt";
    let path = Path::new(path_location);
    println!("Path to file: {:?}", path);

    {
        let mut file = File::create(path)?;

        println!("File created successfully.");

        file.write(b"Let's put this in the file.")?;
        file.write("My name is shubhendu".as_bytes())?;

        println!("Data written successfully.");
    }
    // File should close after going out of scope

    // Following scope, we open the file in append mode and then as file goes out of scope it will close.
    {
        let mut file = OpenOptions::new().append(true).open(path)?;

        file.write(b"\nLet's put this in the file.")?;

        // Storing program output as a variable in the file
        let str1 = "Shubhendu";
        file.write(str1.as_bytes())?;

        // Storing the vector into a file
        let some_vec = vec![1, 2, 3, 4, 5];
        let str_from_vec = some_vec
            .into_iter()
            .map(|i| format!("{} \n", i.to_string()))
            .collect::<String>();
        file.write(str_from_vec.as_bytes())?;

        let (name, age) = ("Dutta", 34);
        let formatted_str = format!("I am {} and my age is {}", name, age);
        file.write(formatted_str.as_bytes())?;
    }

    // Reading the file, in one go
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!("The file contains {:?}", contents);
    }

    // Reading the file line by line
    {
        let file = File::open(path)?;
        let file_buffer = BufReader::new(file);
        for (i, line) in file_buffer.lines().enumerate() {
            println!("Line {}: {:?}", i, line?);
        }
    }

    Ok(())
}

pub fn main() {
    let _ = file_handling();
}
```
- We can call `file_handling()` function from `main()` function to run the code.
