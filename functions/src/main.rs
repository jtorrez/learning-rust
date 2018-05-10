// this is valid function
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;

    another_function(plus_one(x));
}

fn another_function (x: i32) {
    println!("x is {}", x);
}
