//----------------------------------------------------------------------
//              - If else
//              - If else if ladder
//              - Match
//----------------------------------------------------------------------

fn main() {
    // If else
    let num = 40;
    if num < 50 {
        println!("Number is less than 50");
    } else {
        println!("Number is greater than 50");
    }

    // If else if ladder
    let marks = 95;
    let mut _grade = 'N';

    if marks >= 90 {
        _grade = 'A';
    } else if marks >= 80 {
        _grade = 'B';
    } else if marks >= 70 {
        _grade = 'C';
    } else {
        _grade = 'F';
    }

    println!("Grade is: {}", _grade);

    let marks = 95;

    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };

    println!("Grade is: {}", grade);

    // Redefining above grading as match
    let grade;
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }
    println!("Grade is: {}, calculated using match.", grade);
}
