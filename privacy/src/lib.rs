pub mod outermost {
    pub fn middle_function () {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function () {}
    }
}

fn try_me() {
    outermost::middle_function(); // no errors, middle_function is public
    outermost::middle_secret_function(); // error, middle_secret_function is private
    outermost::inside::inner_function(); // error, inside module is private
    outermost::inside::secret_function(); // error, inside module AND function are private
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
