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
## Finding an Employee with No Meeting
---------------------------------------------------------
### Problem Statement - *Given a list of employees and their meeting schedules, determine overlapping time.*
### Data Structures and Patterns Used - *MultiDimensional Arrays, Nested Loops, Conditional Statements*
=========================================================
- #### Real Life Scenario
    - *Consider an office environment where there is a boss and he has two secretaries. He needs services of these secretaries from time to time. The secretaries are also involved in other activities and therefore has their own schedule of meetings. Some of the meetings of the secretaries may be overlapping. The boss wants to know the time slots in which both the secretaries may be busy, that is, both have meetings. This will help him in finding out a time slot in which at least one of the secretaries will be free so that some task may be assigned to them*
- #### Implementation Detail
    - **Assumption** - The meeting schedule for the two secretaries are given to us in the form of vectors. Each meeting is associated with the start and end time, which is also given in the form of vectors. Times are in 24 hours system for simplicity.
    - **Approach** - We will be using nested loops and conditional statements to implement the solution.
```rust
use std::cmp;

fn overlapping_meetings(meetings_a: Vec<Vec<i32>>, meetings_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut overlaps: Vec<Vec<i32>> = Vec::new();
    for i in 0..meetings_a.len() {
        for j in 0..meetings_b.len() {
            let (start_a, start_b) = (meetings_a[i][0], meetings_b[j][0]);
            let (end_a, end_b) = (meetings_a[i][1], meetings_b[j][1]);

            let overlap_status = overlap(start_a, start_b, end_a, end_b);
            if overlap_status.is_some() {
                overlaps.push(overlap_status.unwrap());
            }
        }
    }

    overlaps
}

fn overlap(start_a: i32, start_b: i32, end_a: i32, end_b: i32) -> Option<Vec<i32>> {
    let mut intersection_time = Vec::new();
    // Calculating overlaps
    if cmp::max(start_a, start_b) < cmp::min(end_a, end_b) {
        intersection_time.push(cmp::max(start_a, start_b));
        intersection_time.push(cmp::min(end_a, end_b));
        Some(intersection_time)
    } else {
        None
    }
}

pub fn main() {
    let meeting_sec_a: Vec<Vec<i32>> = vec![vec![13, 15], vec![15, 16], vec![7, 9]];
    let meeting_sec_b: Vec<Vec<i32>> = vec![vec![14, 15], vec![5, 10]];
    
    let intersection = overlapping_meetings(meeting_sec_a, meeting_sec_b);
    println!("The overlapping timings are {:?}", intersection);
}
```
- #### Output
```shell
The overlapping timings are [[14, 15], [7, 9]]
```
---------------------------------------------------------
## Longest Non-Stop Working Hours
---------------------------------------------------------
### Problem Statement - *Given time slots numbers, we want to determine the longest consecutive (non-stop) time slots.*
### Data Structures and Patterns Used - *Vectors, Loops, HashSets*
=========================================================
- #### Real Life Scenario
    - *Suppose we are working on a project with very tight time schedule. We want our employees to put some extra effort, to make sure that the employees spent their time productively, we have divided their time schedule in chunks of two hour slots. The first slot is **8-10** on Monday, the next slot is **10-12** on the same day and so on... We have numbered all the slots during the week. After each time slot we would like the employee to tell us what he has produced. The employees may decide to take some rest for some slots during the week. However, some of the employees may work too much in consecutive time slots and may negatively affect their all productivity and well being. __We would like to determine the employees who has worked non-stop for the longest time in some interval of time (let's say a week or a month), so that we can ask him to have some break.__ This is necessary for their overall well being.*
- #### Implementation Detail
    - **Assumption** - The input will be in the form of vector containing numbers indicating time slots, the numbers are not necessarily in order. We will be using hash sets to implement the solution.
    - **Approach** - We will be using hash sets to store the time slots. We will then iterate over the hash set to find the longest consecutive time slots.
    
```rust
use std::collections::HashSet;

pub fn main() {
    let schedule = vec![
        vec![4, 1, 2, 5, 6, 10, 11],
        vec![3, 1, 2, 5, 7, 10, 11, 14],
        vec![3, 1, 15, 5, 13, 12, 10, 14, 15, 16, 17, 18, 8, 9],
    ];

    println!(
        "Employee Number {} has the highest number of nonstop working hours",
        longest_busy_time(schedule)
    );
}

fn longest_busy_time(working_slots: Vec<Vec<u8>>) -> u8 {
    let mut employee_longest_nonstop_work: Vec<u8> = Vec::new();
    for employee in working_slots {
        employee_longest_nonstop_work.push(longest_period(employee));
    }
    for i in 0..employee_longest_nonstop_work.len() {
        println!(
            "Employee {} has worked nonstop for {} slots",
            i, employee_longest_nonstop_work[i]
        );
    }
    let max_nonstop_work = employee_longest_nonstop_work.iter().max().unwrap();
    employee_longest_nonstop_work
        .iter()
        .position(|&r| r == *max_nonstop_work)
        .unwrap() as u8
}

fn longest_period(employee_slots: Vec<u8>) -> u8 {
    let mut longest_busy_period = 0;

    let slot_set = employee_slots.into_iter().collect::<HashSet<_>>();

    for slot in &slot_set {
        if !(slot_set.contains(&(slot - 1))) {
            let mut current_slot = slot.to_owned();
            let mut current_consecutive_slot = 1;
            while slot_set.contains(&(current_slot + 1)) {
                current_slot += 1;
                current_consecutive_slot += 1;
            }
            if current_consecutive_slot > longest_busy_period {
                longest_busy_period = current_consecutive_slot;
            }
        }
    }
    longest_busy_period
}
```
- #### Output
```shell
Employee 0 has worked nonstop for 3 slots
Employee 1 has worked nonstop for 3 slots
Employee 2 has worked nonstop for 7 slots
Employee Number 2 has the highest number of nonstop working hours
```
---------------------------------------------------------
## Items Suggestions
---------------------------------------------------------
### Problem Statement - *Given a list of prices, return a couple of items with their sum matching the target price.*
### Data Structures and Patterns Used - *HashMap, Vectors, Loops*
=========================================================
- #### Real Life Scenario
    - *There is an online shopping business who recently held a lucky draw contest where thousands of people participated. The ones who were successful have been given a $50 shopping gift card which can be used to buy items from the online store. There is however a restriction from business that the customers can only buy two products at most. Now we want to help the customers by suggesting a list of triplets that contains products worth $50 exactly, so that they are not worried from paying the extra from their own pockets. In other words, we want to suggest package deals containing two products that sum up to $50, and we want to suggest as many such possibilities as there may exist to implement the feature, we will have access to a list of products that the customer is likely to buy. These products will include products from the person wish list and other products based on the previous purchases. Our job is to return the possible pair of products whose sum is $50.*
- #### Implementation Detail
    - **Assumption** - We are given a list of prices (as vectors in Rust). And prices value does not repeat.
    - **Approach** - We will be using hash sets and loops to implement the solution.

```rust
use std::collections::HashSet;

pub fn main() {
    let product = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
    println!("{:?}", product_suggestions(product, 50));
}

fn product_suggestions(product_prices: Vec<i32>, amount: i32) -> Vec<Vec<i32>> {
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();

    for price in product_prices {
        let diff = amount - price;
        if !prices_hash.contains(&diff) {
            prices_hash.insert(price);
        } else {
            offers.push(vec![price, diff]);
        }
    }

    offers
}
```
- #### Output
```shell
[[30, 20], [45, 5], [10, 40]]
```
- #### Explanation
    - We are iterating over product prices and calculating the difference between the target price and the current price. If the difference is not present in the hash set, we are inserting the current price into the hash set. If the difference is present in the hash set, we are pushing the current price and the difference into the offers vector, because we know if any price is present in the hash set, it is price of some other product which can be bought with the current product to make the total price equal to the target price.
---------------------------------------------------------
## Items in Range
---------------------------------------------------------
### Problem Statement - *Given a list of items and a range, return the items that fall within the price range.*
### Data Structures and Patterns Used - *Binary Search Trees, Box Pointers, Vectors*
=========================================================
- #### Real Life Scenario
    - *Consider a business where many products are available for purchase, the customer are frequently interested in determining the products which are in some price range. Since there are too many products with lots and lots of data, the company wants to quickly retrieve this information. Therefore they would like to have a suitable data structure for storing the product price information, which will enable quick retrieval of the requested information. As consultant of business we recommend __binary search tree__ for efficient storage and quick retrieval.*
- #### Implementation Detail
    - **Assumption** - We are given a list of items and a price range. 
    - **Approach** - We will be using binary search trees to implement the solution.
=========================================================
#### Tree
- **Tree** is a collection of **nodes**. **Nodes** are connected by **edges**. Each **node** contains a **value** or **data**, and it may or may not have a **child node**. The **topmost node** is called the **root node**. The **nodes** that have **child nodes** are called **internal nodes**. The **nodes** that do not have **child nodes** are called **leaf nodes**. The **edges** that connect the **nodes** are called **branches**. The **height** of a **tree** is the **length of the longest path** to a **leaf node**. The **depth** of a **node** is the **length of the path** to the **root node**. The **depth** of the **root node** is **0**. The **height** of a **tree** is **1** more than the **depth** of the **deepest node**.
#### Binary Tree
- A **binary tree** is a **tree** in which each **node** has **at most two children**, which are referred to as the **left child** and the **right child**.
- **Binary Search Tree** is a **binary tree** in which the **left child** of a **node** contains only **nodes** with **values** less than the **node's value**, and the **right child** of a **node** contains only **nodes** with **values** greater than the **node's value**.
- The *left* and *right* subtree each must also be a binary search tree themselves.
=========================================================
```rust
#[derive(Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: i32) {
        if val > self.value {
            match self.right {
                None => self.right = Some(Box::new(Node::new(val))),
                Some(ref mut node) => node.insert(val),
            }
        } else {
            match self.left {
                None => self.left = Some(Box::new(Node::new(val))),
                Some(ref mut node) => node.insert(val),
            }
        }
    }
}

struct BinarySearchTree {
    root: Node,
}

fn traversal(node: Option<Box<Node>>, low: i32, high: i32, mut output: &mut Vec<i32>) {
    if node.is_some() {
        // Calling with as_ref() to avoid moving the value out of the Option
        if node.as_ref().unwrap().value >= low && node.as_ref().unwrap().value <= high {
            output.push(node.as_ref().unwrap().value);
        }
        if node.as_ref().unwrap().value >= low {
            traversal(node.as_ref().unwrap().left.clone(), low, high, &mut output);
        }
        if node.as_ref().unwrap().value <= high {
            traversal(node.as_ref().unwrap().right.clone(), low, high, &mut output);
        }
    }
}

fn products_in_range(root: Node, low: i32, high: i32) -> Vec<i32> {
    let mut output = Vec::new();
    traversal(Some(Box::new(root)), low, high, &mut output);
    output
}

pub fn main() {
    let product_prices = vec![9, 6, 14, 20, 1, 30, 8, 17, 5];

    let mut bst = BinarySearchTree {
        root: Node::new(product_prices[0]),
    };

    for i in 1..product_prices.len() {
        bst.root.insert(product_prices[i]);
    }

    let result = products_in_range(bst.root, 7, 20);
    println!("Products in range {:?}", result);
}
```
- #### Output
```bash
Products in range [9, 8, 14, 20, 17]
```
- #### Explanation
    - We have implemented a binary search tree to store the product prices. We have then inserted the product prices into the binary search tree. We have then found the products in the price range 7 to 20 using `products_in_range` function.
    - `products_in_range` function takes the root of the binary search tree, the lower limit of the price range, and the upper limit of the price range as input. It then traverses the binary search tree and finds the products in the price range using the `traversal` function.
    - `traversal` function takes the node of the binary search tree, the lower limit of the price range, the upper limit of the price range, and the output vector as input. It then traverses the binary search tree and finds the products in the price range, using three conditions:
        - If the value of the node is greater than or equal to the lower limit and less than or equal to the upper limit, the value of the node is pushed into the output vector.
        - If the value of the node is greater than or equal to the lower limit, the left child of the node is traversed.
        - If the value of the node is less than or equal to the upper limit, the right child of the node is traversed.
---------------------------------------------------------
## Fetching Top Products
---------------------------------------------------------
### Problem Statement - *Given a list of products and their ratings, return the top products.*
### Data Structures and Patterns Used - *Linked Lists, Iterators*
=========================================================
#### Real Life Scenario

- *A business has operations in multiple countries, in each countries, business maintains a list of top products. Country wise lists are stored in the form of linked lists that information regarding ranks of the products. Business wants to consolidate the lists so that it can determine the products that are being overall ranked top across all the countries. They want to combine linked lists in a single linked list sorted with their ranks.*

#### Implementation Detail
- **Assumption** - We are given a list of products and their ratings. We will be using linked lists to implement the solution.
- **Approach** - We will be using linked lists to store the ranks, for simplicity sake, we are storing only the ranks. Product detail can be stored in the same node or as a hashmap.

```rust
#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<T> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(current_head) => Some(&current_head.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        loop {
            match list_traversal {
                Some(current_node) => {
                    print!("{:?} -> ", current_node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                }
                None => {
                    print!("None");
                    break;
                }
            }
        }
        println!("\n");
    }

    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut previous = None;
        let mut current = self.head.take(); // Take the head of the linked list
        while current.is_some() {
            let mut next = current.as_mut().unwrap().next.take(); // Take the next node of the current node
            current.as_mut().unwrap().next = previous.take(); // Reverse the link
            previous = current.take(); // Move the previous pointer to the current node
            current = next; // Move the current pointer to the next node
        }
        self.head = previous.take(); // Set the head of the linked list to the last node
    }
}

pub fn main() {
    let mut list1 = LinkedList::new();
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(23);
    list1.add(11);

    println!("List 1:");
    list1.print();

    let mut list2 = LinkedList::new();
    list2.add(60);
    list2.add(40);
    println!("List 2:");
    list2.print();

    let mut list3 = LinkedList::new();
    list3.add(85);
    list3.add(20);
    list3.add(15);
    println!("List 3:");
    list3.print();

    let mut consolidated = sort_lists(&mut vec![list1, list2, list3]);
    println!("Consolidated List:");
    consolidated.print();

    println!("Reversing the consolidated list, to get the top products in ascending order");
    consolidated.reverse();
    println!("Consolidated List:");
    consolidated.print();
}

fn sort_lists(lists: &mut Vec<LinkedList<i32>>) -> LinkedList<i32> {
    let mut consolidated_list = LinkedList::new();
    let mut values: Vec<i32> = Vec::new();

    loop {
        values = lists
            .into_iter()
            .map(|list| list.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>(); // Collect head values of all linked lists in a vector
        
        let min_val = *values.iter().min().unwrap();    // Find the minimum value
        let min_index = values.iter().position(|&x| x == min_val).unwrap(); // Find the index of the minimum value, that is the linked list's index in lists vector

        consolidated_list.add(min_val);     // Add the minimum value to the consolidated list
        lists[min_index].remove();                  // Remove the minimum value from the linked list

        if lists[min_index].head.is_none() {    // If the linked list is empty, remove it from the lists vector
            lists.remove(min_index);
        }

        if lists.is_empty() {   // If all linked lists are empty, break the loop
            break;
        }
    }

    consolidated_list
}
```
- #### Output
```shell
List 1:
11 -> 23 -> 35 -> 40 -> 45 -> None

List 2:
40 -> 60 -> None

List 3:
15 -> 20 -> 85 -> None

Consolidated List:
85 -> 60 -> 45 -> 40 -> 40 -> 35 -> 23 -> 20 -> 15 -> 11 -> None

Reversing the consolidated list, to get the top products in ascending order
Consolidated List:
11 -> 15 -> 20 -> 23 -> 35 -> 40 -> 40 -> 45 -> 60 -> 85 -> None
```
- #### Explanation
    - We have implemented a linked list to store the ranks of the products. We have then added the ranks of the products to the linked list. We have then printed the linked list.
    - We have then implemented a function to sort the linked lists and consolidate them into a single linked list. We have then printed the consolidated linked list.
    - We have then reversed the consolidated linked list to get the top products in ascending order. We have then printed the reversed consolidated linked list.
---------------------------------------------------------
## Efficient Storage and Retrieval of Words
---------------------------------------------------------
### Problem Statement - *Given a list of words, store them in a data structure and retrieve them efficiently.*
### Data Structures and Patterns Used - *Trie, HashMaps, Vectors*
=========================================================
#### Real Life Scenario
- *Consider a business case where we are involved in designing a module for search engine system that will enable efficient storage and retrieval of words. This module will act as a dictionary with insert and search functionalities. We would like to do search with a blazing fast speed. Since the speed is crucial to success of the system, we will implement the solution using `Trie` data structure.*
=========================================================
#### Trie
- `Trie` data structure is also known as digital tree or prefix tree. It is a type of _k_ array search tree which is used for locating specific keys from within a set. These keys are most often strings with links between nodes defined not by the entire key but by individual.

- It is a tree data structure used for efficient retrieval of a key in a large set of strings. A `Trie` is a tree where each node represents a single character of a string. The root node represents an empty string. Each node has a map of characters to child nodes. The nodes at the end of the tree represent the strings in the `Trie`.
=========================================================
- #### Implementation Detail
    - **Assumption** - We are given a list of words. We will be using a `Trie` data structure to store the words.
    - **Approach** - We will be using a `Trie` data structure to store the words. We will then implement the insert and search functionalities. We will also use a hash map to store the child nodes of the `Trie`.
```rust
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
```
- #### Output
```shell
Searching `there` in the dictionary result: true
Searching `them` in the dictionary result: false
Searching `ther` in the dictionary result: false
```
- #### Explanation
    - We have implemented a `Trie` data structure to store the words. We have then inserted the words into the `Trie`. We have then searched for the words in the `Trie` using the `search` function.
    - The `insert` function takes a word as input and inserts the word into the `Trie`. The `search` function takes a word as input and searches for the word in the `Trie`. It returns true if the word is present in the `Trie` and false otherwise.