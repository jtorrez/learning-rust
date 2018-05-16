// valid example of multiple mutable borrows in a program
// data races prevented at COMPILE time
fn main() {
    let mut = String::from("hello");

    {
        let r1 = &mut s;
    }
    // r1 goes out of scope so it is dropped
    // which means we can make new mutable references/borrows
    // with no issues

    let r2 = &mut s;

    // bad example of multiple mutable borrows in a program
    //
    // let mut = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // not valid because both references are in the same scope,
    // i.e., the references could occur simultaneously
}
