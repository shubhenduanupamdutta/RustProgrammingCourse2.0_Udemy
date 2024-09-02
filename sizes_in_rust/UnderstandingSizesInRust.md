# Understanding Sizes in Rust
----------------------------------------------
- Understanding size related issues in Rust is integral to writing safe, performant and efficient code.
- It allows you to make informed decisions, design better data structures and ensures memory safety throughout your code.
- From size perspective there are two types,
    - `Sized Types` - A type whose size is known at compile time.
    - `Unsized Types` - A type whose size is not known at compile time.
- `Dynamically Sized Types` - A different name for unsized types.

========================================================
## Sized Types
========================================================
- Some sized data types are,
    - `primitives` - All primitives are sized.
    - `tuples of primitives` - All tuples of primitives are sized.
    - `arrays of primitives` - All arrays of primitives are sized.
```rust
use std::mem::size_of;

pub fn main() {
    // Sized Types
    // Primitives
    println!("Size of i32: {}", size_of::<i32>());
    println!("Size of f64: {}", size_of::<f64>());
    println!("Size of char: {}", size_of::<char>());
    println!("Size of bool: {}", size_of::<bool>());
    println!("Size of usize: {}", size_of::<usize>());

    // Tuples consisting of primitives are also sized
    println!("Size of (i32, f64): {}", size_of::<(i32, f64)>());

    // Array of primitives are also sized
    println!("Size of [i32; 5]: {}", size_of::<[i32; 5]>());
}
```
```shell
Size of i32: 4
Size of f64: 8
Size of char: 4
Size of bool: 1
Size of usize: 8
Size of (i32, f64): 16
Size of [i32; 5]: 20
```
- All types constructed from primitive types, including structs, enums, HashMaps and Arrays are also sized.
- Consider a struct `Point` which is made up of two `i32` values.
```rust
use std::mem::size_of;

struct Point {
    _x: i32,
    _y: i32,
}

fn main() {
    println!("Size of Point: {}", size_of::<Point>());
}
```
```shell
Size of Point: 8
```
- The size of `Point` is `8` bytes, because it is made up of two `i32` values. Size of struct is determined by the sizes of its fields and padding added by the compiler for alignment.

- `Pointers` and `References` are also of fixes sizes, regardless of they being mutable or immutable.
```rust
println!("Size of &i32: {}", size_of::<&i32>());
println!("Size of &mut i32: {}", size_of::<Box<&mut i32>>());
```
```shell
Size of &i32: 8
Size of &mut i32: 8
```
- `Pointers` are one machine word in size, `A Machine Word` refers to the unit of data that a particular computer architecture can handle in one operation. The size of
machine word is architecture dependent, for 64-bit systems it is 8 bytes.

- `Smart Pointers` like `Box`, `Rc`, `RefCell` and `Arc` are also sized, because they are essentially wrappers around pointers.
```rust
// Machine Word Size
println!();
println!("Size of Machine Word: {}", size_of::<&()>());

// Smart pointers
println!();
println!("Size of Box<i32>: {}", size_of::<Box<i32>>());
println!("Size of Rc<i32>: {}", size_of::<std::rc::Rc<i32>>());
println!("Size of RefCell<i32>: {}", size_of::<std::cell::RefCell<i32>>());
println!("Size of Arc<i32>: {}", size_of::<std::sync::Arc<i32>>());
```
```shell
Size of Machine Word: 8

Size of Box<i32>: 8
Size of Rc<i32>: 8
Size of RefCell<i32>: 16
Size of Arc<i32>: 16
```
- `Function Pointers` are also of fixes sizes, since they are essentially also pointers.
```rust
println!("Size of fn(i32) -> i32: {}", size_of::<fn(i32) -> i32>());
```
```shell
Size of fn(i32) -> i32: 8
```
========================================================
## Unsized Types
========================================================
- Slice of an integer array is an unsized type.
- We can't even use `size_of` because compiler will not compile the code.
- Since a slice is just a reference to a contiguous section of elements in a collection, so its size is not known at compile time.
- In literature or in community `[i32]` is not referred to as an array slice, by an array slice we refer to `&[i32]`.
- Let's from here on refer to `[i32]` as `raw slice`.
- Compiler wil throw an error if we also want to define a variable of type `[i32]`.
```rust
use std::mem::size_of;

fn main() {
    // Unsized Types
    // Raw Slice
    println!("Size of [i32]: {}", size_of::<[i32]>()); // Compiler Error
    let _slice: [i32]; // Compiler Error

    // But if we add a size to it, the compiler error will go away, because it will convert from a raw slice to an array
    let _slice: [i32; 5];
}
```
- To properly use a raw slice, we must always use a reference to it.
- Similar to raw slice, we also have raw string slice `str`, whose size is also not known at compile time, same error will occur here also.
- If we again use a reference to it, the error will go away.
```rust
use std::mem::size_of;

fn main() {
    // Raw String Slice
    println!("Size of str: {}", size_of::<str>()); // Compiler Error
    let _str: str; // Compiler Error

    // But if we add a size to it, the compiler error will go away, because it will convert from a raw string slice to a string
    let _str: &str;
}
```
- `Trait Objects` are also unsized types, because their size is not known at compile time.
```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

println!("Size of dyn Shape: {}", size_of::<dyn Shape>());  // Compiler Error
```
- This is because traits can be implemented by any numbers of enums and structs with different sizes, so their size is not known at compile time.
- Here also we need to use a reference to it.
```rust
use std::mem::size_of;

trait Shape {
    fn area(&self) -> f64;
}

println!("Size of &dyn Shape: {}", size_of::<&dyn Shape>());
```
- Any complex types like enums, structs made of unsized types are also unsized.

========================================================
## Pointers to Sized and Unsized Types
========================================================
```rust
use std::mem::size_of;

pub fn main() {
    println!("Size of a reference or sized type: {}", size_of::<&[i32, 3]>());
    println!("Size of a reference to an unsized type: {}", size_of::<&[i32]>());
}
```
- Let's consider above code, `&[i32, 3]` is a reference to an array of size `3`, which is a sized type, so its size is known at compile time.
- `&[i32]` is a reference to an array slice. Array slice is an unsized type, so its size is not known at compile time.
- Let's check the output of above code.
```shell
Size of a reference or sized type: 8
Size of a reference to an unsized type: 16
```
- Why is reference to an unsized type is `16` bytes?
- Simple reference to sized types are one Machine Word (i.e. 8 bytes in 64-bit systems), but reference to unsized types are two Machine Words (i.e. 16 bytes in 64-bit systems).
- Why is this so?
- Let's define an array containing 3 elements,
```rust
let num_1: &[i32; 3] = &[1, 2, 3];
let num_2: &[i32] = &[1, 2, 3];
```
- In the first case, data length information is embedded in the type information itself. There for size information is not required to be stored again.
- But in second case, data length information is not attached to the type itself, therefore pointer needs to have some additional information (which may be mutable). That added information about size of data type is stored in next Machine Word size.
- Reference which are two machine word are referred to as `Fat Pointers`.
- References which are only one machine word are referred to as `Thin Pointers`.
- To confirm, that the size information is stored in the pointer itself in second case, we can iterate through the two data.
```rust
let num_1: &[i32; 3] = &[1, 2, 3];
let num_2: &[i32] = &[1, 2, 3];

let mut sum_1 = 0;
for num in num_1 {
    sum_1 += num;
}

println!("Sum of the array num_1: {}", sum_1);

let mut sum_2 = 0;
for num in num_2 {
    sum_2 += num;
}
println!("Sum of the slice num_2: {}", sum_2);
```
- We expect the second loop to throw an error, because size data is not explicitly stated anywhere, but it will compile and run successfully.
- Because size data is stored in the pointer itself, making the pointer a `Fat Pointer`.
- References to trait objects are also two machine words in size, because size information is stored in the pointer itself.
```rust

use std::mem::size_of;

trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    println!();
    println!("Size of reference to Circle: {}", size_of::<&Circle>());
    println!("Size of reference to Rectangle: {}", size_of::<&Rectangle>());
    println!("Size of a reference to a trait object: {}", size_of::<&dyn Shape>());
}
```
- Let's check the output of above code.
```shell
Size of reference to Circle: 8
Size of reference to Rectangle: 8
Size of a reference to a trait object: 16
```
- As expected, reference to trait object is `16` bytes, why is this, there is no length data needs to associated with trait objects.
- In this case the extra 8 bytes will be used to point to the virtual table or `vtable`. `vtable` is a table of function pointers associated with the trait.
- When a trait object is created rust generates a `vtable` that contains pointers to the actual implementation of the trait methods for the specific types.
========================================================
## Sized and Optionally Sized Trait
========================================================
- The `Sized` trait in Rust serves as both as `Auto Trait` and `Marker Trait`.

- #### Some Background:
    - `Auto Trait` - Auto traits are traits which are automatically implemented of a type if certain conditions are met.
    - `Marker Trait` - Marker traits on the other hand are traits which indicate that the type has certain properties.
- All `Auto Traits` are `Marker Traits`. Which means we can mention with a type and the compiler will provide automatic default implementation for them.
- For instance, for an `i32` type, some of the `Auto Traits are` `Copy`, `Clone`, `Default`, `Send`, `Sync`, `Sized` etc. This means I can call function mentioned in these traits on `i32` type without implementing them.
```rust
let x: i32 = Default::default();
println!("Checking default value of i32: {}", x);
```
- Since `Default` is an `Auto Trait` for `i32` type, therefore we can call `Default::default()` on `i32` type without implementing it.
- For `Sized Trait`, its auto implementation occurs when members/fields/variants/items of the type are also sized, i.e. have implemented `Sized` trait.
- If a type has auto implementation of `Sized` trait, this means that its size in bytes is known at compile time.
- Its impossible to opt-out of `Sized` trait, unlike some other `Auto Marker Traits`.
- Let's add `negative-impl` to our dependency.
```toml
[dependencies]
negative-impl = "0.1.6"
```
- My current version of Rust is `1.74`.
- `negative-impl` is a crate which allows us to put restrictions that a specific trait which will not be implemented on a specific type.
- This means that when we want to opt out of implementation of a specific `Auto Trait` for a specific type, we can use this crate.
```rust
use negative_impl::negative_impl;

struct ABC;

#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}
```
- `Send` trait indicates whether a type can be safely transferred across thread boundaries.
- `Sync` trait indicates whether references to a type can be safely shared between threads.
- In above code, we have opted out of `Send` and `Sync` traits for `ABC` type. Since we are not using `Send` and `Sync` traits, we can opt out of them.
- We can't opt out of `Sized` trait.
- Its very hard and not logical to imagine a situation where we would want to opt out of `Sized` trait. This brings no advantage.
- On the other hand, negative implementation of `Send` and `Sync` are very useful, because if we are not going to use a type across threads or in multithreading, these traits are not required.
- ### By default, `Sized` trait is automatically applied to a bound for every generic type parameter.
- For instance, if we define a function with a generic type parameter, compiler will actually read it as or (it will be de-sugared to),
```rust
fn some_fn<T>(t: T) {}
```
```rust
// De-sugared version (compiler reads it as)
fn some_fn<T: Sized>(t: T) {}
```
- This means by default, type parameters are expected to have a known size at compile time, ensuring memory management and safety.
- `<T: ?Sized>` - This syntax is used to specify that the type parameter `T` may or may not be `Sized`.
- It is often used in Trait Bounds to allow the implementation with bot Sized and Unsized types.
- The advantage of using `?Sized` (also called Optionally Sized) is that it provides flexibility. It allows us to write generic code that allows us to work with types whose size can be known or unknown at compile time.
- Parameter to a function can't have optional `Sized` trait, in that case only references can be used.
- Let's consider an example of a function which takes a parameter of type `T` which may or may not be `Sized`.
```rust
fn print<T: ?Sized>(t: &T) {
    println!("{:?}", t);
}
```
- But the code `fn print<T: ?Sized>(t: T) {...}` will not compile, because the size of `T` is not known at compile time.
- The `?` in this case is commonly known as `Widening Bound` or and `Expanded or Relaxed Bound`, because it relaxed the limitation on the type parameter, instead of restricting it.
- Special thing about `Optional Sized Bound`, `?Sized` is that, among the bounds in Rust, it is a sole example of relaxed constraint.

========================================================
## ?Sized and Generic Parameters
========================================================
### Use Case 1 of `?Sized` - Creating an Unsized Struct
- Let's first consider an unsized struct.
- Unsized Struct is a struct that contains one and only one unsized field.
- An unsized field will be of unsized type, such as a `Raw Slice` or `Trait Object`.
```rust
struct UnSizedStruct {
    sized_field_1: i32,
    unsized_field: [i32],
}
```
- To keep in mind to create an Unsized Struct, we need to fulfill two conditions:
    - It must have at one unsized field, and
    - The unsized field should be the last field in the struct.
- The above struct field fulfills the two requirements.
- If we try to add another field after the unsized field, the compiler will throw an error.
- These two requirements are related to Rust memory management and safety features. Rust needs to know the sizes of the fields in a struct at compile time, to correctly calculate the memory offsets, and do necessary allocations required to store and instance of the struct.
- In this case, the unsized field, whose size is not known at compile time, prevents scenarios where it might be unclear how much memory should be allocated for an instance of a struct.
- So when we define the struct, rust allows the struct to be defined, but it will not allow us to create an instance of the struct.
```rust
fn main() {
    let x = UnSizedStruct {
        sized_field_1: 3,
        unsized_field: [1, 2, 3],
    }
}
```
- This will throw compiler error,
```shell
the size for values of type `[i32]` cannot be known at compilation time
within `UnSizedStruct`, the trait `Sized` is not implemented for `[i32]`, which is required by `UnSizedStruct: Sized`
structs must have a statically known size to be initialized
```
- Why does the compiler throw an error?
- The reason for the error in this case is that every type gets the `Sized` trait as an `Auto Trait` which can't be opted out, and when compiler tries to auto calculate `Sized` trait for `UnSizedStruct`, it can't calculate the size of the unsized field `[i32]`.
- We can use optionally sized trait, `?Sized` to allow the struct to have an unsized field.
```rust
struct UnSizedStruct<T: ?Sized> {
    sized_field_1: i32,
    unsized_field: T,
}
```
- Now we can create an instance of the struct.
```rust
fn main() {
    let x = UnSizedStruct {
        sized_field_1: 3,
        unsized_field: [1, 2, 3],
    }
}
```
### Use Case 2 of `?Sized` - 
- Let's consider a print function with a generic parameter `T` with a `Debug` trait bound.
```rust
fn print_fn<T: Debug> (t: T) {
    println!("{:?}", t);
}
```
- Above function will be de-sugared to,
```rust
fn print_fn<T: Debug + Sized> (t: T) {
    println!("{:?}", t);
}
```
- Because generic parameters are auto bounded to `Sized` trait, we can't pass unsized types to the function.
- Now let's call this function
```rust
fn main() {
    let x: &str = "My name is Heisenberg";
    print_fn(x);
}
```
- Above code will compile and run successfully.
- But the current `print_fn` assumes ownership of any value passed to it, which can be somewhat inconvenient if we pass on non copy types, that is, heap allocated types.
- If we modify the function to only accept references, then the code will not compile, even though we are passing a reference to the slice `&str`.
```rust
fn print_fn<T: Debug> (t: &T) {
    println!("{:?}", t);
}
```
- Why is this?
- This is because when we pass `&str` to the function, basically we are saying, `T` will be resolved to Raw String Slice `str`, not the reference, and there is no `Sized` trait for `str`, only for `&str`.
- This is because de-sugared version of the function is `fn print_fn<T: Debug + Sized> (t: &T) {...}`.
- In this case, error will go away, if we add another `&` when passing to the function.
```rust
fn main() {
    let x: &str = "My name is Heisenberg";
    print_fn(&x);
}
```
========================================================

| Parameter Type    |    `T`     |   `&T`     |   `&T`     |
|-------------------|------------|------------|------------|
|Function Call Input|   `&str`   |   `&str`   |   `&&str`   |
|Resolves to        | `T = &str` | `T = str`  | `T = &str`  |
- Working with references like this `&&str` is very inconvenient and always remembering the correct conversion is hard to manage and error prone.
- We can use `?Sized` traits in this situation, now we can pass both `&str` and `str` to the function, without worrying about how `T` will be resolved.
```rust
fn print_fn<T: Debug + ?Sized> (t: &T) {
    println!("{:?}", t);
}

fn main() {
    let x: &str = "My name is Heisenberg";
    print_fn(x);
    print_fn(&x);
}
```
========================================================
## Unsized Coercion
========================================================
- `Unsized Coercion` occurs when a sized type is transformed into an unsized type.
- This is similar to `Deref Coercion`.

========================================================
### Refresher on Deref Coercion
- `Deref Coercion` occurs for automatic conversion of reference of a type to a reference of another type, specifically when using method and functions that expect certain types.
- `Deref Coercion` occurs when a type gets coerced into another type following a `deref` operation.
- Let's define a function that takes a string slice as an argument.
```rust
fn str_slice_fn(s: &str) {
    println!("{}", s);
}

pub fn main () {
    let some_string = "Calling with String".to_string();
    str_slice_fn(&some_string);
}
```
- The function `str_slice_fn` only accepts a `&str` type, but we are able to pass a reference to owned `String` type, since it can be coerced into a `&str` type.
- This is called `Deed of coercion`.
- In general, `deed of coercion` enables the function to accept any type, including custom defined types, which can be ultimately dereferenced to the expected type.

========================================================
### Continuing with Unsized Coercion
========================================================
- Let's see how `Unsized Coercion` works and is different from `Deref Coercion`.
- Let's create another function with an input of array slice.
```rust
fn array_slice_fn<T>(arr: &[T]) {}
```
- This function will accept any input that can be coerced into an array slice.
- Let's call this function with different types of inputs.

```rust
fn main() {
    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    array_slice_fn(&vec);
    array_slice_fn(&array);
    array_slice_fn(slice);
}
```
- In one of the call `array_slice_fn(slice)` we provided the expected input of array slice.
- In another call `array_slice_fn(&vec)` we provided a reference to a vector, but it was coerced into an array slice. Since vectors can be coerced into array slices and `Deref Coercion` took place.
- In the third case `array_slice_fn(&array)` we passed a reference to an array, while the function expects an array slice. In this case `array` which has a known size (3 in this case) is coerced into an array slice which don't have a known size at compile time. So an `Unsized Coercion` took place.
- Key difference between `Deref Coercion` and `Unsized Coercion` is that in `Deref Coercion` type changes, while in `Unsized Coercion` it is more appropriate to say that the property of the type changes from sized to unsized.
- Let's look at another example using traits.
```rust
trait SomeTrait {
    fn method(&self);
}

impl<T> SomeTrait for [T] {
    fn method(&self) {}
}
```
- In the above code, we have implemented `SomeTrait` for an array slice.
- But the method of this trait `method` can be called on many different types due to `Deref` and `Unsized` coercion.
- Let's get into detail for few cases where it can be called,
    - `Array Slice` any `&[T]` type - Default
    - `Vec<T>` - Via `Deref Coercion`
    - `[T; N]` - Via `Unsized Coercion`
```rust
fn main() {
    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    slice.method();
    vec.method(); // Deref Coercion happens
    array.method(); // Unsized Coercion happens
}
```
- In the above code, `slice.method()` will call the method on the array slice, because it is the default type.
- `vec.method()` will call the method on the array slice, because `vec` can be coerced into an array slice via `Deref Coercion`.
- `array.method()` will call the method on the array  slice, because `array` can be coerced into an array slice via `Unsized Coercion`.
--------------------------------------------------------
## Zero Sized Types
--------------------------------------------------------
- There are many `zero sized types` in rust, each having different use cases.

========================================================
### Never Type
========================================================
- `Never Type` represents computation that never resolves to any value. In other words, it corresponds to a computation that will always panic or exit the program, i.e. never finishes.
- `Never Type` is currently only available in Nightly Version of Rust.
- `Never Type` was supposed to be stabilized in Rust 1.41 but it was postponed.
- `Never Type` is represented by `!` symbol.
- **Currently only available in Nightly Version of Rust.**
```rust
#![feature(never_type)]

fn unrecoverable_state() -> ! {
    panic!("This is an unrecoverable state");
}

fn main() {
    unrecoverable_state();
}
```
- Functions returning `never type` are called `diverging functions`.
- They can only panic, exit the program or loop forever.
- `Never Type` is used when a function is guaranteed to never return normally.
- `Never Type` can not be assigned to a variable.
- Assignment of a function, returning never type is currently not a compile time error.
- We can create variable of never types using a function to return never type.
```rust
#![feature(never_type)]

fn unrecoverable_state() -> ! {
    panic!("This is an unrecoverable state");
}

fn main() {
    let x: ! = unrecoverable_state();
}
```
- `Never Type` can be used in match arms that are guaranteed to never be reached.
```rust
#![feature(never_type)]

let x = match "123".parse::<i32>() {
    Ok(num: i32) => num,
    Err(_) => panic!(),
}
```
- This code will compile and work, because `never type` can be coerced into any type.
- `return` also returns `never type` so can be coerced into any type.
- `break` and `continue` also return `never type`.
```rust
fn main() {
    let x: String = return;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }
}
```
- Due to back compatibility, `!` is not yet stabilized, but it is expected to be stabilized in future versions of Rust. So returning from loop it returns `()` type.
- Whereas in docs it is mentioned that `break` and `continue` return `!` type.
- `!` is a subtype of all types, so it can be coerced into any type.
- Another use case of never type is that it can be used to permit us to designate particular states as unreachable at type level.
- Consider a function which returns a result.
```rust

fn function() -> Result<i32, String> {}

fn function_1() -> Result<i32, !> {}
```
- With `function_1` signature we can confidently say that it will never return an error. This is because instantiating a `Result` with `!` type will never be possible.
- Alternative to simulate the behavior of the never type on the stable version is to use our own custom defined never types.
```rust
enum NeverType {}

fn function_2() -> Result<i32, NeverType> {}
```
- In this case, `NeverType` is a custom defined type which will never be instantiated, so it can be used to represent a never type, in case of returning a `Result` type.
- However our custom defined never type will not have the same properties as the never type, like being able to be coerced into any type.

========================================================
### Unit Type
========================================================
- The unit type represents a lack of meaningful data or information.
- It is denoted by `()`, parentheses with nothing inside.
- It has one and only one value, called `Unit Value` also denoted by `()`.
- Unit values are useful when we have some piece of code that does something, but do not return anything.
- Let's look at some scenarios,
```rust
fn f1() {}

fn main() {
    y = f1();
}
```
- Functions that do not return anything explicitly, returns a unit value.
- In the above case variable `y` stores the returning unit value.
- More precisely function can be `de-sugared` to,
```rust
fn f1() -> () {}
```
- In many cases, we are interested in knowing if the function has successfully done some processing or it failed for any reason. In the case of success we are not interested in the result.
- For instance, consider a division function which will return a result.
```rust
fn division_status(dividend: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => return Err("Division by zero".to_string()),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}
```
- Above function, returns a unit in case of success, error otherwise. 
- This depicts, a nice use case of the unit value, which together with the result, can be used to check if the status of some operation is successful or not.
- **All the statements return a unit value, except the last statement in the function.**
```rust
let z: () = println!("Hello, World!");
```
- **Vector with zero capacity are treated as vector of units.**
```rust
let mut _vec: Vec<()> = Vec::with_capacity(0);
_vec.push(());
_vec.push(());
_vec.push(());
println!("{:?}", _vec);

println!("Length of _vec: Vec<()> after three pushes: {}", _vec.len());
assert_eq!(_vec.len(), 3);

println!("Capacity of _vec: Vec<()> after three pushes: {}", _vec.capacity());

```
```shell
[(), (), ()]
Length of _vec: Vec<()> after three pushes: 3
Capacity of _vec: Vec<()> after three pushes: 18446744073709551615
```
- The compiler recognizes that the unit type has no size and optimizes its interactions with instances of the unit type.
- Therefore, pushing of unit values will only update the length of the vector and will not lead to any heap allocation or change in the capacity of the vector.
- We just said compiler will not change the capacity of the vector, then why we are getting the capacity as `18446744073709551615`?
- The compiler sets the capacity to be highest possible value, `u64::MAX` in this case.
- Whenever the length of the vector exceeds it allocated capacity, the capacity is increased by pinging the allocator and new allocation takes place.
- Since the zero sized types doesn't take any memory, therefore we shouldn't be pinging the allocator at all.
- In order to rule out the possibility of pinging the allocator, it has been allocated the highest possible value that a vector can take.
- #### IMPORTANT NOTE: It is very important to take note of the distinction between a unit type and never type.
|Sr.|         Never Type                |         Unit Type                |
|--|-----------------------------------|----------------------------------|
|1.| Represents computation that never produces a value | Represents a computation with no meaningful value. |
|2.| The Functions which return never type are guaranteed to never return normally | Functions that return unit type are guaranteed to return normally. |
|3.| Never type has no associated value and can be coerced into all other types | Unit type has single value, `()` and can not be coerced into any other type. |

========================================================
### Unit Struct
========================================================
- #### A unit struct is a struct with no fields due to absence of associated fields, it has zero size.
- Unit structs can be used as marker types, for marking types to ensure certain concept or properties.
- Let us go over a example.
- We would like to have different login methods being invoked based on the role being set to admin or user.
- We will defined two marker types of admin and user which represents roles in a login system.
```rust
struct Admin;

struct User;

trait Authenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}

impl Authenticate for Admin {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "adminpass"
    }
}


impl Authenticate for User {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "user" && password == "userpass"
    }
}

fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
    role.authenticate(username, password)
}

pub fn main() {
    let admin = Admin;
    let user = User;

    let admin_login = login(admin, "admin", "adminpass");
    let user_login = login(user, "user", "userpass");

    println!("Admin login status, logged_in?: {}", admin_login);
    println!("User login status, logged_in?: {}", user_login);
}
```
- The example demonstrates how marker types can be used to differentiate roles and behaviors within a simple login authentication system.
- There are many other useful use cases of the unit structs.
- Compared to other unit sized types, Unit Structs are not copy by default.
- In general, all structs are not copy by default, i.e. they are moved through ownership change, when assigned to a variable and not copied.
- Consider a unit variable, `x`, it can be copied many times.
```rust
let x = ();
let y = x;
let z = x;
```
- Now consider the unit struct variable, `x`, it can't be copied.
```rust
struct ABC;

let x = ABC;
let y = x;
let z = y;  // This line will throw compiler error
```
- The reason for this is that, the unit struct is not copy by default, and it is moved through ownership change, when assigned to a variable and not copied.
- This behavior is important in situations where you want to enforce strict ownership and prevent accidental data duplication.
- Let's say in the use case of being marker types, they may be used to enforce different behaviors based on their presence or absence in a function signature.
- If they were copyable, it could lead to unexpected behavior and unintended sharing of traits or marker information.
- For instance, in previous example, suppose we want to have single admin, but if the admin is copyable, we would unexpectedly create two admins, which may violate the requirement  of having a single admin of the system.

========================================================
### Phantom Data
========================================================
- `Phantom Data` is a marker struct with zero size.
- It helps in expressing the relationship and constraints between types without introducing any runtime overhead.
- Consider following struct,
```rust
struct ABC;
```
- If we want to opt out of the auto marker traits of `send` and `sync` for this struct.
- One approach was to use `negative-impl` crate and opt out of these traits.
- Use of `negative-impl` has drawbacks, it will depend on an external crate, which means that we will be bringing the crate into scope, which is associate with extra overhead.
- **Let's see another approach**
- A type has `send` and `sync` as `Auto-Trait` if all of its fields/members are also have `send` and `sync` as `Auto-Trait`, or as defined by the user.
- If we introduce a field, which doesn't have `send` and `sync` as `Auto-Trait`, then the type will not have `send` and `sync` as `Auto-Trait`.
- Luckily we have such a type, `Rc` smart pointer, which doesn't have `send` and `sync` as `Auto-Trait`.
- We can use `Rc` smart pointer to introduce a field in our struct, which doesn't have `send` and `sync` as `Auto-Trait`.
- Let's see how we can do this.
```rust
use std::rc::Rc;

struct ABC {
    ensuring_no_send_sync: Rc<()>,
}
```
- But again this approach is also not ideal, because it increases the size of every struct instance and introduces the need to create an `Rc` smart pointer whenever an instance of struct is created.
- Let's see size of the struct in cargo run
```rust
use std::rc::Rc;

struct A;

struct ABC {
    _ensuring_no_send_sync: Rc<()>,
}

pub fn main() {
    println!("Size of Unit Struct A: {}", std::mem::size_of::<A>());

    println!("Size of Struct ABC: {}", std::mem::size_of::<ABC>());
}
```
```shell
Size of Unit Struct A: 0
Size of Struct ABC: 8
```
- The size of struct `ABC` which doesn't implement `send` and `sync` has increased to `8` bytes, from `0` bytes.
- #### Phantom data can be used to solve this problem.
- Instead of defining the field to be a simple `Rc`, we change its type to that of a phantom data wrapping an `Rc` type.
- Let's see how we can do this.
```rust
use std::marker::PhantomData;
use std::rc::Rc;

struct ABC {
    ensuring_no_send_sync: PhantomData<Rc<()>>,
}
```
- Since `PhantomData` has zero size, therefore the size will remain as `0` and will maintain the constraints of not implementing `send` and `sync` as `Auto-Trait`.
- Adding a phantom data field to your type tells the compiler that your type acts as though it stores a value of certain type, even though it doesn't really.
- This is super helpful in ensuring certain property of the type, without introducing any runtime overhead.
- It's a zero sized type used for compile time type checking and optimizations, so it doesn't impact runtime performance.
--------------------------------------------------------