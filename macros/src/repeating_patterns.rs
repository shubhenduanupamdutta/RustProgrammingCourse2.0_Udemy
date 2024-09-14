//------------------------------------------------------------------
//          Repeating Patterns
//------------------------------------------------------------------

macro_rules! string_concat {
    () => { String::new() };
    // ($some_str: expr) => {{
    //     let mut s = String::new();
    //     s.push_str($some_str);
    //     s
    // }};  // Last match can be used for above two matches
    ($($some_str: expr), +) => {{
        let mut s = String::new();
        $(
            s.push_str($some_str);
        )+
        s
    }};
}

// Another example with a vector
macro_rules! vec_mac {
    ($($element: expr), *) => {{
        let mut some_vec = Vec::new();
        $(
            some_vec.push($element);
        )*
        some_vec
    }}
}

pub fn main() {
    let null_str = string_concat!();
    println!("Null String: {}", null_str);

    println!();
    println!("Using one string:");
    let some_str = string_concat!("Hello, World!");
    println!("Some String: {}", some_str);

    println!();
    println!("Using multiple strings:");
    let multiple_str = string_concat!("Hello, ", "World", "!");
    println!("Multiple Strings: {}", multiple_str);

    println!();
    println!("Using vec_mac macro:");
    let some_vec = vec_mac!(1, 2, 3, 4, 5);
    println!("Some Vector: {:?}", some_vec);
}
