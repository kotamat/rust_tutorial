mod sample;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("someusername"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user is : {}", user.username);

    let mut user1 = User {
        username: String::from("someusername"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("anotheremail@example.com");

    build_user(String::from("a"), String::from("h"));

    let user2 = User {
        username: String::from("someuser2"),
        email: String::from("email.com"),
        ..user1
    };

    //     named tuple
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    sample::main();
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

