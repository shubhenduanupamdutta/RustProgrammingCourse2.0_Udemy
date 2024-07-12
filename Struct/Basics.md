# Structs Basics

#### Structs are used to create custom data types. They are similar to classes in other programming languages.
#### Structs are created using the keyword struct followed by the name of the struct. The fields of the struct are defined inside curly braces.

```rust
struct Point {
    x: i32,
    y: i32,
}

struct Car {
    make: String,
    model: String,
    year: i32,
}
```
- Structs can be instantiated using the struct name followed by curly braces containing the field values.

```rust
let p = Point { x: 10, y: 20 };
let c = Car { make: String::from("Toyota"), model: String::from("Corolla"), year: 2015 };
```
- Struct fields can be accessed using the dot operator.

```rust
println!("Point coordinates: ({}, {})", p.x, p.y);
println!("Car details: {} {} ({})", c.make, c.model, c.year);
```
- Ownership of heap allocated data can be moved out of a struct if clone is not implemented for the type.

```rust
struct Person {
    name: String,
    age: i32,
}

let p1 = Person { name: String::from("Alice"), age: 30 };
let p2 = p1.name; // Ownership of name field is moved to p2
```
- Ownership of heap allocated data `name` is moved to p2. p1 can no longer be used to access the name field.

- We can copy the values from one struct to another using the `..` syntax.

```rust
let p1 = Point { x: 10, y: 20 };
let p2 = Point { x: 30, ..p1 };
println!("Point coordinates: ({}, {})", p2.x, p2.y); // Point coordinates: (30, 20)
```
- The `..` syntax copies the values of the fields that are not explicitly set in the new struct.

## Tuple Structs
- Usually tuple are defined as
```rust
let point1 = (10, 20);
let point2 = (3, 4, 5);
```
- In the above case meaning of the tuple fields are not clear.
- When passing these values to a function, the implementer of the function has to remember the meaning of the fields.
- There are no limitation of number and types of fields in a tuple.
- We can define tuple structs to give meaning to the fields.

```rust
struct Point_2D(i32, i32);
struct Point_3D(i32, i32, i32);
```
- Tuple structs are created by specifying the name of the struct followed by the types of the fields in parentheses.

```rust
let p1 = Point_2D(10, 20);
let p2 = Point_3D(3, 4, 5);
```
- Rust compiler will treat the tuple struct as a separate type from a tuple.
- Compiler will make sure that structure of the tuple struct remains as defined.


## Unit Structs
- Unit structs are structs that do not have any fields.
- They are used to define a custom type without any data associated with it.

```rust
struct EmptyStruct;
```