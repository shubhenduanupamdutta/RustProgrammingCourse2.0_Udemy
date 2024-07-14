pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        println!("Creating a circle with radius: {}", radius);
        Circle { radius }
    }

    pub fn new_1(radius: f32) -> Result<Circle, String> {
        if radius >= 0.0 {
            Ok(Circle { radius })
        } else {
            Err("Radius cannot be negative".to_string())
        }
    }

    pub fn new_2(radius: f32) -> Circle {
        match radius {
            ..=0.0 => panic!("Radius cannot be zero or negative"),
            _ => Circle { radius },
        }

        // You shouldn't return a panic, but you can use it to test the code. Usually you handle panic gracefully.
    }

    pub fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

#[cfg(test)]
mod tests {
    use std::error;

    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger = Circle::new(2.0);
        let smaller = Circle::new(1.0);
        assert!(larger.contains(&smaller), "Custom test failure message.");
        // This is same as
        // assert_eq!(larger.contains(&smaller), true, "Custom test failure message");
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger = Circle { radius: 2.0 };
        let smaller = Circle { radius: 1.0 };
        assert!(!smaller.contains(&larger));
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        match Circle::new_1(-11.0) {
            Ok(_) => Err("Circle created with negative radius".to_string()),
            Err(_) => Ok(()),
        }
    }

    #[test]
    #[should_panic(expected="Radius cannot be zero or negative")]
    fn should_not_create_and_panic() {
        let _some_circle = Circle::new_2(-11.0);
    }

    #[test]
    #[ignore]
    fn huge_test_which_will_take_lots_of_time() {
        // Code that runs for hours

    }
}

