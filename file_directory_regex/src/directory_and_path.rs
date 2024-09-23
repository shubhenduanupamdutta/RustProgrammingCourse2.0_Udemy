//----------------------------------------------------------------------
//       Directory and Path Related Functions
//----------------------------------------------------------------------
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn main() {
    path_related_functions();
    directory_related_functions();
    removing_file_and_directories();
}

fn path_related_functions() {
    println!();
    println!("======= Path Related Functions =======");
    let path = Path::new(r"D:\PersonalProjects\Examples\my_text.txt");

    println!();
    println!("Getting File Metadata");
    // Getting parent directory
    println!(
        "Parent directory of {:?} is {:?}",
        path,
        path.parent().unwrap()
    );

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
    let paths = [
        r"D:\PersonalProjects\Examples\NewDir",
        r"Rust",
        r"Examples",
        r"MyFile.txt",
    ];

    let new_path = paths.into_iter().collect::<PathBuf>();
    println!("Full File Path created: {:?}", new_path);

    // Checking if a file or directory exists
    println!();
    println!("Checking if a file or directory exists");
    println!("Checking if {:?} exists: {:?}", dir_path, dir_path.exists());

    println!("Checking if {:?} exists: {:?}", path, path.exists());

    println!(
        "Checking if {:?} is a file: {:?}",
        dir_path,
        dir_path.is_file()
    );

    println!("Checking if {:?} is a file: {:?}", path, path.is_file());

    println!(
        "Checking if {:?} is a directory: {:?}",
        dir_path,
        dir_path.is_dir()
    );

    println!(
        "Checking if {:?} is a directory: {:?}",
        path.parent().unwrap(),
        path.parent().unwrap().is_dir()
    );

    // Getting metadata
    println!();
    println!("Getting metadata of file and directory");
    let file_path = path;
    let dir_path = path.parent().unwrap();

    println!(
        "Metadata of {:?} is {:#?}",
        file_path,
        file_path.metadata().unwrap()
    );
    println!(
        "Metadata of {:?} is {:#?}",
        dir_path,
        dir_path.metadata().unwrap()
    );

    // Getting metadata attributes
    let file_metadata = file_path.metadata().unwrap();
    println!("Type of {:?} is {:?}", file_path, file_metadata.file_type());
    println!("Length of {:?} is {:?}", file_path, file_metadata.len());
    println!(
        "Permissions of {:?} are {:?}",
        file_path,
        file_metadata.permissions()
    );
    println!(
        "Created time of {:?} is {:?}",
        file_path,
        file_metadata.created().unwrap()
    );
    println!(
        "Modified time of {:?} is {:?}",
        file_path,
        file_metadata.modified().unwrap()
    );
}


fn directory_related_functions() {
    println!();
    println!("======= Directory Related Functions =======");

    println!();
    println!("Getting all sub-directories in a directory");
    let path = Path::new(r"D:\PersonalProjects");
    for files in path.read_dir().expect("Unable to read directory") {
        println!("{:?}", files.unwrap().path());
    }

    println!();
    println!("Getting current directory");
    let current_dir = env::current_dir().unwrap();
    println!("Current directory is {:?}", current_dir);

    let mut path = PathBuf::from(r"D:\PersonalProjects\Examples\");
    path.push("NewDir");
    println!("Create a new directory: {:?}", fs::create_dir(&path).unwrap());
    path.push("level1");
    path.push("level2");
    println!("Create many directory: {:?}", fs::create_dir_all(path).unwrap());
}

fn removing_file_and_directories() {
    println!();
    println!("======= Removing Files and Directories =======");
    println!("Removing a specific directory");
    let path = Path::new(r"D:\PersonalProjects\Examples\NewDir\level1\level2");
    println!("Removing directory {:?}: {:?}", path, fs::remove_dir(path));

    println!();
    println!("Removing directory and its contents");
    let path = Path::new(r"D:\PersonalProjects\Examples\NewDir");
    println!("Removing directory {:?}: {:?}", path, fs::remove_dir_all(path));

    println!();
    println!("Renaming a file");
    let path = Path::new(r"D:\PersonalProjects\Examples\my_text.txt");
    let new_path = Path::new(r"D:\PersonalProjects\Examples\my_text_renamed.txt");
    println!("Renaming file {:?} to {:?}: {:?}", path, new_path, fs::rename(path, new_path));

    println!();
    println!("Copying a file");
    let path = Path::new(r"D:\PersonalProjects\Examples\my_text_renamed2.txt");
    println!("Copying file {:?} to {:?}: {:?}", new_path, path, fs::copy(new_path, path));
    
    println!();
    println!("Removing a file");
    println!("Removing file {:?}: {:?}", new_path, fs::remove_file(new_path));
}