fn main() {
    let result = divide(10, 0);  // Test mit Division durch Null

    // Verarbeite nur das erfolgreiche Ergebnis, ignoriere Fehler
    if let Ok(value) = result {
        println!("Das Ergebnis ist: {}", value);
    }
    // Wenn es ein Err gibt, wird dieser einfach ignoriert.
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division durch Null ist nicht erlaubt"))
    } else {
        Ok(a / b)
    }
}
