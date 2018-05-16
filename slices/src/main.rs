fn main() {
    // create the variable my_string which is of String type
    let my_string = String::from("hello world");

    // API of first_word allows passing of all string slices
    let word = first_word(&my_string[..]);

    // variable of type &str, a string slice
    let my_string_literal = "hello world";

    // pass an explicit string slice of the string literal
    let word = first_word(&my_string_literal[..]);

    // type of string literals is implicitly &str, so this
    // works too
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // returning here without explicit return keyword
}
