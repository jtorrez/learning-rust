fn main() {
    let mut s = String::from("hello"); // must denote variable as mutable in definition to allow mutable borrows

    change(&mut s); // &mut denotes mutable borrow (write access)
}
// s goes out of scope here and is dropped

// change takes a mutable reference (write access) to
// s and mutates it
fn change(some_string: &mut String) {
    some_string.push_str(", world");
} // nothing special happens here because some_string was
  // borrowed and not owned by change
