//----------------------------------------------------------------
//           Performance Lints
//----------------------------------------------------------------
struct _A {
    values: Box<Vec<i32>>,
}

pub fn main() {
    let _x: Box<i32> = Box::new(Default::default());

    let _x: Box<i32> = Box::default();

    let _y = String::from("Nouman");
    let _z = "Nouman";
    if _y == _z.to_owned() {
        println!("Equal");
    }

    println!("Above can be written as:");
    if _y == _z {
        println!("Equal");
    }

    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];
    a.extend(b.drain(..));
}   