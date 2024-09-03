//----------------------------------------------------------------
//         Practice on Threads Basics
//----------------------------------------------------------------

use std::thread;

pub fn main() {
    let handle_1 = thread::spawn(|| {
        let mut sum = 0;
        let range = 0..=1000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_2 = thread::spawn(|| {
        let mut sum = 0;
        let range = 1001..=2000;
        for num in range {
            sum += num;
        }
        sum
    });

    let handle_3 = thread::spawn(|| {
        let mut sum = 0;
        let range = 2001..=3000;
        for num in range {
            sum += num;
        }
        sum
    });

    let sum;
    sum = handle_1.join().unwrap() + handle_2.join().unwrap() + handle_3.join().unwrap();

    println!("Sum of 1 to 3000 is: {}", sum);
}