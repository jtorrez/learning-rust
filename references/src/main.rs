// example of dangling pointer that is caught at compile time
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // return value goes out of scope here so it is dropped
  // but the function returns a reference to this dropped
  // value, which would result in a dangling pointer
