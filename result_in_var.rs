fn main() {
    let number = Some(10);

    let value_for_print: i32 = match number {
        Some(value) => {
            // Mehrere Anweisungen im Block
            println!("Die Zahl ist: {}", value);  // Erste Anweisung
            let doubled_value = value * 2;  // Zweite Anweisung
            println!("Der doppelte Wert ist: {}", doubled_value);  // Dritte Anweisung
            doubled_value  // Dieser Wert wird am Ende des Blocks zurückgegeben
        },
        None => {
            println!("Keine Zahl vorhanden.");  // Fehlermeldung bei None
            0  // Standardwert zurückgeben
        }
    };

    println!("value_for_print: {}", value_for_print);
}
