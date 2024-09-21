//----------------------------------------------------------------
//        Efficient Storage adn Retrieval of Words
//----------------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            is_word: false,
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &String) {
        let mut current = &mut self.root;
        for w in word.chars() {
            current = current.children.entry(w).or_insert(Node::new())
        }

        if !current.is_word {
            current.is_word = true;
        }
    }

    fn search(&self, word: &String) -> bool {
        let mut current = &self.root;
        for w in word.chars() {
            if current.children.get(&w).is_some() {
                current = current.children.get(&w).unwrap();
            } else {
                return false;
            }
        }

        current.is_word
    }
}

pub fn main() {
    let words = vec![
        "the", "a", "there", "answer", "any", "by", "bye", "their", "abc", "abstract",
    ];
    let words = words
        .into_iter()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();

    let mut d = WordDictionary::new();
    for word in words {
        d.insert(&word);
    }

    println!("Searching `there` in the dictionary result: {}", d.search(&"there".to_string()));
    println!("Searching `them` in the dictionary result: {}", d.search(&"them".to_string()));
    println!("Searching `ther` in the dictionary result: {}", d.search(&"ther".to_string()));
}
