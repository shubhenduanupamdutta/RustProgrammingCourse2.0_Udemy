//----------------------------------------------------------------
//              Capturing Types
//----------------------------------------------------------------

macro_rules! input {
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read input.");

        let n: $t = n.trim().parse().expect("Invalid input");
        n
    }};
}

macro_rules! add_as {
    ($a: expr, $b: expr, $type: ty) => {
        $a as $type + $b as $type
    };
}

macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1;
    };
}

macro_rules! create_function {
    ($func_name: ident, $input: ident, $type: ty, $type_output: ty) => {

        fn $func_name($input: $type) -> $type_output {
            println!("You called {:?}() with the input of {:?}", stringify!($func_name), stringify!($input));
            $input as $type_output
        }
    }
}

create_function!(some_function, x, i32, f64);
create_function!(another_function, y, f64, i32);


pub fn main() {
    println!("Please enter a floating point number:");
    let some_input = input!(f64);
    println!("You entered: {}", some_input);

    println!();
    println!("Using add_as macro to add two numbers as f64:");
    let a: i8 = 5;
    let b: f32 = 10.5;

    let result = add_as!(a, b, f64);
    println!("Result: {}", result);

    println!();
    println!("Using identifiers in macros:");
    
    let mut x = 5;
    some_macro!(x);
    println!("Value of x after using some_macro: {}", x);

    println!();
    println!("Use function created using macro:");
    some_function(10);
    another_function(20.5);
}
