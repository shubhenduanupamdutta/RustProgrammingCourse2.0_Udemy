//------------------------------------------------------------------------
//         ToDo Macro and Some Useful Extensions
//------------------------------------------------------------------------

#[derive(Default)]
struct Student {
    _name_std: String,
    _age: u8,
    _sex: char,
    _country: String,
    _salary: u32,
    _nationality: String,
}

impl Student {
    fn _some_fn_1(&self) -> String {
        todo!()
    }

    fn _some_fn_2(&self) -> String {
        todo!()
    }
}


pub fn main() {
    todo_macro();
}

fn todo_macro() {
    let _student = Student::default();
    // student.some_fn_1();
}