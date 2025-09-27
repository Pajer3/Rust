enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColour {r: u128, g: u128, b: u128}
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Goodbye"),
        Message::Write(text) => println!("Text: {}", text),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::ChangeColour { r, g, b} => println!("Colour changed to ({},{},{}) ", r, g, b,)
    }
}

fn main() {
    handle_message(Message::Write(String::from("Hello")));
    handle_message(Message::ChangeColour{ g: 29, r: 39, b: 1});
}
