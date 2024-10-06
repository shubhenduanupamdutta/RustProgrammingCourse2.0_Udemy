//----------------------------------------------------------------------
//           Efficient Programming Tips
//----------------------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    _age: u8,
}

pub fn main() {
    let cancer = true;
    let smoking = false;

    // match cancer {
    //     true => match smoking {
    //         true => println!("Your cancer is likely caused by smoking."),
    //         false => println!("Your cancer is not caused by smoking. You should consult your doctor for further advice."),
    //     },
    //     false => match smoking {
    //         true => println!("Smoking is dangerous and may cause cancer. You should consult your doctor for further advice."),
    //         false => println!("You don't have any disease."),
    //     }
    // }

    match (cancer, smoking) {
        (true, true) => println!("Your cancer is likely caused by smoking."),
        (true, false) => println!("Your cancer is not caused by smoking. You should consult your doctor for further advice."),
        (false, true) => println!("Smoking is dangerous and may cause cancer. You should consult your doctor for further advice."),
        (false, false) => println!("You don't have any disease."),
    }

    println!();
    println!("Using collect to convert a vector of Results into a Result of a vector");
    using_collect();

    println!();
    println!("Organizing hash maps");
    organizing_hash_maps();

    println!();
    println!("Using reverse");
    using_reverse();

}

fn using_collect() {
    let responses = vec![Ok(100), Err("Client Error"), Ok(300), Err("Server Error")];
    let result = responses.into_iter().collect::<Result<Vec<_>, _>>();
    println!("{:?}", result);
}

fn organizing_hash_maps() {
    let person_1 = Person {
        name: "Joseph".to_string(),
        _age: 25,
    };

    let person_2 = Person {
        name: "Jane".to_string(),
        _age: 30,
    };

    let person_3 = Person {
        name: "Michael".to_string(),
        _age: 35,
    };

    let persons = vec![person_1, person_2, person_3];

    let person_hash = persons_by_name(persons);
    println!("{:?}", person_hash);

    for (name, details) in &person_hash {
        println!("Person {:?} has details {:?}", name, details);
    }
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    persons
        .into_iter()
        .map(|person| (person.name.clone(), person))
        .collect()
}

fn using_reverse() {
    println!();
    println!("When using normal range operator '0..10'");
    for i in 0..10 {
        println!("{}", i);
    }

    println!();
    println!("When using reverse range operator '10..0'");
    for i in 10..0 {
        println!("{}", i);
    }
    println!("The output is empty because the range operator '10..0' is not valid.");

    println!();
    println!("For reverse range, use '0..10.rev()' instead");
    for i in (0..10).rev() {
        println!("{}", i);
    }


}