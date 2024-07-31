// ----------------------------------------------------------------------------
//              Generic Lifetimes
// ----------------------------------------------------------------------------

pub fn main() {
    let int1 = 5;
    let picked_value;
    {
        let int2 = 10;
        picked_value = picking_int(&int1, &int2);
        println!("Picked value: {}", picked_value);
    }
    // println!("Picked value: {}", picked_value); We start getting an error here, because the value of int2 is not valid anymore

    println!("Returning a created value from a function using static lifetime: {}", get_static_int());
}

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}

fn get_static_int() -> &'static i32 {
    let new_y: &'static i32 = &10;
    new_y
}