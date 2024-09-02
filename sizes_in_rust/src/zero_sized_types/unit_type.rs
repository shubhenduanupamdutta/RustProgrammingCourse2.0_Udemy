//--------------------------------------------------------------------------------------------------
//           Unit Type
//--------------------------------------------------------------------------------------------------
fn f1() {}
// de-sugared
// fn f1() -> () {}

fn _division_status(_dividend: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err("Error: Division by zero".to_string()),
        _ => {
            println!("The division is invalid.");
            Ok(())
        }
    };
    answer
}

pub fn main() {
    let _x: () = ();
    let _y = f1();

    let _result = _division_status(10.0, 0.0);
    println!("{:?}", _result);
    let _result = _division_status(10.0, 5.0);
    println!("{:?}", _result);

    let _z = println!("Hello, World!");

    let mut _vec: Vec<()> = Vec::with_capacity(0);
    _vec.push(());
    _vec.push(());
    _vec.push(());
    println!("{:?}", _vec);

    println!("Length of _vec: Vec<()> after three pushes: {}", _vec.len());
    assert_eq!(_vec.len(), 3);

    println!("Capacity of _vec: Vec<()> after three pushes: {}", _vec.capacity());

    println!("Maximum u64 value: {}", std::u64::MAX);
}