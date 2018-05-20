// declares the client module here, but contents of module are in the client.rs file
// mod <module> must be defined in lib.rs, rust doesn't know how to look elsewhere for
// module definitions
mod client;

mod network {
    fn connect() {
    }
    mod server {
        fn connect() {
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
