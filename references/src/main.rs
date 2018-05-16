// example of invalid simultaneous borrows of different kinds
fn main() {
    let mut s = String::from("hello");

    // Invalide because there are multiple references in the
    // same scope, which COULD occur simultaneously and result
    // in a data race. The race in this example is the mutation
    // of r3 could occur before the immutable reference/borrow
    // despite the ordering of the statements
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
}
