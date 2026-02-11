#[derive(Debug)]
struct User {
    name: String,
    email: String,
    created_at: u32,
    active: bool,
}

fn main() {
    let mut antonio = User {
        name: String::from("Antonio"),
        email: String::from("hello@antonio.land"),
        created_at: 12345487,
        active: true,
    };

    antonio.name = String::from("Joao");

    println!("name is: {}", antonio.name);

    if is_online(&antonio) {
        println!("{} is online", antonio.name);
    } else {
        println!("{} is offline", antonio.name);
    }
}

fn is_online(user: &User) -> bool {
    user.active
}
