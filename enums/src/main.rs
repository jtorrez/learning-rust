#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    println!("Home is {:?}", home);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Loopback is {:?}", loopback);
}
