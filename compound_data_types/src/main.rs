// ---------------------------------------------------------------------
//                  - Compound Data Types
//                         - Tuples
//                         - &str and String
//                         - Arrays
//                         - Vectors
//                         - Empty Tables
// ---------------------------------------------------------------------

fn main() {
    // &str and String
    let fixed_str: &str = "Hello World";
    // &str is called string sliced and fixed length
    let mut flexible_str: String = String::from("This string can grow.");
    flexible_str.push('s');
    // String is a growable string, and comes from the standard library
    println!("Fixed String: {fixed_str}, Flexible String: {flexible_str}");

    // Difference between &str and String is that &str is immutable, that is
    // we can't remove or add characters to it, but String is mutable, that is
    // we can remove or add characters to it.
    // Typically &str (string slice) will be used to read some read only data.

    // Arrays
    let mut array_1: [i32; 5] = [1, 2, 3, 4, 5];
    // Size of array is fixed, and we can't add or remove elements from it.
    let num = array_1[3];
    println!("Number at index 3: {num}");
    println!("Array: {array_1:?}");
    // Changing array element
    array_1[3] = 10;
    println!("Array: {array_1:?}");
    // We can initialize array with same value
    let array_2: [i32; 5] = [0; 5]; // All values are 0
    println!("Array: {array_2:?}");

    // Vectors
    // Contrast to array, shrink and grow in size
    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let num = vec_1[3];
    println!("Number at index 3: {num}");

    // Tuples
    // In contrast to arrays, tuples can hold values of different types.
    let my_info = ("Salary", 40000, "Age", 40);
    let salary_value = my_info.1;
    println!("Salary: {salary_value}");
    // Destructuring tuple
    let (_salary, salary_value, _age, age_value) = my_info;
    println!("Salary: {salary_value}, Age: {age_value}");

    // Empty Tuples or Unit
    // It is a tuple with no elements, and is used to represent absence of value.
    let unit: () = (); // It is used to represent absence of value. 0 size
    println!("Unit: {unit:?}");
}
