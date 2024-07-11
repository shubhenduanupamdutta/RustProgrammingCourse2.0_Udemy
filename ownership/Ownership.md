# Ownership Basics

## Ownership Rules
#### 1. Each value has a variable that is called its owner.
#### 2. There can only be one owner at a time.
#### 3. When the owner goes out of scope, the value will be dropped.

## Review of Basics of Memory
### Non-volatile Memory
- Non-volatile memory is a type of computer memory that can retain stored information even when not powered.
- Examples: Hard disk, SSD, Flash memory, etc.
- Common features across all non-volatile memory:
  - Slow to access compared to volatile memory.
  - Large storage capacity.
  - Retains data even when powered off.
  - Managed by the operating system.
  - Not directly relevant for program execution.

### Volatile Memory
- Volatile memory is a type of computer memory that requires power to maintain the stored information.
- Examples: RAM, Cache memory, etc.
- Common features across all volatile memory:
  - Fast to access compared to non-volatile memory.
  - Small storage capacity.
  - Loses data when powered off.
  - Used during power execution.

### Program memory layout
```
+---------------------------+
|            ⬇️              |   <- Stack
|                           |
|                           |   
|            ⬆️              |   <- Heap
+---------------------------+
|                           |   <- Static
+---------------------------+
```

#### Static
- Used to store programs binary instructions and static variables.
- This region is populated with relevant program data when your program starts up, and destroyed when the program exits.
- Clean up of values from this part is automatic.

#### Stack
- It deals with data which has fixed and known size at compile time.
- Values are stored in order, using LIFO (Last In First Out) order.
- Management of this region is easy and fast, since everything is very predictable, and doesn't require special computation.

#### Heap
- It deals with data which has unknown size at compile time or size which might change during runtime.
- Not possible to know the size of data at compile time, so it is not stored in any order, data is placed all over the place, where a suitable space is found.
- Management of this region is complex, requires a lot of memory management, and is slower than stack.

#### Notes
- During program execution stack and heap portion can grow and shrink.


## Coming back to ownership
```rust
fn main() {
    let s1 = String::from("hello");
    println!("{}", s1);
    s2 = s1;
    println!("{}", s1);    // Error
}
```
- In the above code, `s1` is the owner of the string "hello".
- Let's see how the String type is stored in memory.
    - `String` is made up of three parts:
        - Pointer (in Stack) to the memory in the heap that holds the contents of the string.
        - Length of the string. (in Stack)
        - Capacity of the string. (in Stack)
- When we assign `s1` to `s2`, the pointer, length, and capacity are copied, but not the data in the heap. This means that both `s1` and `s2` point to the same memory location in the heap. Due to rust ownership models, rust invalidates `s1` after the assignment, so we can't use `s1` after the assignment.
- We can use `.clone()` method to create a deep copy of the data in the heap, so that both `s1` and `s2` have their own memory in the heap.
- `copy` when only stack data is copied, `clone` when heap data is also copied.


## Ownership and Functions
```rust
fn main {
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1);
    println!("{:?}", vec_1);    // Error
}

fn take_ownership(vec: Vec<i32>) {
    println!("Vec is {:?}", vec);
}
```
- In the above code, `vec_1` is the owner of the vector.
- When we pass `vec_1` to the function `take_ownership`, the ownership of the vector is transferred to the function. So, we can't use `vec_1` after the function call.
- Since the ownership is transferred, the vector is dropped when the function ends.

```rust
fn gives_ownership() -> Vec<i32> {
    vec![1, 2, 3]
}

fn main() {
    let vec_1 = gives_ownership();
    println!("{:?}", vec_1);
}
```
- In the above code, the ownership of the vector is transferred from the function `gives_ownership` to the variable `vec_1`.

- Let's combine both the above examples:
```rust
fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn main() {
    let vec_3 = vec![1, 2, 3];
    let vec_4 = takes_and_gives_ownership(vec_3);
    println!("{:?}", vec_3);    // Error
    println!("{:?}", vec_4);    // [1, 2, 3, 10]
}