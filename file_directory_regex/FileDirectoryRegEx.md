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
-------------------------------------------------------
## Directory and Path Related Functions
-------------------------------------------------------
### Path Related Functions
=======================================================
- We can use `r` before a string to make it a raw string, which means that the string will not be escaped when `\` is used.
- `path = Path::new(r"D:\PersonalProjects\Examples")` creates a new path object.
- To get parent directory of a path, we can use `path.parent()` function. Which returns an `Option<&Path>` type, and we can use `unwrap()` to get the value.
- We can get the file name from a path using `path.file_name()` function. Which returns an `Option<&OsStr>` type, and we can use `unwrap()` to get the value.
- We can also get the extension of a file using `path.extension()` function. Which returns an `Option<&OsStr>` type, and we can use `unwrap()` to get the value.
- `path.file_stem()` function returns the file name without the extension. Which returns an `Option<&OsStr>` type, and we can use `unwrap()` to get the value.
- We can create directory, subdirectory using `PathBuf` and `push` function.
- You can also use vector, iterators and `collect::<PathBuf>()` to create a `PathBuf` object.
- `path.is_dir()` function checks if the path is a directory or not.
- `path.is_file()` function checks if the path is a file or not.
- `path.exists()` function checks if the path exists or not.
- `path.metadata()` function returns the metadata of the file or directory, in the form of a `Result` type.
- After getting the `metadata` we can use the data obtained and get individual information like file size, creation time, modification time etc.
```rust
fn path_related_functions() {
    let path = Path::new(r"D:\PersonalProjects\Examples\my_text.txt");
    
    println!();
    println!("Getting File Metadata");
    // Getting parent directory
    println!("Parent directory of {:?} is {:?}", path, path.parent().unwrap());

    // Getting file name without extension
    println!("File name of {:?} is {:?}", path, path.file_stem().unwrap());

    // Getting file extension
    println!("Extension of {:?} is {:?}", path, path.extension().unwrap());

    // Getting the full file name
    println!("File name of {:?} is {:?}", path, path.file_name().unwrap());

    // Creating directories
    println!();
    println!("Creating directories");
    let mut dir_path = PathBuf::from(r"D:\PersonalProjects\Examples\NewDir");
    dir_path.push(r"Rust");
    dir_path.push(r"Examples");
    dir_path.push(r"MyFile");
    dir_path.set_extension("txt");
    println!("Full File Path created: {:?}", dir_path);

    // We can also do the same using iterators and vectors
    println!();
    println!("Creating directories using iterators and vectors");
    let paths = [r"D:\PersonalProjects\Examples\NewDir", r"Rust", r"Examples", r"MyFile.txt"];

    let new_path = paths.into_iter().collect::<PathBuf>();
    println!("Full File Path created: {:?}", new_path);

    // Checking if a file or directory exists
    println!();
    println!("Checking if a file or directory exists");
    println!("Checking if {:?} exists: {:?}", dir_path, dir_path.exists());

    println!("Checking if {:?} exists: {:?}", path, path.exists());

    println!("Checking if {:?} is a file: {:?}", dir_path, dir_path.is_file());

    println!("Checking if {:?} is a file: {:?}", path, path.is_file());

    println!("Checking if {:?} is a directory: {:?}", dir_path, dir_path.is_dir());

    println!("Checking if {:?} is a directory: {:?}", path.parent().unwrap(), path.parent().unwrap().is_dir());

    // Getting metadata
    println!();
    println!("Getting metadata of file and directory");
    let file_path = path;
    let dir_path = path.parent().unwrap();

    println!("Metadata of {:?} is {:#?}", file_path, file_path.metadata().unwrap());
    println!("Metadata of {:?} is {:#?}", dir_path, dir_path.metadata().unwrap());

    // Getting metadata attributes
    let file_metadata = file_path.metadata().unwrap();
    println!("Type of {:?} is {:?}", file_path, file_metadata.file_type());
    println!("Length of {:?} is {:?}", file_path, file_metadata.len());
    println!("Permissions of {:?} are {:?}", file_path, file_metadata.permissions());
    println!("Created time of {:?} is {:?}", file_path, file_metadata.created().unwrap());
    println!("Modified time of {:?} is {:?}", file_path, file_metadata.modified().unwrap());
}
```
=======================================================
### Directory Related Functions
=======================================================
- `path = Path::new(r"D:\")` creates a new path object.
- We can use `path.read_dir()` function to read the contents of a directory. It returns a `Result` type, which contains a `DirEntry` type.
- We can use `dir_entry.file_name()` function to get the file name from the `DirEntry` type.
- We can call `unwrap()` and `path()` functions to get the file name as a `PathBuf` type from `Result` obtained from `read_dir()` function.
- We can get current directory using `std::env::current_dir()` function, which returns a `Result` type containing a `PathBuf` type.
- We can create a new directory using `std::fs::create_dir()` function. It returns a `Result` type.
- To create a directory with `create_dir()` function, parent directories should exist. If they don't exist, we can use `create_dir_all()` function, it will create all the parent directories.

=======================================================
### Removing Files and Directories
=======================================================
- To remove a directory we can use `std::fs::remove_dir()` function. It returns a `Result` type. It will not work if the directory is not empty.
- `std::fs::remove_dir_all()` function can be used to remove a directory and all its contents. It returns a `Result` type. It will remove all the contents of the directory and then remove the directory.
- We can use `std::fs::remove_file()` function to remove a file. It returns a `Result` type.
- We can rename a file using `std::fs::rename()` function. It returns a `Result` type.
=======================================================
### Copying Files and Directories
=======================================================
- We can use `std::fs::copy()` function to copy a file. It returns a `Result` type.