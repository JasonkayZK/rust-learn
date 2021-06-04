// Define
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Init
    let user1 = User {
        username: String::from("jasonkayzk"),
        email: String::from("jasonkayzk@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{:?}", user1);

    // Build a struct
    let mut user2 = build_user(
        String::from("271226192@qq.com"),
        String::from("jasonkayzk"),
    );
    user2.username = String::from("jasonkay");
    println!("{:?}", user2);

    // Update to create a struct
    let user3 = update_user(
        String::from("other@gmail.com"),
        String::from("jasonkay"),
        user1,
    );
    println!("{:?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // equals to `email: email,`
        username, // Equals to `username: username,`
        active: true,
        sign_in_count: 1,
    }
}

fn update_user(email: String, username: String, user_tpl: User) -> User {
    User {
        email, // equals to `email: email,`
        username, // Equals to `username: username,`
        ..user_tpl
    }
}
