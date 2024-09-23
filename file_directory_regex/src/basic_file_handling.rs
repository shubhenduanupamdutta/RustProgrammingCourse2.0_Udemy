//----------------------------------------------------------------
//           Basic File Handling
//----------------------------------------------------------------

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
