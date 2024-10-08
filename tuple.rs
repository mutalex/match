fn main() {
    let point = (10, 20);

    match point {
        (0, y) => println!("Auf der y-Achse: {}", y),
        (x, 0) => println!("Auf der x-Achse: {}", x),
        (x, y) => println!("Punkt ist bei ({}, {})", x, y),
    }
}
