fn five() -> i32 {
    5
}

fn plus_one(mut x: i32) -> i32 {
    x += 1;
    return x;
}

fn main() {
    println!("Hello, world!");
    another(10, 'm');
    let x = five();
    println!("This is the value of x : {x}");
    let x = plus_one(x);
    println!("This is the value of x : {x}");
}

fn another(x: i32, unit: char) {
    println!("Another function hello");
    println!("Hello number x : {x}{unit}");
}
