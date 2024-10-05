//-----------------------------------------------------------------------
//            String Concatenation and Ownership
//-----------------------------------------------------------------------

pub fn main() {
    let s1 = String::from("Hello, ");
    let s2 = "world!";

    let s3 = s1 + s2;
    println!("{}", s3);
    // println!("{}", s1); // This will throw an error because s1 has been moved to s3

    println!();
    println!("Concatenating couple strings");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);

    println!();
    println!("Concatenating multiple strings");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;

    println!("s4: {}, s3: {}, s2: {}", s4, s3, s2);

}