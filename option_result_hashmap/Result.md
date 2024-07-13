# Result
#### Syntax
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- Result enum has two variants
    - Ok: Contains a value of type T (a generic type)
    - Err: Contains a value of type E (a generic type)

#### Example
```rust
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

fn main() {
    let student_db = vec![
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
    ];

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
```
- In the above example, we don't have a clear mechanism for checking if the student exist in the database or not.
- In the above example, we return `None` if the student is not found in the database, which implicitly means student doesn't exist.
- But, in the context of the function `None` represents the absence of grade, **NOT** the absence of student.
- **More logical approach will be to check if the student exist in the database or not.**
- If the student exist, return the grade, else return an error.
- We can define a function check student.
- If the student exist, return `Ok(grade)`, else return `Err("Student not found")`.
```rust
fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

struct Student {
    name: String,
    grade: Option<u32>,
}


fn main() {
    let student_db = vec![
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
    ];

    let student_name = String::from("Alice");

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
```
- Updated code to check if the student exist in the database or not, and returning the grade if the student exist, and grade is available.