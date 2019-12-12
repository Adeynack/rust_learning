fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {:#?}", user1);

    user1.email = String::from("anotheremail@example.com");
    println!("user1 after mutation of email: {:#?}", user1);

    let user2 = build_user(String::from("asd@example.com"), String::from("Asd"));
    println!("user2: {:#?}", user2);

    let user3 = build_user_from(user1, String::from("user3"));
    println!("user3: {:#?}", user3);
    // println!("user1: {:#?}", user1); // todo: Find a way to borrow user1 for that ..user usage inside of the function

    tuple_strucs();
}

#[derive(Debug)] // allows {:?} formatting
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand possible, since variable and field have same name
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from(u: User, username: String) -> User {
    User { username, ..u }
}

#[derive(Debug)]
struct RGBColor(u8, u8, u8);
#[derive(Debug)]
struct Point3D(f64, f64, f64);

fn tuple_strucs() {
    let color = RGBColor(46, 172, 178);
    let position = Point3D(231.23, 65433.3, 234.653);
    println!("Point at {:?} has color {:?}.", position, color);
}
