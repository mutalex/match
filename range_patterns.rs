fn main() {
    let number = 6;

    match number {
        1 => println!("Die Zahl ist eins"),
        2..=5 => println!("Die Zahl liegt zwischen 2 und 5"),
        _ => println!("Die Zahl ist größer als 5 oder kleiner als 1"),
    }
}
