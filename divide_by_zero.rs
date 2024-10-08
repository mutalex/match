fn main() {
    let result = divide(10, 2);

    match result {
        Ok(value) => println!("Das Ergebnis ist: {}", value),
        Err(e) => println!("Fehler: {}", e),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division durch Null ist nicht erlaubt"))
    } else {
        Ok(a / b)
    }
}
