struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print(&self) {
        println!("active: {}", self.active);
        println!("username: {}", self.username);
        println!("email: {}", self.email);
        println!("sign_in_count: {}", self.sign_in_count);
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user1@example.com");

    user1.print();

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    println!();
    user2.print();
}
