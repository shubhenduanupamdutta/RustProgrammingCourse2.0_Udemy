// -----------------------------------------------------------------
//                  Struct and its Types
// -----------------------------------------------------------------

fn main() {
    let mut my_car = Car {
        owner: String::from("Shubhendu"),
        year: 2020,
        fuel_level: 50.0,
        price: 5_000,
    };

    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    
    let extracted_owner = my_car.owner.clone(); // ownership will pass to extracted_owner if clone is not used (called Partial move)

    let another_car = Car {
        owner: "new_name".to_string(),
        ..my_car // struct update syntax
        // We need to be aware of partial move
    };


    // Tuple Structs
    let point_2D = (1, 3);
    let point_3D = (1, 3, 5);

    struct Point2D(i32, i32);
    struct Point3D(i32, i32, i32);

    let point1 = Point2D(1, 3);
    let point2 = Point3D(1, 3, 5);

    // Unit Struct
    struct ABC;

}

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}
