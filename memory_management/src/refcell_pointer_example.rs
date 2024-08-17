//--------------------------------------------------------------------------------------------------
//              RefCell Example
//--------------------------------------------------------------------------------------------------

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File {
    active_user: i32,
}

#[derive(Debug)]
struct User {
    // To ensure multiple user can have acess to the same file
    file: Rc<RefCell<File>>,
}

pub fn main() {
    let txt_file = Rc::new(RefCell::new(File { active_user: 0 }));

    let user_1 = User {
        file: Rc::clone(&txt_file),
    };
    user_1.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file.borrow().active_user);

    let user_2 = User {
        file: Rc::clone(&txt_file),
    };
    user_2.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file.borrow().active_user);

    // Although there are multiple user, we are able to change the internal value.

    // Because of multiple owners, the any update of data by one owner is visible to all the owners.
    // In this case user_1 updates the data, but updated data is visible via txt_file instance to user_2 also.
    // Also note that `user_`.file.borrow_mut().active_user += 1; is mutable borrow. But it is restricted to this line and dropped after that because it is not assigned to any variable.
}
