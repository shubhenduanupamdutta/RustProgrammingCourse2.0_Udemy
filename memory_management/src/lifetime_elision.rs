// ----------------------------------------------------------------------------
//                    Lifetime Elision
// ----------------------------------------------------------------------------

pub fn main() {
    let str_1 = "some str";
    let str_2 = "some other str";
    let received_str = return_str(&str_1);
    println!("Received str: {}", received_str);

    let received_str_2 = return_str_2(&str_1, &str_2);
    println!("Received str_2: {}", received_str_2);
}

fn return_str(s_1: &str) -> &str {
    s_1
}

fn return_str_2<'a, 'b: 'a>(s_1: &'a str, s_2: &'b str) -> &'a str {
    if s_1.len() > s_2.len() {
        s_1
    } else {
        s_2
    }
}
