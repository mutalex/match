fn main() {
    let number = Some(4);

    match number {
        Some(n) if n < 5 => println!("Die Zahl ist kleiner als 5: {}", n),
        Some(n) => println!("Die Zahl ist: {}", n),
        None => println!("Keine Zahl vorhanden."),
    }
}
