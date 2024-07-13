use std::collections::HashMap;

fn main() {
    println!("#################### Option ####################");
    option();

    println!("\n#################### Result ####################");
    result();

    println!("\n#################### Hashmap ####################");
    hashmap();
}

// -----------------------------------------------------------------------
//                       Option
// -----------------------------------------------------------------------

// struct Student {
//     name: String,
//     grade: u32,
// }

#[derive(Debug)]
struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn get_student_db() -> Vec<Student> {
    vec![
        Student {
            name: String::from("Alice"),
            grade: Some(95),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ]
}

fn option() {
    let student_db = get_student_db();

    let student_name = String::from("Alice");
    let student_grade = get_grade(&student_name, &student_db);

    // match student_grade {
    //     Some(grade) => println!("{}'s grade is {}", student_name, grade),
    //     None => println!("{}'s grade is not available", student_name),
    // }

    // We can also utilized if let syntax, if you are dealing with only one value.
    if let Some(grade) = student_grade {
        println!("{}'s grade is {}", student_name, grade);
    }
}

// -----------------------------------------------------------------------
//                       Result
// -----------------------------------------------------------------------

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

fn result() {
    let student_db = get_student_db();
    println!("Current student db: {:#?}", student_db);

    let student_name = String::from("Bob");
    println!("Checking if {} is in the student db, and getting his/her grade.", student_name);

    match check_student(&student_name, &student_db) {
        Ok(grade) => {
            println!("{} is in the student db", student_name);
            match grade {
                Some(g) => println!("{}'s grade is {}", student_name, g),
                None => println!("{}'s grade is not available", student_name),
            }
        }
        Err(e) => println!("{}", e),
    }
}

// -----------------------------------------------------------------------
//                       Hashmap
// -----------------------------------------------------------------------

fn hashmap() {
    // Mapping name to age using HashMap
    let mut person: HashMap<&str, i32> = HashMap::new();

    person.insert("Alice", 20);
    person.insert("Bob", 25);
    person.insert("Shubhendu", 33);

    println!("Current person hashmap: {:#?}", person);

    // Accessing value from hashmap
    let name = "Alice";
    let age = person.get(name).unwrap();
    println!("{}'s age is {}", name, age);
    
    // Checking if a key exists in the hashmap
    let name = "Charlie";
    if person.contains_key(name) {
        println!("{} is in the person hashmap", name);
    } else {
        println!("{} is not in the person hashmap", name);
    }

    // We can also use match to handle the Option to identify if the key exists in the hashmap
    let name = "Bob";
    match person.get(name) {
        Some(age) => println!("{}'s age is {}", name, age),
        None => println!("{}'s age is not available, most likely key doesn't exist.", name),
    }

    for (name, age) in &person {
        println!("{} is {} years old", name, age);
    }

    let mut likes: HashMap<&str, &str> = HashMap::new();
    likes.insert("Alice", "Python");
    likes.insert("Bob", "Rust");
    likes.insert("Shubhendu", "Rust");

    // Let's updated the value of Bob
    likes.insert("Bob", "Python");
    println!("Current likes hashmap: {:#?}", likes);

    likes.entry("Shubhendu").or_insert("C++"); // This will not do anything as Shubhendu already exists
    likes.entry("Charlie").or_insert("Python"); // This will insert Charlie with Python as value

    println!("Current likes hashmap, after using entry on Shubhendu and Charlie: {:#?}", likes);

    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 7, 8];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("Frequency of each element in the vector: {:#?}", &freq_vec);
}