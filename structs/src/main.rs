// the User struct is the owner of all it's data and the
// memory of the data in it's field won't be dropped until
// the struct is dropped. If the types of, say username for
// example, were &str, then User would no longer own it's data
// and the compiler would expect a lifetime parameter with
// that field
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// named tuples, act as tuples but now have defined type
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

    // black and origin now have different types
    // due to using the tuple structs vs. them being
    // untyped if we didn't use the tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
