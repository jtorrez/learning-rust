fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & denotes borrow

    println!("The length of '{}' is {}.", s1, len);
}

// calculate length borrows (read only access) s so that
// ownership of s is not passed to the function and s
// is not dropped when it goes out of scope in the function
fn calculate_length(s: &String) -> usize {
    s.len()
}
