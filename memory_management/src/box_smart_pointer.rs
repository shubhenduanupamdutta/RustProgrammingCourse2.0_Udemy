// --------------------------------------------
//              Box Smart Pointer
// --------------------------------------------

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }


#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

#[derive(Debug)]
struct HugeData;

#[derive(Debug)]
struct SmallData;

trait Storage: std::fmt::Debug {}

impl Storage for HugeData {}
impl Storage for SmallData {}

pub fn main() {
    let x = 0.625;
    let y = Box::new(x);
    let z = &x;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!();

    // Using List
    // let list = List::Cons(
    //     1,
    //     Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    // );

    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None))))))
    );

    println!("list: {:?}", list);
    println!();
    let List::Cons(q, r) = list;
    println!("q: {}", q);
    println!("r: {:?}", r);

    println!("Another use case of Box Smart Pointer is to copy large amounts of data when transferring ownership.");
    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    let data_3 = data_1;
    let data_4 = data_2;

    let data_5 = Box::new(SmallData);

    // let data = vec![Box::new(data_3), data_4, data_5]; // Throws an error mismatched types
    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];

    println!("data: {:?}", data);
}
