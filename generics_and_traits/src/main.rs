// ---------------------------------------------------------------------------
//                   Generics and Traits
// ---------------------------------------------------------------------------

fn main() {
    println!("############## Generics ##############");
    generics();

    println!("\n############## Traits ##############");
    traits();

    println!("\n############## Trait Bounds ##############");
    trait_bounds();

    println!("\n############## Super Traits ##############");
    super_traits();

    println!("\n############## Trait Objects ##############");
    trait_objects();

    println!("\n############## Derived and Marker Traits ##############");
    derived_and_marker_traits();

    println!("\n############## Associated Types in Traits ##############");
    associated_types();

    println!("\n############## Choosing Associated Types vs Generic Types ##############");
    choosing_associated_types_vs_generic_types();
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn printing(&self) {
        println!(
            "The value of the coordinates are: x = {}, y = {}",
            self.x, self.y
        );
    }

    fn new_1(x: i32, y: i32) -> Point<i32, i32> {
        Point { x, y }
    }
}

impl Point<f64, f64> {
    fn printing(&self) {
        println!(
            "The value of the coordinates are: x = {}, y = {}",
            self.x, self.y
        );
    }

    fn new_1(x: f64, y: f64) -> Point<f64, f64> {
        Point { x, y }
    }
}

// Restrict T and U to only numbers
fn add_points<T: std::ops::Add<Output = T> + Copy, U: std::ops::Add<Output = U> + Copy>(
    p1: &Point<T, U>,
    p2: &Point<T, U>,
) -> Point<T, U> {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn generics() {
    let origin = Point { x: 0, y: 0 };
    let p1 = Point::new(1.1, 2.3);
    let p2 = Point::new(1, 2.3);
    let p3: Point<u8, f32> = Point::new(6, 8.9);
    println!("origin: {:?}", origin);
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    origin.printing();
    p1.printing();

    add_points(&origin, &origin);
    add_points(&p3, &p3);
}

//-------------------------------------------------------
//                    Traits
//-------------------------------------------------------

struct DrawingInfo {
    line_width: u8,
    color: String,
}

struct Square {
    side: f32,
    info: DrawingInfo,
}

struct Rectangle {
    length: f32,
    width: f32,
    info: DrawingInfo,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area of the square is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

// Draw being super trait of Shape
trait Shape: Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented for this shape, returning dummy value.");
        0.0
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("The area of the square is: {}", area_of_square);
        area_of_square
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing square with side: {}", self.side);
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!(
            "Drawing rectangle with length: {}, width: {}",
            self.length, self.width
        );
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        println!("The area of the rectangle is: {}", area_of_rectangle);
        area_of_rectangle
    }

    // Will override default implementation
    fn perimeter(&self) -> f32 {
        let perimeter_of_rectangle = 2.0 * (self.length + self.width);
        println!(
            "The perimeter of the rectangle is: {}",
            perimeter_of_rectangle
        );
        perimeter_of_rectangle
    }
}

fn traits() {
    let r1 = Rectangle {
        length: 10.0,
        width: 5.0,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("blue"),
        },
    };

    let s1 = Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("red"),
        },
    };

    r1.area();
    s1.area();

    s1.perimeter();
    r1.perimeter();
}

//-------------------------------------------------------
//                    Trait Bounds
//-------------------------------------------------------

fn shape_properties<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}

// Impl Trait Syntax
fn return_area(object: impl Shape) -> f32 {
    object.area()
}

// Where clause
fn return_perimeter<T>(object: T) -> f32
where
    T: Shape,
{
    object.perimeter()
}

fn returns_shape() -> impl Shape {
    Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 2,
            color: String::from("Red"),
        },
    }
}

struct Circle {
    radius: f32,
}

fn get_rectangle_and_square() -> (Rectangle, Square) {
    let r1 = Rectangle {
        length: 10.0,
        width: 5.0,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("blue"),
        },
    };

    let s1 = Square {
        side: 5.0,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("red"),
        },
    };

    (r1, s1)
}

fn trait_bounds() {
    let (r1, s1) = get_rectangle_and_square();

    shape_properties(r1);
    shape_properties(s1);

    let c1 = Circle { radius: 5.0 };
    // shape_properties(c1); // This will give an error as Circle does not implement Shape trait
    let (r1, s1) = get_rectangle_and_square();
    return_area(r1);
    return_perimeter(s1);
}

//-------------------------------------------------------
//                    Super Traits
//-------------------------------------------------------

trait Draw {
    fn draw_object(&self);
}

trait OtherTrait {}
trait SomeOtherTrait {}

impl OtherTrait for Rectangle {}
impl SomeOtherTrait for Square {}
impl SomeOtherTrait for Rectangle {}
impl OtherTrait for Square {}

trait BaseTrait: OtherTrait + SomeOtherTrait {}

impl BaseTrait for Rectangle {}
impl BaseTrait for Square {}

// Instead of using
// fn some_function<T>(object: T)
// where
//     T: BaseTrait + OtherTrait + SomeOtherTrait,
// {
//     // Do something
// }

// We can use
fn some_function<T>(object: T)
where
    T: BaseTrait,
{
    // Do something
}
// Since BaseTrait has `super traits` OtherTrait and SomeOtherTrait, we can use BaseTrait instead of using all three traits

fn super_traits() {}

//-------------------------------------------------------
//                    Trait Objects
//-------------------------------------------------------

fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}

// Throws an error as we cannot return a trait object
// fn returns_shape_dynamic(dimension: Vec<f32>) -> impl Shape {
//     if dimension.len() == 1 {
//         Square {
//             side: dimension[0],
//             info: DrawingInfo {
//                 line_width: 2,
//                 color: String::from("Red"),
//             }
//         }
//     } else {
//         Rectangle {
//             length: dimension[0],
//             width: dimension[1],
//             info: DrawingInfo {
//                 line_width: 1,
//                 color: String::from("blue"),
//             },
//         }
//     }

// }

fn returns_shape_dynamic(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        Box::new(Square {
            side: dimension[0],
            info: DrawingInfo {
                line_width: 2,
                color: String::from("Red"),
            },
        })
    } else {
        Box::new(Rectangle {
            length: dimension[0],
            width: dimension[1],
            info: DrawingInfo {
                line_width: 1,
                color: String::from("blue"),
            },
        })
    }
}

trait Vehicle {
    fn start_engine(&self) -> String;
    fn drive(&self) -> String;
}

struct Car;
struct Bicycle;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "🚗 Engine Started".to_string()
    }

    fn drive(&self) -> String {
        "🚗 Driving the car".to_string()
    }
}

impl Vehicle for Bicycle {
    fn start_engine(&self) -> String {
        "🚲 No engine to start".to_string()
    }

    fn drive(&self) -> String {
        "🚲 Pedaling the bicycle".to_string()
    }
}

fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "car" => Box::new(Car),
        "bicycle" => Box::new(Bicycle),
        _ => panic!("No vehicle of this type available"),
    }
}

fn operate_vehicle(driver: &dyn Vehicle) {
    println!("{}", driver.start_engine());
    println!("{}", driver.drive());
}

fn trait_objects() {
    let (r1, s1) = get_rectangle_and_square();

    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));

    println!("\n########## Driving and Vehicle ##########\n");
    let car = get_vehicle("car");
    let bicycle = get_vehicle("bicycle");
    let car2 = get_vehicle("car");

    let vehicles: Vec<&dyn Vehicle> = vec![&*car, &*bicycle, &*car2];

    for vehicle in vehicles {
        operate_vehicle(vehicle);
    }
}

//-------------------------------------------------------
//                    Derived and Marker Traits
//-------------------------------------------------------

// Marker Trait
trait Properties: PartialEq + Default + Clone {}

// Derived Traits (Debug and PartialEq) using derive attribute
#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

impl Properties for Student {}


fn derived_and_marker_traits() {
    let s1 = Student {
        name: String::from("ABC"),
        age: 35,
        sex: 'M',
    };

    let s2 = Student {
        name: String::from("XYZ"),
        age: 40,
        sex: 'F',
    };

    println!("Student 1: {:?}", s1);

    println!("s1 and s2 are equal: {}", s1 == s2);
}

// ---------------------------------------------------------------------------
//                Associated Types in Traits
// ---------------------------------------------------------------------------

// Suppose I want to calculate how far my car will travel in 3 hours, if it is going at certain speed.

#[derive(Debug)]
struct Km {
    value: u32,
}

// Kmh is Kilometers per hour
#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     // self represents Kmh and represents speed
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

// Let's use trait for the same
trait DistanceThreeHours {
    type Distance;
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Km {
        Km {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Miles {
        Miles {
            value: self.value * 3,
        }
    }
}

fn associated_types() {
    let speed_kmh = Kmh { value: 60 };
    let distance_km = speed_kmh.distance_in_three_hours();

    println!("At {:?} Kmh, the car will travel {:?} Km in 3 hours", speed_kmh, distance_km);

    let speed_mph = Mph { value: 40 };
    let distance_miles = speed_mph.distance_in_three_hours();

    println!("At {:?} Mph, the car will travel {:?} Miles in 3 hours", speed_mph, distance_miles);
}

// ---------------------------------------------------------------------------
//                Choosing Associated Types vs Generic Types
// ---------------------------------------------------------------------------

// trait Addition {
//     type Rhs;
//     type Output;
//     fn add(&self, rhs: Self::Rhs) -> Self::Output;
// }

trait Addition<Rhs> {
    type Output;
    fn add(&self, rhs: Rhs) -> Self::Output;
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Addition<Point2> for Point2 {
    type Output = Point2;
    fn add(&self, rhs: Point2) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32> for Point2 {
    type Output = Point2;
    fn add(&self, rhs: i32) -> Self::Output {
        Point2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point2,
    end: Point2,
}

fn choosing_associated_types_vs_generic_types() {
    let p1 = Point2 { x: 1, y: 2 };
    let p2 = Point2 { x: 3, y: 4 };

    let p3 = p1.add(p2);
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);

    let p1 = Point2 { x: 1, y: 2 };
    let p3 = p1.add(3);

    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 5);

}