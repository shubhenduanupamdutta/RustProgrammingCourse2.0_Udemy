//--------------------------------------------------------------------------------------------------
//                 Builder Pattern
//--------------------------------------------------------------------------------------------------
#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Default, Clone)]
enum MembershipType {
    #[default]
    New,
    Casual,
    Loyal,
}

impl Customer {
    // fn new(name: String) -> Self {
    //     Customer{
    //         name,
    //         ..Default::default()
    //     }
    // }

    // fn new_2(name: String, username: String) -> Self {
    //     Customer{
    //         name,
    //         username,
    //         ..Default::default()
    //     }
    // }

    // fn new_3(name: String, username: String, membership: MembershipType) -> Self {
    //     Customer{
    //         name,
    //         username,
    //         membership,
    //         ..Default::default()
    //     }
    // }

    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: MembershipType) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.clone().unwrap_or_default(),
            country: self.country.clone().unwrap_or_default().clone(),
            age: self.age.clone().unwrap_or_default(),
        }
    }
}

pub fn main() {
    // let new_user = Customer::new("John Doe".to_string());
    // println!("New user: {:?}", new_user);

    // let user_with_login = Customer::new_2("John Doe".to_string(), "johndoe".to_string());
    // println!("User with login: {:?}", user_with_login);

    // let loyal_user = Customer::new_3("John Doe".to_string(), "johndoe".to_string(), MembershipType::Loyal);
    // println!("User with login and membership: {:?}", loyal_user);

    let new_user = Customer::new("Chris Evans".to_string()).build();
    println!("New user: {:?}", new_user);

    let user_with_login = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .build();

    println!("User with login: {:?}", user_with_login);

    let loyal_user = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .membership(MembershipType::Loyal)
        .build();
    println!("User with login and membership: {:?}", loyal_user);

    let loyal_user = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .membership(MembershipType::Loyal)
        .build();

    println!("User with login and membership: {:?}", loyal_user);

    let country_user = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .membership(MembershipType::Loyal)
        .country("United States".to_string())
        .build();
    println!("User with login, membership, and country: {:?}", country_user);

    let mut age_user = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .membership(MembershipType::Loyal)
        .country("United States".to_string())
        .age(40)
        .build();
    println!("User with login, membership, country, and age: {:?}", age_user);

    println!(
        "User with login, membership, country, and age:\n
        Name: {}\n,
        Username: {}\n,
        Membership: {:?}\n,
        gender: {}\n,
        Country: {}\n,
        Age: {}",
        age_user.name,
        age_user.username,
        age_user.membership,
        age_user.gender,
        age_user.country,
        age_user.age
    );

    age_user.membership = MembershipType::Casual;

    let gender_user = Customer::new("Chris Evans".to_string())
        .username("chrisevans".to_string())
        .membership(MembershipType::Loyal)
        .country("United States".to_string())
        .gender('M')
        .age(40)
        .build();
    println!("User with gender: {:?}", gender_user);

}
