fn main() {
    let result = divide(10, 0);  // Teste mit Division durch Null

    match result {
        Ok(value) => println!("Das Ergebnis ist: {}", value),
        Err(_) => (),  // Ignoriere den Fehler, indem du nichts machst
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division durch Null ist nicht erlaubt"))
    } else {
        Ok(a / b)
    }
}
