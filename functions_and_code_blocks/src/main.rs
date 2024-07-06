// --------------------------------------------------------------------------
//                  - Functions
//                 - Code Blocks
// --------------------------------------------------------------------------

fn main() {
    my_function("This is my function.");
    
    // Calling a function with multiple arguments and storting the result in a variable
    let answer = multiplication(5, 10);
    println!("The answer for multiplying 5 and 10 is {answer}");

    // Calling a function with multiple arguments and returning multiple values
    let (product, sum, difference) = basic_math(10, 5);
    println!("Product: {product}, Sum: {sum}, Difference: {difference}");

}

// Functions
fn my_function(s: &str) {
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication of {num1} and {num2}...");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1* num2, num1 + num2, num1 - num2)
}