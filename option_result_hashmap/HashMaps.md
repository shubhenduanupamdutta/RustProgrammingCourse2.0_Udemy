# HashMaps
<strong><h4>
<ul>
<li> Concept of HashMap is present in almost all programming languages.</li>
<li> It is a data structure that implements Map interface, i.e. a Key-Value pair</li>
<li> In Rust, HashMap is present in the standard library.</li>
<li> With the help from <code>Key</code> we can get the <code>Value</code>.</li>
<li> <code>Key</code> are uniques, with no duplicates allowed, but <code>Value</code> can be duplicated.</li>
</h4><strong>

- `HashMap` are parts of rust `std` library.
- They can be included in the scope using `use std::collections::HashMap;`
```rust
use std::collections::HashMap;

fn hashmap() {
    // Mapping name to age using HashMap
    let mut person: HashMap<&str, i32> = HashMap::new();

    person.insert("Alice", 20);
    person.insert("Bob", 25);
    person.insert("Shubhendu", 33);

    println!("Current person hashmap: {:#?}", person);
    
    // Getting the value of a key
    let name = "Alice";
    let age = person.get(name).unwrap();
    println!("{}'s age is {}", name, age);

    // Checking if a key is present
    let name = "Bob";
    if person.contains_key(name) {
        println!("{} is present in the hashmap", name);
    } else {
        println!("{} is not present in the hashmap", name);
    }
}
```
- We can use `HashMap::new()` to create a new empty `HashMap`.
- We can use `insert` method to insert a new key-value pair.
- We can use `get` method to get the value of a key. `get` returns an `Option` type.
- We can use `unwrap` to get the value from `Option` type.
- We can use `contains_key` method to check if a key is present in the `HashMap`.
- We can iterate over the `HashMap` using `for` loop.

```rust
use std::collections::HashMap;
for (name: &&str, age: &i32) in &person {
    println!("{} is {} years old", name, age);
}
```
- **Note**: `HashMap` is not ordered, so the order of key-value pairs may not be the same as the order of insertion.

```rust
let mut likes: HashMap<&str, &str> = HashMap::new();
    likes.insert("Alice", "Python");
    likes.insert("Bob", "Rust");
    likes.insert("Shubhendu", "Rust");

    // Let's updated the value of Bob
    likes.insert("Bob", "Python");
    println!("Current likes hashmap: {:#?}", likes);
```

- We can update the value of a key by inserting the same key with a new value, as shown above.
- Output in above case will be:
```rust
/*
Current likes hashmap: {
    "Alice": "Python",
    "Shubhendu": "Rust",
    "Bob": "Python",
}
*/
```
- There is another method `entry` which can be used to update the value of a key.
- `entry` returns an `Entry` enum which has `or_insert` method to insert a value if the key is not present.
- `insert` function will always insert the value, even if the key is already present.
```rust
    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 7, 8];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("Frequency of each element in the vector: {:#?}", &freq_vec);
```
- In the above code, we are calculating the frequency of each element in the vector.
- We are using `entry` method to get the value of the key, if the key is not present, we are inserting 0.
- We are then incrementing the value of the key by 1.