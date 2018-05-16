#[derive(Debug)]
enum IpAddrVer {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    version: IpAddrVer,
    address: String,
}

fn main() {
    let home = IpAddr {
        version: IpAddrVer::V4,
        address: String::from("127.0.0.1"),
    };

    println!("Home is {:#?}", home);

    let loopback = IpAddr {
        version: IpAddrVer::V6,
        address: String::from("::1"),
    };

    println!("Loopback is {:#?}", loopback);
}
