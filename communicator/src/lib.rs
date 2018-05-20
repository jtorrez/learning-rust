
// name of the module is network
mod network {
    fn connect() {
    }
    // module is defined by mod, not by file
    mod client {
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
