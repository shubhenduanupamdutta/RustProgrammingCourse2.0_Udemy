// ----------------------------------------------------
//                   Ownership
// ----------------------------------------------------

/*
1.  Each value has a variable that’s called its owner.
2.  A value can have only one owner at a time.
3.  If the owner goes out of scope, the value will be dropped, deallocated, cleaned up, or whatever you want to call it.
 */

fn main() {
    let s1 = String::from("hello");
    println!("{}", s1);

    let s2 = s1;
    // println!("{}", s1); // This will throw an error because s1 is no longer valid
    println!("{}", s2);

    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);

}
