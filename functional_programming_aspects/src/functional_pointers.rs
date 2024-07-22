// -----------------------------------------------------------------------------
//                  Functional Pointers
// -----------------------------------------------------------------------------

struct User {
    name: String,
    age: u8,
    _salary: u32,
}

fn is_valid_user(
    name: &str,
    banned_user_name: &str,
    age: u8, 
    simple_validator: fn(&str, &str) -> bool, advance_validator: fn(u8) -> bool,
) -> bool
{
    simple_validator(name, banned_user_name) && advance_validator(age)
}

fn validate_user_simple(name: &str, banned_user_name: &str) -> bool {
    name.len() != 0 && name != banned_user_name
}

fn validate_user_advanced(age: u8) -> bool {
    age >= 30
}

pub fn main() {
    let person_1 = User {
        name: String::from("Some One"),
        age: 35,
        _salary: 100_000,
    };

    // let validate_user_simple = |name: &str| name.len() != 0;
    // let validate_user_advanced = |age: u8| age >= 30;

    let banned_user = String::from("Banned User");

    println!(
        "User Validity: {}",
        is_valid_user(
            &person_1.name,
            &banned_user,
            person_1.age,
            validate_user_simple,
            validate_user_advanced
        )
    )
}
