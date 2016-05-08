use std::io;
use std::io::Write;

fn main() {
    let mut side: String = String::new();

    print!("Enter the length of side: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut side)
        .expect("Cannot read");
    let side: f64 = side.trim().parse()
        .expect("Cannot parse to number");

    println!("Area of Square: {:.2}", side * side);
}
