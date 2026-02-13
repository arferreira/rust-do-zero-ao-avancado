#[derive(Debug)]
struct User {
    name: String,
    email: String,
    created_at: u32,
    active: bool,
}

impl User {
    fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            created_at: 42839,
            active: false,
        }
    }

    fn get_now() -> u32 {
        123456
    }

    fn set_created_at(&mut self) -> bool {
        self.created_at = User::get_now();
        true
    }

    fn is_online(&self) -> bool {
        self.active
    }

    fn set_email(&mut self, email: String) {
        self.email = email
    }

    fn login(&mut self) {
        self.active = true
    }

    fn logout(mut self) -> bool {
        self.active = false;
        self.active
    }
}

fn main() {
    let mut antonio = User::new(String::from("Antonio"), String::from("hello@antonio.land"));

    antonio.name = String::from("Joao");

    println!("name is: {}", antonio.name);

    antonio.set_email(String::from("email@joao.com"));

    antonio.set_created_at();

    println!("{:?}", antonio);

    antonio.login();

    if antonio.is_online() {
        println!("{} is online", antonio.name);
    } else {
        println!("{} is offline", antonio.name);
    }

    antonio.logout();
}
