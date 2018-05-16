fn main() {
    let s1 = gives_ownership(); // value returned by gives_ownership is now owned by s1

    let s2 = String::from("hello"); // s2 owns the variable returned by the from method

    let s3 = takes_and_gives_back(s2); // ownership of the value attached to s2
                                       // is given to the function and then
                                       // returned to be owned by s3
}
// s1 and s3 are dropped since they go out of scope
// nothing special happens with s2 because ownership was moved

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
} // nothing special happens here because ownership of some_string is passed out
  // of the function

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // nothing special happens here because ownership of a_string is passed to the
  // function caller
