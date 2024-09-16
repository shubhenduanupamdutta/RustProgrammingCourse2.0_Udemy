# Real Life Problems in Rust
---------------------------------------------------------
## Problem 1: Correct Search Result Using Word Grouping
---------------------------------------------------------
### Problem Statement - *Given a list of words, group the words that are anagrams of each other. An anagram is a word formed by rearranging the letters of another, like "cinema", formed from "iceman".*
### Data Structures and Patterns Used - *HashMap, Nested Loops*
=========================================================
- #### Real Life Scenario
    - *In a search engine, when a user searches for a word, the search engine should return all the words that are anagrams of the searched word. For example, if the user searches for "cinema", the search engine should return "iceman" as well.*
    - *Consider an online store, where store has many items to sell which are searchable. Store wants a functionality which when a user misspells a word, they still get the correct or possible correct search results. Business considers this function to be important from the user satisfaction and usability perspective. The solution to this will include to first split the words describing the items that are present in the store into sets so that all words in a set are anagrams*

- #### Implementation Detail
    - **Assumption** - We are given a list of words describing some items and we will need to group the words that are anagrams of each other.
    - **Approach** - We will be using hash maps and loops to implement the solution.
```rust
use std::collections::HashMap;

pub fn main() {
    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "diputs".to_string(),
        "pudits".to_string(),
        "hello".to_string(),
        "world".to_string(),
        "dlrow".to_string(),
        "olleh".to_string(),
        "apple".to_string(),
    ];

    let grouping = word_grouping(words);

    let input_word = String::from("teh");
    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The word group containing the word '{}' is {:?}", input_word, i);
        }
    }
}

fn word_grouping(words: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash = HashMap::new();
    let mut char_freq = vec![0; 26];
    for current_word in words {
        for c in current_word.to_lowercase().chars() {
            if !c.is_alphabetic() {
                continue;
            }

            char_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }
        let key = char_freq.iter().map(|x| x.to_string()).collect::<String>();
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);
        char_freq = vec![0; 26];
    }

    for (key, value) in &word_hash {
        println!("key # {:?}, values # {:?}", key, value);
    }

    word_hash.into_iter().map(|(_, value)| value).collect()
}
```
- #### Output
```shell
key # "00001001000200100000000000", values # ["hello", "olleh"]
key # "10001000000100020000000000", values # ["apple"]
key # "00010000100000010011100000", values # ["stupid", "diputs", "pudits"]
key # "00001001000000000001000000", values # ["the", "teh", "het"]
key # "00010000000100100100001000", values # ["world", "dlrow"]
The word group containing the word 'teh' is ["the", "teh", "het"]
```
- #### Explanation
    - We have used a hash map to group the words that are anagrams of each other. We have used a vector to store the frequency of each character in a word. We have then used this vector to create a key for the hash map. We have then grouped the words based on the key. Finally, we have printed the word group containing the input word "teh".
---------------------------------------------------------
## Product Popularity Using HashMaps
---------------------------------------------------------
### Problem Statement - *Given a list of products and their popularity, find the most popular product.*
### Data Structures and Patterns Used - *HashMap, Loops, Conditional Statements*
=========================================================
- #### Real Life Scenario
    - *A business has many products to offer. For each product the business has collected some information which basically shows the popularity of the product. The business wants to know which product is the most popular among all the products. This popularity score is derived from customer feedback, likes, dislikes, reviews, etc. The scores are updated weekly and added to the end of list containing previous scores. The business wants to know if the popularity of a product is fluctuating, increasing or decreasing over time. Moreover, the would like to identify and separate a product if it is gaining or losing popularity.*

- #### Implementation Detail
    - **Assumption** - We are given a list of products and their popularity scores. Company uses HashMaps to effectively store the product and their respective popularity scores. Key are the product names and values is a vector containing popularity scores.
```rust
use std::collections::HashMap;

fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        } else if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }

    return increasing || decreasing;
}


pub fn main() {
    let mut products = HashMap::new();

    products.insert("product_1", vec![1, 2, 2, 3]);
    products.insert("product_2", vec![4, 5, 6, 3, 4]);
    products.insert("product_3", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    for (product_id, popularity) in products {
        if popularity_analysis(popularity) {
            println!("{} popularity is increasing or decreasing", product_id);
        } else {
            println!("{} popularity is fluctuating", product_id);
        }
    }
}
```
- #### Output
```shell
product_1 popularity is increasing or decreasing
product_2 popularity is fluctuating
product_3 popularity is increasing or decreasing
```
- #### Explanation
    - We have used a hash map to store the products and their popularity scores. We have then used a function to analyze the popularity scores. We have then printed the popularity analysis of each product.
    - The `popularity_analysis` function checks if the popularity scores are increasing or decreasing. If the scores are increasing or decreasing, the function returns true. Otherwise, it returns false. In detail, the function checks if the popularity scores are increasing or decreasing by comparing the current score with the next score. If the current score is greater than the next score, the scores are decreasing. If the current score is less than the next score, the scores are increasing. If the scores are neither increasing nor decreasing, the scores are fluctuating.
---------------------------------------------------------
## Highest Price Stock using MaxStack and Structures
---------------------------------------------------------
### Problem Statement - *Given a list of stocks and their week-wise prices, find the stock with the highest price in any week in little to no time.*
### Data Structures and Patterns Used - *MaxStack, Structures, Vectors*
=========================================================
- #### Real Life Scenario
    - *A business has many stocks to offer. For each stock the business has collected some information which basically shows the price of the stock for each week. They are interested in a feature, which grabs the highest value the stock may have reached at any point in time. They may also want to go back sequentially week wise and determine the highest value of stocks. Since there are lots of data and many different kinds of stocks, the business wants to have a solution that is efficient and fast.*
- #### Implementation Detail
    - **Assumption** - We are given a list of stocks and their week-wise prices. We will be using a max stack to find the stock with the highest price in any week.
    `MaxStack` is simple stack, but has information of the maximum element in the stack at all times. And this information can be obtained in little to no time. In `O(1)` time complexity.

    - `MaxStack` can be implemented in different ways, but we will implement it using structure containing two stacks. Two stacks will be referred to as the main stack and the max stack. The main stack will store the elements and the max stack will store the maximum element at the top of the stack.
```rust
struct MaxStack {
    main_stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack {
            main_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.main_stack.push(value);
        if !self.max_stack.is_empty() && (self.max_stack.last().unwrap() > &value) {
            self.max_stack.push(*self.max_stack.last().unwrap());
        } else {
            self.max_stack.push(value);
        }
    }

    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();
    }

    fn max_value(&self) -> i32 {
        *self.max_stack.last().unwrap_or(&0)
    }
}

pub fn main() {
    let mut stack = MaxStack::new();
    stack.push(55);
    stack.push(80);
    stack.push(120);
    stack.push(99);
    stack.push(22);
    stack.push(140);
    stack.push(145);

    println!("Maximum value of stock: {}", stack.max_value());

    println!("After going back one week");
    stack.pop();
    println!("Maximum value of stock: {}", stack.max_value());
}
```
- #### Output
```shell
Maximum value of stock: 145
After going back one week
Maximum value of stock: 140
```
- #### Explanation
    - We have implemented a max stack to find the stock with the highest price in any week. We have then pushed the stock prices into the stack. We have then printed the maximum value of the stock. We have then popped the stock price from the stack and printed the maximum value of the stock.
---------------------------------------------------------