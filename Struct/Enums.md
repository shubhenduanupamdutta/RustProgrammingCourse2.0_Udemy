# Enums

#### Enums allows us to define a type by enumerating its possible values. 

```rust
enum IpAddrKind {
    V4,
    V6,
}
```
- We don't need to define a type for each of the possible values, they are all of the same type, `IpAddrKind`.
- We can get to a value of `IpAddrKind` by using the `::` syntax.
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- This is similar to defining a struct, but instead of defining fields, we define possible values.
- We can also define methods on enums.
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn call(&self) -> String {
        match self {
            IpAddr::V4(ip) => format!("V4: {}", ip),
            IpAddr::V6(ip) => format!("V6: {}", ip),
        }
    }
}
```
