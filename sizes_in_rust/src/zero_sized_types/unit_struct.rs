//--------------------------------------------------------------------------------------------------
//         Unit Struct
//--------------------------------------------------------------------------------------------------

struct Admin;

struct User;

trait Authenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}

impl Authenticate for Admin {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "adminpass"
    }
}


impl Authenticate for User {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "user" && password == "userpass"
    }
}

fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
    role.authenticate(username, password)
}

pub fn main() {
    let admin = Admin;
    let user = User;

    let admin_login = login(admin, "admin", "adminpass");
    let user_login = login(user, "user", "userpass");

    println!("Admin login status, logged_in?: {}", admin_login);
    println!("User login status, logged_in?: {}", user_login);

    let _x = ();
    let _y = _x;    // _x is copied to _y not moved
    let _z = _x;    // _x is copied to _z not moved

    struct ABC;
    let x = ABC;
    let _y = x;   // x is moved to _y
    // let _z = x;  // compiler error because x is moved to _y


}