// -----------------------------------------
//             Enums
// -----------------------------------------

// enum WeekDay {
//     Sunday,
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
// }

// fn main() {
//     let mut day: String = "Saturday".to_string();

//     // We can also define a vector of strings to represent the days of the week
//     let week_day = vec![
//         "Sunday".to_string(),
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//     ]

//     // Now we can call a day using the index of the vector
//     day = week_day[0].clone();
//     // But this will require us to remember the index of each day

//     let day = WeekDay::Saturday;
// }

enum TravelType {
    Car(f32),
    Train(f32),
    Airplane(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles)=> miles * 3.0,
            TravelType::Airplane(miles) => miles * 5.0,
        };
        allowance
    }
}

fn main() {
    let participant = TravelType::Car(100.0);
    println!{"The travel allowance for the participant is {}", participant.travel_allowance()};
}
