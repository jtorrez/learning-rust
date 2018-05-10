// this is valid function
fn five() -> i32 {
    5
}

fn main() {
    let x = five();


    another_function(x);
}

fn another_function (x: i32) {
    println!("x is {}", x);
}
