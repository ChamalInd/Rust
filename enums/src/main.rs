#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let mut m = Message::Write(String::from("Hello"));
    m.call();
    m = Message::Move {x: 128, y: 256};
    m.call();
}
