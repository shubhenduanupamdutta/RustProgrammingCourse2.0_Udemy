// -----------------------------------------------------------------------------
//                        Closures
// -----------------------------------------------------------------------------

struct User {
    name: String,
    age: u8,
    _salary: u32,
}

// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

pub fn main() {
    let person_1 = User {
        name: String::from("Some One"),
        age: 35,
        _salary: 100_000,
    };

    // let validate_user = |name: &str| name.len() != 0;

    let banned_user_name = String::from("Banned User");
    let validate_user = |name: &str| {
        let banned_user = &banned_user_name;  // Transferring ownership of banned_user_name, hence FnOnce
        name.len() != 0 && name != banned_user
    };

    let mut banned_user_name = String::from("Banned User");
    let _validate_user_mut = |name: &str| {
        let banned_user = &mut banned_user_name; // Mutable borrow, hence FnMut
        name.len() != 0 && name != banned_user
    };

    println!("User Validity: {}", validate_user(&person_1.name));

    let validate_user_advance = |age: u8| age >= 30;

    let validity = is_valid_user(
        &person_1.name,
        person_1.age,
        validate_user,
        validate_user_advance,
    );

    println!("User Validity using both validators: {}", validity);
}
