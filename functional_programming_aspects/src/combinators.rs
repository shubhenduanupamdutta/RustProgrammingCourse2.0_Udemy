// -----------------------------------------------------------
//            Combinators
// -----------------------------------------------------------

pub fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "watermelon", "pineapple", "mango", "strawberry", "beet", "blueberry", "cherry", "apricot"];

    let mut result: Vec<String> = vec![];

    for word in words {
        if word.starts_with("a") || word.starts_with("b") {
            let uppercase_word = word.to_uppercase();
            result.push(uppercase_word);
        }
    }

    println!("Result: {:?}", result);

    let fruits = vec!["apple", "banana", "grape", "orange", "watermelon", "pineapple", "mango", "strawberry", "beet", "blueberry", "cherry", "apricot", "avocado", "blackberry", "cantaloupe", "coconut", "fig", "kiwi", "lemon", "lime", "lychee", "nectarine", "papaya", "peach", "pear", "plum", "pomegranate", "raspberry", "tangerine", "tomato", "watermelon"];

    let result = fruits
    .into_iter()
    .filter(|&word| word.starts_with("a") || word.starts_with("b"))
    .map(|word| word.to_uppercase())
    .collect::<Vec<String>>();

    println!("Result: {:?}", result);
}