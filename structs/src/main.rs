#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("Rect1 is a rectanglef {rect1:#?}");
    println!("The are of the rectangle is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
