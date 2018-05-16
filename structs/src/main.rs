struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_new_active_user(email: String, username: String) -> User {
    // uses struct init shorthand
    // if the variables are the same name as the struct field
    // you don't have to explicitly write the struct field name
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_user(user1: User, email: String, username:String) -> User {
    // uses struct update syntax, the ..user1 says to use the values
    // for user1 for any struct fields not explicitly named in the new
    // user struct
    let user2 = User {
        email,
        username,
        ..user1
    };
    user2
}

fn main() {
    let email = String::from("heybruv@example.com");
    let username = String::from("jerry_bob");
    build_new_active_user(email, username);
}
