// -----------------------------------------------------------------------------
//             Iterating through collections
// -----------------------------------------------------------------------------

use std::collections::HashMap;

pub fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.iter();
    // let value_1 = vec_1_iter.next();
    // println!("Value 1: {:?}", value_1);

    for values in &vec_1 {
        println!("Value: {}", values);
    }

    println!("Iterating through mutable vector");
    for values in &mut vec_1 {
        *values += 50;
        println!("Value: {}", values);
    }

    println!("Iterating through via owning");
    let vec_2 = vec![45, 30, 85, 90, 41, 39];
    for values in vec_2{
        println!("Value: {}", values);
    }

    println!("Examples with HashMap");
    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("John".to_string(), 23);
    person.insert("Doe".to_string(), 24);
    person.insert("Smith".to_string(), 25);

    println!("Iterating through HashMap, borrowing immutable reference");
    for (name, age) in &person {
        println!("Name: {}, Age: {}", name, age);
    }

    println!("Iterating through HashMap, borrowing mutable reference");

    for (name, age) in &mut person {
        *age += 1;
        println!("Name: {}, Age: {}", name, age);
    }

    println!("Iterating through HashMap, owning");
    for (name, age) in person {
        println!("Name: {}, Age: {}", name, age);
    }
}