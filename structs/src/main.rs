// Structs are similar to tuples
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuples Structs with named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// Unit-Like Structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: true,
        username: String::from("anotherusername456"),
        email: String::from("anotheremail@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ...user1,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
