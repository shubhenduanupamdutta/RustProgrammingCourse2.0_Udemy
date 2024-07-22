// -----------------------------------------------------------------------------
//                   Iterators
// -----------------------------------------------------------------------------


struct Employee {
    name: String,
    _salary: u16,
}

struct EmployeeRecords {
    employee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecords {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

pub fn main() {
    let emp_1 = Employee {
        name: String::from("Employee 1"),
        _salary: 1000,
    };
    let emp_2 = Employee {
        name: String::from("Employee 2"),
        _salary: 2000,
    };
    let emp_db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    for employee in emp_db {
        println!("{employee}");
    }
}
