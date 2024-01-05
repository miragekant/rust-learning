struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("robertfripp"),
        email: String::from("k@c.com"),
        sign_in_count: 1,
    };

    let user1 = build_user(String::from("robert"), String::from("kc@abc.com"));

    let user2 = User {
        email: String::from("wellidk"),
        ..user1
    };

    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}
