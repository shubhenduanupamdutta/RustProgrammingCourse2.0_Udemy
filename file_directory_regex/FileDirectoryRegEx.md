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
-------------------------------------------------------
## Regular Expressions Basics
-------------------------------------------------------
- Regular expressions are a powerful tool for text processing. They are used to search, replace, and extract text based on a pattern.
- We have two inputs, one is the text we want to search in, and the other is the pattern (RegEx) we want to search for.
- Output is the text that matches the pattern.
- `RegEx` is usually provided in many programming language as standard library, like in Python, Java etc.
- But in Rust, we have to use `regex` crate to use regular expressions, since the intention in Rust is to keep standard library minimal.
- We add `regex` crate in `Cargo.toml` file as
- Then we can use `regex` crate in our code as
```rust
use regex::Regex;

pub fn main() {
    let re = Regex::new(r"[prt]ain").unwrap();
}
```
- `Regex::new()` function is used to create a new `Regex` object. It returns a `Result` type, so we can use `unwrap()` to get the value. It will return and error if the pattern is not valid.
- There are many constructs in regular expression. But the most fundamental one is called `character class`. It is a set of characters enclosed in square brackets `[]`. It matches any one of the characters in the set.
- In our case `[prt]ain` will match `pain`, `rain` and `tain`. We are looking words that contain, either `p`, `r` or `t` followed mandatorily by `ain`.
- We can use `is_match()` function to check if the pattern matches the text or not.
- `.find()` function can be used to find the first occurrence of the pattern in the text.
- `.find_iter()` function can be used to find all the occurrences of the pattern in the text.
- `.capture_iter()` function can be used to capture the groups in the pattern, it finds successive non-overlapping matches.
```rust
use regex::Regex;

pub fn main() {
    let re = Regex::new(r"[prt]ain").unwrap();

    let text = "rrain spain none";
    println!("The text: {:?}\nhas a match: {:?}", text, re.is_match(text));

    println!();
    println!("Finding all matches in the text:");
    for mat in re.find_iter(text) {
        println!("Found: {:?}", mat.as_str());
    }

    println!("Finding first match in the text:");
    if let Some(mat) = re.find(text) {
        println!("Found: {:?}", mat.as_str());
    }

    println!();
    println!("Finding all matches in the text using captures:");
    for cap in re.captures_iter(text) {
        println!("Found: {:?}", &cap.get(0).unwrap().as_str());
    }
}
```
- `.` the dot in regex will match a single character, including letters and digits. Dots should not be used inside character classes.
- `*` the star in regex will match zero or more occurrences of the preceding character. It is a quantifier.
- `+` the plus in regex will match one or more occurrences of the preceding character. It is a quantifier.
- We can use `regex` to match spellings where there are multiple spellings can be correct, such as gray/grey, color/colour etc.
- `let re = Regex::new(r"gr[ae]y").unwrap();` will match both `gray` and `grey`.
=======================================================
### Character Ranges
=======================================================
- Character ranges are used to check if character in certain range is part of text or not.
- We can use `-` to specify a range of characters.
- For instance, for all lowercase characters, we can use `[a-z]`, for all uppercase characters, we can use `[A-Z]`, for all digits, we can use `[0-9]`.
- We can use multiple ranges inside a character class.
- We can also use `^` inside a character class to negate the character class.
- Some shorthands for frequently used character classes
    - `\d` for `[0-9]`
    - `\D` for `[^0-9]`
    - `\w` for `[a-zA-Z0-9_]`
    - `\W` for `[^a-zA-Z0-9_]`
    - `\s` for space
- `^` outside a character class is used to match the start of a line. Inside the character class it has meaning of negation.
- `$` is used to match the end of a line.
=======================================================
### Word Boundaries
=======================================================
- `\b` is used to match word boundaries. It is a zero-width assertion. It is a meta-character. It will match at three places,
    - Before the start in the input text, before any input character
    - At the end of the input text after there is no character and 
    - Between two characters if first is a word character and second is not a word character.
- It doesn't match any character, it matches a position.
-------------------------------------------------------
## Repetition, Quantifiers and Capture Groups
-------------------------------------------------------
### Quantifiers
=======================================================
- **Quantifiers** are used to denote the repetitions. There are three quantifiers that are commonly used
    - `*` - Zero or more occurrences
    - `+` - One or more occurrences
    - `?` - Zero or one occurrence
```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"a?aa").unwrap();

    let text = "aa aaa";
    for cap in re.captures_iter(text) {
        println!("Found: {:?}", &cap[0]);
    }
}
```
- In the above code, `a?aa` will match `aa` and `aaa`. It matches `aa` because `a` is optional, and it matches `aaa` because `a` is present.
- Another very good use case is to filter out file of particular types.
```rust
use regex::Regex;

fn main() {
    println!();
    println!(r"Use of quantifier to get file of particular type. For the regex pattern '\w?\w?\w?.rs' and text 'file1.rs file2.txt file3.rs':");
    let re = Regex::new(r"\w?\w?\w?.rs").unwrap();
    let text = "fil.rs t1.rs file.rs";
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}
```
- In this case we are looking for `.rs` files, which can have minimum 0 and maximum 3 characters before `.rs`.
- **Use of `+` Quantifier** - We can use `+` quantifier to match one or more occurrences of the preceding character.
```rust
use regex::Regex;

fn main() {
     println!();
    println!("Use of '+' quantifier.");

    println!("For the regex pattern 'a+' and text 'a aa aaa baab bab':");
    let re = Regex::new(r"a+").unwrap();
    let text = "a aa aaa baab bab";
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}
```
- In this case, `a+` will match `a`, `aa`, `aaa` and `aa` and `a` from the text.
- **Use of `*` Quantifier** - We can use `*` quantifier to match zero or more occurrences of the preceding character.
```rust
use regex::Regex;

fn main() {
    println!();
    println!("Use of '*' quantifier.");
    let re = Regex::new(r"ab*").unwrap();
    let text = "a ab abbbbb";
    println!("For the regex pattern 'ab*' and text 'a ab abbbbb':");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}
```
- In this case, `ab*` will match `a`, `ab` and `abbbbb` from the text.

- We can use `{}` to specify the number of occurrences, if we have some limited number of occurrences.
```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"\b\w{3,5}\b").unwrap();
    let text = "Hello I think you are because I have a gift for you.";
    println!(r"For the regex pattern '\w{{3,5}}' and text 'Hello I think you are because I have a gift for you.'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}
```
- In this case, `\b\w{3,5}\b` will match any word character that has minimum 3 and maximum 5 occurrences, and `\b` is used to match word boundaries, this will make sure no partial word is matched.
- A Nice use case of limited repetition is to limit the number of digits in the fraction and whole number part of a number.
```rust
use regex::Regex;

fn main() {
        println!();
    println!("Making sure that the number of digits in the whole and fraction parts are between 1 to 3 digits with a dot in between.");
    let re = Regex::new(r"\b\d{1,3}\.\d{1,3}\b").unwrap();
    let text = "921.583 0.0 1456.25";
    println!(r"For the regex pattern '\b\d{{1,3}}\.\d{{1,3}}\b' and text '921.583 0.0 1456.25'");
    for cap in re.captures_iter(text) {
        println!("Found match: {}", &cap[0]);
    }
}
```
- In this case, `\b\d{1,3}\.\d{1,3}\b` will match any number that has 1 to 3 digits before and after the dot.

===========================================================
### Capture Groups
===========================================================
- **Capture Groups** are used to capture the groups in the pattern. We can use `()` to create a capture group.
- Sometimes the pattern we want to define may be a bit more complex and therefore we may want to break it down into smaller parts.
- This can be done by placing parts of a regular expression inside parentheses `()` to create groups or parts withing a regular expression pattern.
- Consider a case for detecting the dates which are given in the form of year, month and day. The year is in the form of four digits, followed by a month containing two digits and lastly the day containing two digits.
- We can write each part of the date as a separate group and then combine them to form the date.
```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2021-07-23 2021-08-01 2021-09-30";
    println!(r"For the regex pattern '(\d{{4}})-(\d{{2}})-(\d{{2}})' and text '2021-07-23 2021-08-01 2021-09-30'");
    for cap in re.captures_iter(text) {
        println!("Month: {}, Day: {}, Year: {}, and whole match: {}", &cap[2], &cap[3], &cap[1], &cap[0]);
    }
}
```
- In this case, `(\d{4})-(\d{2})-(\d{2})` will match any date that has year in the form of 4 digits, month in the form of 2 digits and day in the form of 2 digits.
- We can use `&cap[0]` to get the whole match, and `&cap[1]`, `&cap[2]` and `&cap[3]` to get the year, month and day respectively, from the text.
- `&cap[i]` is used to get the `i-th` capture group from the text.
-------------------------------------------------------
# String Literals
-------------------------------------------------------
- Sometimes you may encounter many double quotes inside a string. In such cases, to make the `"` part of string, we can use `\"` to escape the double quote.
- If we have many quotes and double quotes in a string, it becomes annoying and error-prone to escape all the quotes. In such cases, we can use `r#` before the string to make it a raw string.
```rust
fn main() {
    let s = r#"This is a "raw" string"#;
    println!("{}", s);
}
```
- In this case, `r` is used to indicate that the string should be treated as a **raw string**. `#` at beginning and end of the string is used to indicate the start and end of the raw string.
- If the string doesn't contain `"` then we can skip `#` at the beginning and end of the string.
- One of the the major use case in which string literals are useful is when we are processing a JSON string.
```rust
fn main() {
        println!();
    println!("Use Case: JSON String");
    let json_str = "{
        \"name\": \"John Doe\",
        \"age\": 30,
        \"city\": \"New York\"
    }";
    println!("Using escaped string literals: {}", json_str);
    let json_str = r#"{
        "name": "John Doe",
        "age": 30,
        "city": "New York"
    }"#;
    println!("Using raw string literals: {}", json_str);
}
```
- In this case, we have a JSON string, which contains many double quotes. We can use `\"` to escape the double quotes, or we can use `r#` to make it a raw string. Raw string is more readable and less error-prone.
- **Thing to note**: If we want to use `#` in the string, then we can use `#` multiple times to indicate the start and end of the raw string.
```rust
fn main() {
    let s = r###"This is a #"raw" string"###;
    println!("{}", s);
}
```
- Basically you have to use one more `#` than the number of continuous `#` you want to use in the string as start and end of the raw string.
- Summary of String Literals
    - `r` is used to indicate that the string should be treated as a raw string.
    - `#` at beginning and end of the string is used to indicate the start and end of the raw string.
    - If the string doesn't contain `"` then we can skip `#` at the beginning and end of the string.
    - If we want to use `#` in the string, then we can use `#` multiple times to indicate the start and end of the raw string.
    - If we have n-1 continuous `#` in the string, then we have to use n `#` at the beginning and end of the string.
-------------------------------------------------------
# String Concatenation and Ownership
-------------------------------------------------------
- `+` operator is used to concatenate two strings. It can only be used to concatenate a String with a string slice `&str`.
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = " World!";
    let s3 = s1 + s2;
    println!("{}", s3);
}
```
- `+` operator will first concatenate the two strings together as expected. The result of concatenation will not be stored in a separate memory location, but it will be stored at the location in the heap that is being pointed to by the variable `s1.`
- Basically, ownership of `s1` is moved to `s3` and `s1` is no longer valid.
=======================================================
## Concatenating Pair of Strings
=======================================================
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2;
```
- In this case, `s1` is moved to `s3` and `s1` is no longer valid.
- `&s2` is used to convert `s2` to a string slice `&str`.
- Here `deref coercion` is used to convert `&String` to `&str`.
- Ownership of `s2` will not change.

=======================================================
## Concatenating Multiple Strings
=======================================================
```rust
fn main() {
    println!()
    println!("Concatenating multiple strings");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
}
```
- Concatenation result is first stored in `s1` as always, and then `s1` is moved to `s4`.
