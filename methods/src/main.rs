#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // creates new instance of self
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // creates new instance of square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
    
    // calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // compares the size
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let rect2 = Rectangle {
        width: 15,
        height: 25
    };

    let rect3 = Rectangle::new(10, 30);

    let squr = Rectangle::square(10);

    println!("Rectangle 1: {rect1:?}");
    println!("Rectangle 2: {rect2:?}");
    println!("Rectangle 3: {rect3:?}");
    println!("Square: {squr:?}");

    println!("Area of the rectangle 1: {}", rect1.area());
    println!("Area of the rectangle 2: {}", rect2.area());
    println!("Area of the rectangle 3: {}", rect3.area());

    println!("Can rectangle 1 holds 2: {}", rect1.can_hold(&rect2));
    println!("Can rectangle 2 holds 1: {}", rect2.can_hold(&rect1));
}
