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
## 