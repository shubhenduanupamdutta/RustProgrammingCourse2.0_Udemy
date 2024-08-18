use student::Student;

//--------------------------------------------------------------------------------------------------
//       Initializing Struct Instance
//--------------------------------------------------------------------------------------------------
pub mod student;

pub fn main() {
    // Throws an error since id is a private field
    // let student = student::Student {
    //     id: 1,
    //     age: 20,
    //     name: String::from("John Doe"),
    // };
    // println!("Student: {:?}", student);

    let student_1 = Student::new("Shubhendu".to_string());

    println!("Student 1: {:?}", student_1);

    // Returns default value
    let student_2 = Student::new("Shubhendu123".to_string()).unwrap_or_default();
    println!("Student 2: {:?}", student_2);

    // Returns student constructed
    let student_3 = Student::new("Shubhendu".to_string()).unwrap_or_default();
    println!("Student 3: {:?}", student_3);

    // Default implementation
    let student_4 = Student::default();
    println!("Student 4: {:?}", student_4);
}
