fn main() {
    const SUM: i32 = 1000;
    let mut a = 1;
    let mut b = 2;
    let mut c: i32 = SUM - a - b;

    while a < b && b < c {
        if a * a + b * b == c * c {
            println!("a: {}, b: {}, c: {}", a, b, c);
            println!("Sum: {}", a + b + c);
            break;
        }

        if c - b > 2 {
            b += 1;
            c -= 1;
        } else {
            a += 1;
            b = a + 1;
            c = SUM - a - b;
        }
    }
}
