// ----------------------------------------------------
//              - Loops
//              - For loop
//              - While loop
// ----------------------------------------------------

fn main() {
    'outer: loop {
        println!("Simple loop!");
        break 'outer;
    }

    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", _result);

    let vec = vec![45, 20, 85 , 67, 34];

    for i in vec {
        println!("Value: {}", i);
    }

    let mut num: i32 = 0;
    while num < 10 {
        println!("Value: {}", num);
        num += 1;
    }
}