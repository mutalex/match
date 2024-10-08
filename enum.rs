enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Programm beenden"),
        Message::Move { x, y } => println!("Bewege zu x: {}, y: {}", x, y),
        Message::Write(text) => println!("Nachricht: {}", text),
        Message::ChangeColor(r, g, b) => println!("Farbe ändern zu: ({}, {}, {})", r, g, b),
    }
}
