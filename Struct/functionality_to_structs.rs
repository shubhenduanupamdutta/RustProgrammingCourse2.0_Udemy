struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn display_car_info(&self) {
        println!(
            "This car is owned by {}. It was made in {} and has a fuel level of {}.",
            self.owner, self.year, self.fuel_level
        );
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        println!("The car has been sold!");
        self
    }

    fn monthly_insurance() -> u32 {
        100
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Car {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 10000,
        }
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("John"),
        year: 2010,
        fuel_level: 0.5,
        price: 10000,
    };

    my_car.display_car_info();

    my_car.refuel(10.0);

    let new_owner = my_car.sell();
    println!("The new owner of the car is {}", new_owner.owner);
    // my_car.refuel(10.0); Throw an error because my_car is moved to new_owner
    // my_car.display_car_info();

    let new_car = Car::new(String::from("Alice"), 2015);
    new_car.display_car_info();
}
