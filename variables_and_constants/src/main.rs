// ------------------------------------------------------------------
//              - Variables
//                   - Mutability
//                   - Definition
//                   - Scope
//                   - Shadowing
//              - Constants
// ------------------------------------------------------------------

fn main() {
    // Definition
    let x: i16 = 10;
    println!("x is {x}");

    // Mutability
    let mut _y = 5;
    _y = 10;

    // Scope
    {
        let _z = 50;
    }
    // let s = z; // Error: z is not defined in this scope

    // Shadowing
    let t: i32 = 10;
    let t: i32 = t + 10;
    println!("t is {t}");

    // Use of shadowing, sometime we may want to change the type of a variable.
    let _u: i32 = 3;
    let _u = 3.0;

    // Useful when a variable needs to be used in inner scope with some other
    // context
    let v = 30;
    {
        let v = 40;
        println!("v is {v}");
    }
    println!("v is {v}");

    // Constants
    const MAX_VALUE: u32 = 100;
    println!("MAX_VALUE is {MAX_VALUE}");
}
