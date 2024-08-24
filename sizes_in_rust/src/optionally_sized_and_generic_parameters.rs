//--------------------------------------------------------------------------------------------------
//        - ?Sized and Generice Parameters
//--------------------------------------------------------------------------------------------------
use std::fmt::Debug;

struct UnSizedStruct<T: ?Sized> {
    _sized_field_1: i32,
    _unsized_field: T,
}

fn print_fn_1<T: Debug>(t: T) {
    println!("{:?}", t);
}

fn print_fn_2<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn print_fn_3<T: Debug + ?Sized>(t: &T) {
    println!("{:?}", t);
}


pub fn main() {
    let _x = UnSizedStruct {
        _sized_field_1: 10,
        _unsized_field: [1, 2, 3, 4, 5],
    };

    // Checking use case 2
    println!();
    let x = "My Name is Heisenberg";
    print_fn_1(x);

    // print_fn_2(x);  // This will not compile
    print_fn_2(&x); // Will pass &&str and thus compile and run

    print_fn_3(x); // Will pass &str and thus compile and run, since Sized is optional
    print_fn_3(&x); // Will pass &&str and thus compile and run
}