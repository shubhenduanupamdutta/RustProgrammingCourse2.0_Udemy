//----------------------------------------------------------------

macro_rules! make_struct {
    ($name: ident { $($field: ident: $type: ty), *}) => {
        struct $name {
            $(
                pub $field: $type,
            )*
        }
    };
}

make_struct!(Person {
    name: String,
    age: u32
});

make_struct!(Car {
    make: String,
    model: String,
    year: u32
});

make_struct!(Nothing {});

pub fn main() {
    let person = Person {
        name: String::from("John Doe"),
        age: 30,
    };
    println!("Person: name - {}, age - {}", person.name, person.age);

    let car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2015,
    };
    println!(
        "Car: make - {}, model - {}, year - {}",
        car.make, car.model, car.year
    );

    let _nothing = Nothing {};
}
