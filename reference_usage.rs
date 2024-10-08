fn main() {
    let number = Some(10);
    
    match &number {
        Some(value) => println!("Die Zahl per Referenz ist: {}", value),
        None => println!("Keine Zahl vorhanden."),
    }
}
