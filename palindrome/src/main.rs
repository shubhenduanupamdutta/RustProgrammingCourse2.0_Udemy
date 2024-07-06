fn main() {
    let input = String::from("1223");
    println!("It is {:?} that given string is palindrome", palindrome(input));
}

fn is_palindrome(input: String) -> bool {
    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    }
    input == reversed
}

fn palindrome(input: String) -> bool {
    let mut is_palindrome = true;
    if input.len() == 0 {is_palindrome = true;}
    else {
        let mut last = input.len() - 1;
        let mut first: usize = 0;

        let my_vec = input.as_bytes();

        while first < last {
            if my_vec[first] != my_vec[last] {
                is_palindrome = false;
                break;
            }
            first += 1;
            last -= 1;
        }
    }

    is_palindrome
}