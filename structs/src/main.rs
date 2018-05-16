struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_new_active_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let email = String::from("heybruv@example.com");
    let username = String::from("jerry_bob");
    build_new_active_user(email, username);
}
