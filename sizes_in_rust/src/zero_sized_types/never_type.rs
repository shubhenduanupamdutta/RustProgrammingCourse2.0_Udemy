//----------------------------------------------------------------
//            Never Type
//----------------------------------------------------------------



fn _unrecoverable_state() -> ! {
    panic!("This function will never return a value");
}

pub fn main() {
    // unrecoverable_state();

    // let x = _unrecoverable_state();

    // let _x: !;

    let _x: i32 = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    println!("Value of x: {}", _x);

}