fn main() {
    let s = String::from("Hello, world!");

    let fw = first_word(&s);

    println!("The first word is {}", fw);

    // s.clear()
    // ^ results in an error because first_word makes
    // an immutable borrow and s.clear() is a mutable borrow
    // which can't co-occur in the same scope
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // returning here without explicit return keyword
}
