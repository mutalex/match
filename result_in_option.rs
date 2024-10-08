fn main() {
    let result = divide(10, 2);

    let result_value: Option<i32> = if let Ok(value) = result {
        Some(value)  // Speichere das Ergebnis in einer Option
    } else {
        None
    };

    // Jetzt kannst du mit `result_value` weiterarbeiten
    if let Some(value) = result_value {
        println!("Gespeichertes Ergebnis: {}", value);
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division durch Null ist nicht erlaubt"))
    } else {
        Ok(a / b)
    }
}
