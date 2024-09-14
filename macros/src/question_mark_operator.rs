//--------------------------------------------------------
//       Questin Mark Operator
//--------------------------------------------------------

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let integer = input.parse::<i32>()?;
    println!("The value of integer is: {}", integer);
    Ok(integer)
}

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        println!("The division of {} by {} is: {}", x, y, x / y);
        Ok(())
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        println!("The square root of {} is: {}", x, x.sqrt());
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        println!("The natural logarithm of {} is: {}", x, x.ln());
        Ok(())
    }
}

fn operations(x: f64, y: f64) -> MathResult {
    div(x, y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

pub fn main() {
    parse_str("10").unwrap();

    let values = vec!["123", "some1", "some(123)", "abc", "53"];

    for value in values {
        println!("Result: {:?}", parse_str(value));
    }

    println!("\n\n");
    println!("Example of divide function");

        println!(
        "Call from main with result equals to {:?}",
        divide(10.0, 2.0)
    );
    println!();
    println!(
        "Call from main with result equals to {:?}",
        divide(10.0, 0.0)
    );
    println!();
    println!(
        "Call from main with result equals to {:?}",
        divide(0.0, 2.0)
    );

    println!("\n\n");
    println!("Use case of `?`, checking if inputs are valid for operations");
    let result = operations(0.0, 10.0);
    if result.is_ok() {
        println!("All operations were successful");
    } else {
        println!("An error occurred, {:?}", result);
    };
}

fn divide(divident: f64, divisor: f64) -> Result<f64, String> {
    let answer = match divisor {
        0.0 => Err("Cannot divide by zero".to_string()),
        _ => Ok(divident / divisor),
    };

    let correct = answer?;
    println!("This line will not be printed if there is an error");
    println!("The answer is: {}", correct);
    Ok(correct)
}
