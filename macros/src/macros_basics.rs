//----------------------------------------------------------
//               Declarative Macros
//                 - Basic Syntax
//----------------------------------------------------------
macro_rules! our_macro {
    () => {
        1 + 1
    };
    (something nonesens ladfja0oasdjf 0) => {
        println!("You found the nonsense!");
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
    ($e1: expr, $e2: expr, $e3: expr) => {
        $e1 * ($e2 + $e3)
    }
}

pub fn main() {
    println!("{}", our_macro!());
    our_macro!(something nonesens ladfja0oasdjf 0);
    println!("{}", our_macro!(1, 2));
    println!("{}", our_macro!(1, 2, 3));

    // println!("{}", our_macro!("Something", 2, "Nothing"));
    // Macros can be called with any type of brackets
    println!("Calling with different brackets");
    println!("{}", our_macro!{1, 2});
    println!("{}", our_macro![1, 2]);
    println!("{}", our_macro!{1, 2, 3});

    let _ = our_macro!(1, 2, 4);
}
