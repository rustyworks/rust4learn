use std::io;
use std::io::Write;
use std::f64::consts::PI; // We can use std::f32::consts::PI if using float

fn main() {
    let mut side_1: String = String::new();
    let mut side_2: String = String::new();
    let mut angle: String = String::new();

    print!("Enter first side length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut side_1)
        .expect("Failed to read");

    print!("Enter second side length: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut side_2)
        .expect("Failed to read");

    print!("Enter the angle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut angle)
        .expect("Failed to read");

    let side_1: f64 = side_1.trim().parse()
        .expect("Parsing error");
    let side_2: f64 = side_2.trim().parse()
        .expect("Parsing error");
    let angle: f64 = angle.trim().parse()
        .expect("Parsing error");
    // I have to calculate and write the type the pi angle first before "sin" it
    let pi_angle: f64 = (PI / 180.0) * angle;

    // This must the same type (f64)
    let area: f64 = (side_1 * side_2 * pi_angle.sin()) / 2.0;
    println!("Area of Scalene triangle: {:.5}", area); // Round to 5 digits
}
