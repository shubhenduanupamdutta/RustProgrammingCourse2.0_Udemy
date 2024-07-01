// ----------------------------------------------------------------------
//              - Scalar Data Types
//                   - Integers
//                   - Floating-Point Numbers
//                   - Booleans
//                   - Chars
// ----------------------------------------------------------------------

fn main() {
    // Unsigned Integers
    let unsigned_num: u8 = 5;   // u16, u32, u64, u128, usize
    println!("Unsigned Integer: {}", unsigned_num);

    // Signed Integers
    let signed_num: i8 = -5;    // i16, i32, i64, i128, isize
    println!("Signed Integer: {}", signed_num);

    // Floating point numbers
    let float_num: f32 = 5.0;   // f64
    println!("Floating Point Number: {}", float_num);

    // isize and usize depends on Architecture of machine and guarantees to be same size as pointer
    let architecture: usize = 5;
    let arch_2: isize = -5;
    println!("Architecture: {}, Architecture 2: {}", architecture, arch_2);
    
    // Characters
    let char_data: char = 'A';
    println!("Character: {char_data}");

    // Booleans
    let bool_data: bool = true;
    println!("Boolean: {bool_data}");

    // Type Aliasing
    type Age = u8;
    let peter_age: Age = 42;
    println!("Peter's Age: {}", peter_age);

    // Type conversion
    let a: f32 = 5.0;
    let b: i64 = a as i64;
    println!("Type Conversion: {}", b);
}