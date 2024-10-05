//----------------------------------------------------------------------
//           String Literal
//----------------------------------------------------------------------

pub fn main() {
    let new_str = "The man said \"Hello World!\"";
    println!("{}", new_str);

    println!();
    println!("Using raw string literals");
    let new_str = r#"The man said "Hello World!""#;
    println!("{}", new_str);

    println!();
    println!("Use Case: JSON String");
    let json_str = "{
        \"name\": \"John Doe\",
        \"age\": 30,
        \"city\": \"New York\"
    }";
    println!("Using escaped string literals: {}", json_str);
    let json_str = r#"{
        "name": "John Doe",
        "age": 30,
        "city": "New York"
    }"#;
    println!("Using raw string literals: {}", json_str);

    println!();
    println!("Using '#' with raw string literals");
    let new_str = r##"Inserting # in # the string"##;
    println!("{}", new_str);
}