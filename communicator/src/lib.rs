// declares the client module here, but contents of module are in the client.rs file
// mod <module> must be defined in lib.rs, rust doesn't know how to look elsewhere for
// module definitions
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
