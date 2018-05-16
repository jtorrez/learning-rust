#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// could define each of the enums as their own structs
// but this gives the flexibility of being able to pass
// a single Message type to a function/method rather
// than having to have functions for each Message type
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// you can also implement methods on enums
// impl Message {
//     fn call(&self) {
//     method body here
//     }
// }

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    println!("Home is {:?}", home);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Loopback is {:?}", loopback);

    // let some_number = Some(5);
}
