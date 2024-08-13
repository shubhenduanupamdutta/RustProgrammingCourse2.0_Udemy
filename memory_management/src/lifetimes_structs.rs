// --------------------------------------------
//            Lifetimes and Structs
// --------------------------------------------

struct ArrayProcessor<'a> {
    data: &'a [i32],
}

impl<'a> ArrayProcessor<'a> {
    fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        let previous_data = self.data;
        self.data = new_data;
        &previous_data
    }
}

pub fn main() { 
    let mut some_data = ArrayProcessor {
        data: &[1, 2, 3, 4, 5],
    };
    let previous_data = some_data.update_data(&[6, 7, 8, 9, 10]);
    println!("Previous data: {:?}", previous_data);
    println!("Current data: {:?}", some_data.data);
}