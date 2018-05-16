// derive annotation to add Debug trait to Rectangle
// so that it can be printed using :? and :#? formatters
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated function, not method
    // call with :: syntax
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // method
    // call with . syntax
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method
    // call with . syntax
    fn can_hold(&self, r2: &Rectangle) -> bool {
        self.width > r2.width && self.height > r2.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of rect 1 is {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(3);

    println!("Look mom, I made a square {:?}", square1);
}
