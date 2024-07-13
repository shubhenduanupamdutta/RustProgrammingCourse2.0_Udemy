# Option

```rust
struct Student {
    name: String,
    grade: u32,
}

fn option() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: 90,
        },
        Student {
            name: String::from("Bob"),
            grade: 80,
        },
    ];
    
}
```
<strong>
<ul>
<li> In the above case we have a struct of Students. </li>
<li> Imagine we are simulating a student database, containing some student records. </li>
<li> Each student in database is instance of Student struct. </li>
<li> We must provide both the name and grade of the student. </li>
<li> But it may happen, that grade may have not been finalized yet, so there would be no value for the grade. </li>
<li> Creating a default value is a bad idea, as it may lead to wrong interpretation of the data. </li>
<li> In such cases, we can use Option enum. </li>
<li> In some languages we can define a variable as null, but in Rust we can't. </li>
</ul>
</strong>

## Option Enum
```rust
enum Option<T> {
    Some(T),
    None,
}
```
<strong>
    <ul>
        <li> Option enum has two variants </li>
            <ol>
                <li> Some: Contains a value of type T (a generic type)</li>
                <li> None: Does not contain any value</li>
            </ol>
    </ul>
</strong>

## Using Option
```rust
struct Student {
    name: String,
    grade: Option<u32>,
}
```
- Now we can use Option<u32> for grade field, which means grade may or may not have a value.
- If it has some value we can pass it as `Some(grade_value)`, else we can pass `None`.

## Example
```rust
fn option() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Student {
            name: String::from("Bob"),
            grade: None,
        },
    ];
}
```

## Unwrapping Option
- We can unwrap the value of Option using `unwrap()` method.
- If the Option contains a value, it returns the value.
- If the Option is None, it panics.
- We can also use `match` to handle the Option.
```rust
fn option() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Student {
            name: String::from("Bob"),
            grade: None,
        },
    ];
    
    for student in student_db {
        match student.grade {
            Some(grade) => println!("{} scored {}", student.name, grade),
            None => println!("{} has no grade", student.name),
        }
    }
}
```
- We can also utilize `if let` to handle the Option.
```rust
fn option() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Student {
            name: String::from("Bob"),
            grade: None,
        },
    ];
    
    for student in student_db {
        if let Some(grade) = student.grade {
            println!("{} scored {}", student.name, grade);
        }
    }
}
```
- `if let` is a convenient way to handle the `Option`, when we are interested in only one variant of the `Option`.