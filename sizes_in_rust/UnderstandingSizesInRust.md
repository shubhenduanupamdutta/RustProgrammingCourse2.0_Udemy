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