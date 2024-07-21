# Generics and Traits
--------------------------------

## Generics
--------------------------------
- Generics in Rust allows us to define functions, structs, enums, and traits, with placeholder for data types enabling us to flexible and reusable code by abstracting over types.
- Intuitively, they are some sort of `to be specified later` type.
```rust
struct Point {
    x: i32,
    y: i32,
}

fn generics() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point { x: 1.0, y: 2.0 }; // Error: expected i32, found f64    
}
```
- Above code will not compile because `Point` struct is defined to have `i32` data type for `x` and `y` fields. - We can use generics to make it work for any data type.
- To define Point with variety of generics data types we can use generics.
```rust
struct Point<T> {
    x: T,
    y: T,
}

fn generics() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point {x: 1.0, y: 2.0};
}
```
- Now, `Point` struct can be used with any data type. But `x` and `y` should be of same data type.
- We can also define multiple generics data types.
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn generics() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1, y: 2.0 };
}
```
- We can also define methods on generic types.
```rust
impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}
```
- Benefits of adding generic types information to implementation block is that it tells rust that this implementation block is for `Point` struct with generic types `T` and `U`.
- This helps rust to differentiate it from other implementation blocks for `Point` struct with different concrete types.
- We can have multiple implementation blocks for same struct with different types.
```rust
impl Point<i32, i32> {
    fn printing(&self) {
        println!("The value of the coordinates are: x = {}, y = {}", self.x, self.y);
    }
}
```
- Using separate implementation block for a type based on different type it could take allows us to achieve `specialization`
- `**Specialization**` is a feature that allows us to provide a more specific implementation for a particular type enabling us to optimize and customize the behavior for that type, in case `i32`.
- We are not allowed to have multiple implementations for same function as in generic implementation. Rust will throw an error. Because compiler will not be able to decide which implementation to use.
- If we really want to have multiple implementation based on type, we can use different names.
- Function duplicates are allowed when defined in different implementation block of different types.
```rust
impl Point<f32, f32> {
    fn printing(&self) {
        println!("The value of the coordinates are: x = {}, y = {}", self.x, self.y);
    }
}

impl Point<u32, u32> {
    fn printing(&self) {
        println!("The value of the coordinates are: x = {}, y = {}", self.x, self.y);
    }
}
```

- Generics are also used in standalone functions i.e. which are not defined in any struct or enum or traits.
```rust
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}
```
- `unimplemented!()` is a macro which tells rust to not to implement this function. It will not create any compile time error, but it will panic at run time.
- If intention is to implement the function later one, better use `todo!()` macro.
- **Generics do not incur any runtime cost.** Rust uses `monomorphization` to generate specific code for each concrete type used in the code.
- During compilation, rust will generate code for each concrete type used in the code. This will increase the size of the binary, but it will not have any runtime cost.
- One of the cost of generics is that it increases the compile time. Because rust has to generate code for each concrete type used in the code. It also may cause `code bloat` i.e. increase in the size of the binary.
------------------------------------------------------
## Traits
------------------------------------------------------
- Traits are similar to interfaces in other languages. They allow us to define a set of methods that a type must implement.
```rust
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}
```
- Consider above two structs `Square` and `Rectangle`. Both have `line_width` and `color` fields. 
- Suppose we want to implement some functionality for both the structs, such as `area` and `perimeter` calculation.
- One way, would be to define a function for each struct. Separate block for implementation and area method for each of them.
```rust
impl Square {
    fn calculate_area(&self) {
        println!("The area of the square is: {}", self.side * self.side);
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}
```
- This however has some issues, there is no restriction on the look and feel on the specific implementation of the method.
- Each type can implement the method in its own way. This can lead to inconsistency in the code.
- In the above case, `calculate_area` method is implemented with no return value for `Square` struct and with return value for `Rectangle` struct. We can also see that the method name is different for both the structs.
- Downside of these different implementation is that
    - We lack consistency in the code.
    - We have to remember the method name for each type, and pay special attention to the type on which we are calling the method.
- It would be better if we had an interface for area method, which will help us to have a consistent way of calling the method.
- This is where `traits` come into picture.
- This way we will not be worrying about specific types when calling the method.
- This phenomenon is also known as `polymorphism`.
- `polymorphism` allows us to invoke methods on an interface without being concerned about the specific type of the object.
- In rust, to share functionality and common interface we use `traits`. This is the way to achieve `polymorphism` in rust.

```rust
trait Shape {
    fn area(&self) -> f32;
}


impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}
```
- Now, we can call `area` method on any type which implements `Shape` trait.
- We will not be worrying about the specific type of the object.
- We will get compiler error if we deviate from the signature specified in the trait, in other implementations.
- Method calls are consistent, having similar and easy to follow syntax.
- We can also define default implementation for the trait methods.
- Default implementation allows us to define a default implementation of a behavior in a trait.
- It is defined by having a body in the trait method.
- This can be optionally overridden by the type implementing the trait.
```rust
trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented for this shape, returning dummy value.");
        0.0
    }
}
```
- In the above code, `perimeter` method has a default implementation. If the type implementing the trait does not provide its own implementation, the default implementation will be used.
- Usually all methods in the traits need to be implemented by the type implementing the trait. But, if we have a default implementation, we can skip implementing it in the type.
- **TO NOTE:** In classical inheritance, where data along with functionality can be shared, traits in rust only share functionality. They do not share data.
- There are however ways to share data in rust, but it is not through traits.
- We can create a new struct which will have the common data and then implement the trait for that struct.
```rust
struct DrawingInfo {
    line_width: u8,
    color: String,
}

struct Square {
    side: f32,
    info: DrawingInfo,
}

struct Rectangle {
    length: f32,
    width: f32,
    info: DrawingInfo,
}
```
- Now, we can implement `traits` for `DrawingInfo` struct. Thus sharing the data among the types.
- **Rust prioritizes composition over inheritance.**
- **Composition** refers to the capability of making new type by combining existing types and functionality. This promotes creation of compact and reusable code, which is easy to maintain and comprehend.
------------------------------------------------------
## Trait Bounds
------------------------------------------------------
- Previously defined `Shape` trait for `Square` and `Rectangle` structs.
- Let us now create, a general purpose function called `shape_properties` which will take any type which implements `Shape` trait and will print the area and perimeter of the shape.
- For this we need to restrict the generic type to types that implement shape trait.
- This can be done using `trait bounds`.
- `trait bounds` are a way to specify that a generic type can be any type that implements a particular trait.
- `trait bounds` are specified after the generic type after a colon `:` followed by the trait name.
```rust
fn shape_properties<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}
```
- In the above code, `T: Shape` specifies that `T` can be any type that implements `Shape` trait. And So, we can call `area` and `perimeter` methods on `T`.
- Bounding of a generic type by a trait is called `trait bounds`. It has two effects
    - It restricts the generic type to the types that conform to the mentioned bounds.
    - Generic instances are allowed to call the methods defined in the trait specified in the bounds. In the above example T can call `area` and `perimeter` methods, since it is specified in the `Shape` trait.
- Multiple trait bounds can be mentioned using `+` operator.
```rust
fn shape_properties<T: Shape + std::fmt::Debug>(object: T) {
    object.area();
    object.perimeter();
    println!("{:?}", object);
}
```
- Sometimes trait bounds are mentioned using slightly different syntax.
```rust
fn shape_properties(object: impl Shape) {
    object.area();
    object.perimeter();
}
```
- This syntax is called `impl trait syntax`. It has essentially the same meaning as `trait bounds`, but it is easier to read and understand.
- Last way to define a trait bound is to use where clause.
```rust
fn shape_properties<T>(object: T) where T: Shape {
    object.area();
    object.perimeter();
}
```
- This is specially useful when we have multiple trait bounds.
- This `where` clause syntax is very easy to read and understand.
- `trait bounds` can also be used for return values.
```rust
fn returns_shape() -> impl Shape {
    Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Red"),
        }
    }
}
```
- To specify return type as a trait, we can use `impl Trait` syntax and only `impl trait` syntax can be used for return types.
- `impl trait syntax` for return values only works for single concrete type. Meaning, if have some condition based on which we want to return different types, that will not be possible.
------------------------------------------------------
## Super Traits
------------------------------------------------------
- In rust, `super traits` are traits that acts as a generalized or higher level trait, encompassing the functionality of multiple traits.
- Its closely to related to `inheritance` in object oriented programming languages.
- It specifically refers to combining and extending the functionality of multiple traits.
- The trait to which we extend other traits is called `super trait`.
```rust
trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value.");
        0.0
    }
}
```
- In the above code, `Shape` trait extends `Draw` trait. `Draw` trait is the `super trait` for `Shape` trait.
- `Shape` trait will have all the methods of `Draw` trait along with its own methods.
- So all the types that implement `Shape` trait will have to implement `Draw` trait as well.
- We can mention multiple traits as super traits.
- Similar to mentioning multiple trait bounds, we can mention multiple super traits using `+` operator.
```rust
trait Shape: Draw + std::fmt::Debug {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value.");
        0.0
    }
}
```

- One of the benefit of `super trait` is that, it helps us in reducing list of `trait bounds` in the function, struct, enum signatures.
- Instead of mentioning all the traits, we can mention only the `super trait`.
```rust
trait OtherTrait {};  // Marker Traits
impl OtherTrait for Square {};
impl OtherTrait for Rectangle {};

trait SomeOtherTrait {};
impl SomeOtherTrait for Square {};
impl SomeOtherTrait for Rectangle {};

fn some_function<T>(object: T)
where
    T: Shape + OtherTrait + SomeOtherTrait
{
    object.area();
    object.perimeter();
}
```
- In above case we only have 3 traits, but there can be more than 3 traits required.
- The `super traits` can be used to reduce the number of traits mentioned in the `where` clause.
- By mentioning other traits as `super traits`, we can reduce the number of traits mentioned in the `where` clause.
```rust
trait Shape: OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
}

fn some_function<T>(object: T)
where
    T: Shape
{
    object.area();
    object.perimeter();
}
```
- In the above code, we have reduced the number of traits mentioned in the `where` clause by mentioning `OtherTrait` and `SomeOtherTrait` as `super traits` for `Shape` trait.
- Since `Shape` trait is a `super trait` for `OtherTrait` and `SomeOtherTrait`, we can mention only `Shape` trait in the `where` clause, since it will automatically mean that T implements `OtherTrait` and `SomeOtherTrait`.

- **TO NOTE:** `super traits` are not a feature in rust. It is just a way to organize and manage the traits in a better way.

------------------------------------------------------
## Trait Objects
------------------------------------------------------
- Consider the shape properties function,
```rust
fn shape_properties<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}

fn main() {
    let square = Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Red"),
        }
    };
    let rectangle = Rectangle {
        length: 5.0,
        width: 3.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Blue"),
        }
    };
    shape_properties(square);
    shape_properties(rectangle);
}
```
- In the above case, behind the hood, rust compiler generates concrete function for each type implementation of the generic function `shape_properties`.
- In the above case, compiler generates two concrete functions for `shape_properties` function, one for `Square` and one for `Rectangle`.
- This is called `monomorphization` or `static dispatch`.
- It improves performance by improving runtime overhead.

- We also have `Dynamic Dispatch` in rust. In this case concrete function is not generated for each type.
```rust
fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}
```
- `Box` is a smart pointer.
- `Box` is a pointer to some heap allocated data.
- `dyn` stands for `Dynamic Dispatch`, and is used to define a `trait object`.
- Special requirement for a `trait object` is that it should be behind a pointer.
- In this case we have used `Box` smart pointer.
- `trait object` allows us to define a type which implements a trait without knowing or having the knowledge of what the type actually is at the compile time.
- In this case, specialized (`mono morphized`) function is not generated for each type. And execution is done at runtime.
```rust
fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}


fn main() {
    let (r1, s1) = get_rectangle_and_square();

    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));
}
```
- In the above code, `shape_properties_dynamic` function takes a `trait object` as an argument.
- `Box::new(r1)` and `Box::new(s1)` are `trait objects`.
- `Box::new()` is used to create a pointer to the heap allocated data.
- `Static Dispatch` is when the compiler knows which function to call at compile time.
- `Dynamic Dispatch` is when the compiler does not know which function to call at compile time, so it inserts a bit of code so compiler can decide it at runtime.
- A requirement of `trait object` is that it should be behind a pointer. We can use any type of pointer. Here we have used `Box` smart pointer, but we can use `&` or `Rc` or `Arc` as well.
```rust
fn shape_properties_dynamic(object: &dyn Shape) {
    object.area();
    object.perimeter();
}
```
- Essential advantage of `trait object` is that it provides us with flexibility.
### Original Code
```rust
fn returns_shape() -> impl Shape {
    Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Red"),
        }
    }
}
```
- Above function can only be used if we are returning a single concrete type, and not based on some condition.
- This is because compiler will use `monomorphization` to generate the concrete function for the return type, and it will not be able to do so if the return type is based on some condition.
- In such cases, we can use `trait object`.

### Dynamic Dispatch Code using Trait Object
```rust
fn returns_shape_dynamic(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        Box::new(Square {
            side: dimension[0],
            info: DrawingInfo {
                line_width: 2,
                color: String::from("Red"),
            },
        })
    } else {
        Box::new(Rectangle {
            length: dimension[0],
            width: dimension[1],
            info: DrawingInfo {
                line_width: 1,
                color: String::from("blue"),
            },
        })
    }
}
```
- In the above code, `returns_shape_dynamic` function returns a `trait object` based on the condition. And compiler will not throw an error, because decision is made at runtime.

- We can also use a `trait object` as a type inside a vector, to point to a vector of different types, that implement the same trait.
```rust
fn main() {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::new(Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Red"),
        },
    }));
    shapes.push(Box::new(Rectangle {
        length: 5.0,
        width: 3.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Blue"),
        },
    }));

    for shape in shapes {
        shape.area();
        shape.perimeter();
    }
}
```
------------------------------------------------------
## Derived and Marker Traits
------------------------------------------------------
### Derived Traits
- `Derived Traits` are traits that are automatically implemented by the compiler for a type, based on the properties of the type.
- Rust provides a way to automatically implement some traits for a type using `derive` attribute.
- `derive` attribute is used to automatically implement some traits for a type.
- These traits are applied to `structs` and `enums`.
- They provide default implementations for some common traits.
```rust
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

fn derived_and_marker_traits() {
    let s1 = Student {
        name: String::from("ABC"),
        age: 35,
        sex: 'M',
    };

    let s2 = Student {
        name: String::from("XYZ"),
        age: 40,
        sex: 'F',
    }

    println!("Student 1: {:?}", s1);
}
```
- `{:?}` is used to print the struct using `Debug` trait.
- `Debug` trait is defined in `std::fmt` module of Rust standard library.
- `[#derive(Debug)]` is used to automatically implement `Debug` trait for `Student` struct.
- Without this line, we will not be able to print the struct using `{:?}`. Compiler will throw an error.
- We can also implement custom `Debug` trait for our struct.
- Derived traits are available for common traits like
    - `Clone`
    - `Copy`
    - `Debug`
    - `PartialEq`
    - `Eq`
    - `PartialOrd`
    - `Ord`
    - `Default`
    - `Hash` and others
```rust
println!("s1 and s2 are equal: {}", s1 == s2);
```
- `PartialEq` trait is used to compare two values for equality, if they have same values for field.
- `Eq` trait is used to compare two values for equality, but it is stricter than `PartialEq`.
- Without implementing `PartialEq` trait, we will not be able to compare two values of the type.
- And we can do that using derived attribute or custom implementation.
```rust
#[derive(PartialEq, Debug)]
struct Student {
    name: String,
    age: u32,
    sex: char,
}
```
- **NOTE:** Derived attribute provide a sort of default implementation of the types in the Struct or Enum, if there is no implementation of the trait for field types, it will not work.

### Marker Traits
- `Marker Traits` are traits that do not have any methods or associated functions.
- We can say that its body is empty.
- `Marker Traits` are typically used to add metadata or constraints to a type. It helps in communicating additional information to the compiler without adding any functionality.
- This is done by adding some super traits to the `Marker Trait`.
- `Marker Traits` are used to add some constraints to the type.

```rust
trait Properties: PartialEq + Default + Clone {}
```
- Now any type that implements `Properties` trait will have to implement `PartialEq`, `Default` and `Clone` traits as well.
- `Default` trait is used to provide a default value for a type.
- `Clone` trait is used to create a copy of a value.
- `Marker Traits` are quite handy in situation where we want certain types to have some essential behavior.

------------------------------------------------------
## Associate Types in Traits
------------------------------------------------------
- `Associated Types` are a way to define a type inside a trait, which can be used by the trait methods.
- Suppose I want to calculate how far my car will travel in 3 hours, if it is going at certain speed.
- I am going to define a couple of structs for this purpose.
```rust
#[derive(Debug)]
struct Km {
    value: u32,
}

// Kmh is kilometers per hour
#[derive(Debug)]
struct Kmh {
    value: u32,
}
```
- Defining structs with single fields like in `Km` and `Kmh` provides some protection against programming errors.
- Now its impossible to accidentally compare `Km` with `Kmh` or add `Km` to `Kmh`.
- Let's accommodate people who use miles instead of kilometers.
```rust
#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}
```
- Now let's define some methods to calculate the distance.
```rust
impl Kmh {
    // self represents Kmh and represents speed
    fn distance_in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl Mph {
    fn distance_in_three_hours(&self) -> Miles {
        Miles {
            value: self.value * 3,
        }
    }
}
```
- Let's now use this in main function.
```rust
fn main() {
        let speed_kmh = Kmh { value: 60 };
    let distance_km = speed_kmh.distance_in_three_hours();

    println!("At {:?} Kmh, the car will travel {:?} Km in 3 hours", speed_kmh, distance_km);

    let speed_mph = Mph { value: 40 };
    let distance_miles = speed_mph.distance_in_three_hours();

    println!("At {:?} Mph, the car will travel {:?} Miles in 3 hours", speed_mph, distance_miles);
}
```
- This code will work and compile, but it seems weird.
- There is almost same implementation for `Kmh` and `Mph` structs. It is perfect use for unification using traits.
- But there is a problem with output, we are getting `Km` and `Miles` as output. But if we define a `trait` output can at most be one type.
- Here comes the the use of `Associated Types`.
- `Associated Types` allows us to define placeholder types inside a trait, where concrete type is determined by implementing type.
- This provides flexibility for trait implementers to choose the specific type which makes sense for their particular implementation.
- Syntax for defining `Associated Types` is similar to defining generics.
```rust
trait DistanceThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}
```
- `Distance` is now a new trait item, `Self::Distance` is the syntax to refer to the associated type.
- It is up to the implementer to decide what type to use for `Distance`.
- Now, we can implement this trait for `Kmh` and `Mph` structs.
```rust
impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Miles {
        Miles {
            value: self.value * 3,
        }
    }
}
```
- This way of implementation, makes the code more organized, clean and unified.

### Points to Note
- **Associated Types shouldn't be confused with generics, they serve different purposes and operate in different context.**
- **Generics are used for defining functions, structs and enums that can work with variety of types.**
- **Associate types are used in traits to define abstract types, that an implementation must specify.**
- **Associated types define a placeholder that will be specified by the type implementing the trait.**
- `type` **keyword is also used to create type aliasing.**
- **However within the context of a trait system it is used to define associated types.**
------------------------------------------------------
## Choosing Associated Types vs Generic Types
------------------------------------------------------
- Both `Associated Types` and `Generic Types` defer the decision to the implementer of which concrete type to use in the crates function and methods.
- Let's define a trait called `Addition` with some associated types.
```rust
trait Addition {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}
```
- This `Addition` trait will let us add two types, one of which will be the type implementing the trait (left hand side (Lhs)) and the other will be the type specified by the implementer (Right hand side (Rhs)).
- Let's define a point struct and implement the trait for it.
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Addition for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point2 { x: 1, y: 2 };
    let p2 = Point2 { x: 3, y: 4 };

    let p3 = p1.add(p2);
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);
}
```
- Above code will work with no problems.
- Now let's say we want to add the ability to add integer to points.
- Integer value will be added to both x and y fields of the point.
- But if we try to implement it for the same trait, we will get an error.
```rust
impl Addition for Point {
    type Rhs = i32;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
```
- This will throw an error, because we have already implemented the trait `Addition` for `Point` struct.
- Since trait is not parametrized by any generic types, we can only implement it once for a type.
- We can only pick one type for `Rhs` and `Output` associated types.
- Which means we have to refactor Rhs to be a generic type, which will allow us to implement the trait for multiple types.
- Let's change the `Rhs` associated type to a generic type.
- Generics allow us to have multiple implementations for the same trait for single type.
```rust
// After refactoring using Rhs Generic
trait Addition<Rhs> {
    type Output;
    fn add(&self, rhs: Rhs) -> Self::Output;
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Addition<Point2> for Point2 {
    type Output = Point2;
    fn add(&self, rhs: Point2) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32> for Point2 {
    type Output = Point2;
    fn add(&self, rhs: i32) -> Self::Output {
        Point2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
```
- Decision for which one to use, is very straightforward now.
- We will use `Associated Types` when we want to have a single implementation for a type.
- We will use `Generic Types` when we want to have multiple implementations for a type for given trait.
- Let's define a struct `Line`.
```rust
struct Line {
    start: Point2,
    end: Point2,
}
```
- Let's assume adding two `Point2` produce a `Line`.
- So output should also be possible to be `Line` along with `Point2`.
- This is not possible with current design of `Addition` trait.
- Let's refactor the trait to use `Generic Types`.
```rust
trait Addition<Rhs, Output> {
    fn add(self, rhs: Rhs) -> Output;
}

impl Addition<Point2, Point2> for Point2 {
    fn add(self, rhs: Point2) -> Point2 {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point2> for Point2 {
    fn add(self, rhs: i32) -> Point2 {
        Point2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Addition<Point2, Line> for Point2 {
    fn add(self, rhs: Point2) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}
```
- Now we have to refactor main also, because add function can generate two types of outputs, for same input.
```rust
fn main() {
    let p1 = Point2 { x: 1, y: 2 };
    let p2 = Point2 { x: 3, y: 4 };

    let p3: Line = p1.add(p2);
    let p4: Point2 = p1.add(p2);

}
```