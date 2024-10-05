//----------------------------------------------------------------------
//          Function Inputs and Coercion
//----------------------------------------------------------------------

pub fn main() {
    let name = String::from("Shubhendu");
    println!("Voewls in the name: {} are {}", name, vowels(&name));

    let new_str = "Shubhendu";
    // println!("Voewls in the name: {} are {}", new_str, vowels(new_str)); // This doesn't compile because of type mismatch if vowels(words: &String) is used
    println!("Voewls in the name: {} are {}", new_str, vowels(new_str)); // This works because of type coercion when vowels(words: &str) is used

    println!();
    println!("===== Coercion from `&Box<T>` to `&T` =====");
    let x = Box::new("Shubhendu");
    println!("Calling length_str using Box<&str>");
    length_str(&x);

    println!();
    println!("Calling length_str using &str");
    // Direct call to length_str using &str
    length_str("Soma");

    println!();
    println!("===== Coercion from `&Vec<T>` to `&[T]` =====");
    let num_vec = vec![1, 2, 3, 4, 5];
    println!("Calling square_values using Vec<i32>");
    square_values(&num_vec);

    println!();
    println!("Calling square_values using &[i32]");
    // Direct call to square_values using &[i32]
    square_values(&[6, 7, 8, 9, 10]);
}

fn vowels(words: &str) -> u8 {
    words
        .chars()
        .into_iter()
        .filter(|&c| (c == 'a') | (c == 'e') | (c == 'i') | (c == 'o') | (c == 'u'))
        .count() as u8
}

/// Example of Coercion from `&Box<T>` to `&T`
fn length_str(x: &str) {
    println!("Length of the string: {} is {}", x, x.len());
}

/// Example of Coercion from `&Vec<T>` to `&[T]`
fn square_values(num_vec: &[i32]) {
    for i in num_vec {
        println!("Square of {} is {}", i, i * i);
    }
}
