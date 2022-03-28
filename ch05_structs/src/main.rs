struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//tuple struct
struct Point(i32, i32, i32);

mod rectangle_example;

fn main() {
    rectangle_example::invoke();
    let user1 = User {
        username: String::from("dripto"),
        email: String::from("abs@abc.com"),
        sign_in_count: 2,
        active: false
    };

    let name = user1.username;

    let mut user2 = User {
        username: String::from("dripto"),
        email: String::from("abs@abc.com"),
        sign_in_count: 2,
        active: false
    };

    user2.sign_in_count = 3;

    let user3 = build_user(
        "asd.com",
        "dripto"
    );

    let user4 = User {
        email: String::from("email blah"), 
        username: String::from("name whatever"),
        ..user3
    };

    let point = Point(1,2,3);
}

fn build_user(email: &str, name: &str) -> User {
    User {
        email: String::from(email), 
        username: String::from(name),
        active: true,
        sign_in_count: 1
    }
}
