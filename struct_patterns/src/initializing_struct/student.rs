#[derive(Debug, Default)]
pub struct Student {
    id: u32,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(name: String) -> Result<Self, String> {
        if name
            .chars()
            .all(|x| matches!(x, 'a'..='z' | 'A'..='Z' | ' '))
        {
            Ok(Self {
                id: 1,
                age: 20,
                name,
            })
        } else {
            Err("Name should only contain alphabets and spaces".to_string())
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

// Custom default implementation
// impl Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             age: 20,
//             name: String::from("Unknown"),
//         }
//     }
// }
