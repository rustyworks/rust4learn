use std::io;
use std::io::Write;

fn main() {
    const PI: f64 = 3.14;
    let mut radius: String = String::new();

    print!("Enter radius of circle: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut radius)
        .expect("Failed to read");

    let radius: f64 = radius.trim().parse()
        .expect("Please input a number");

    let area: f64 = PI * radius * radius;
    let circumference: f64 = PI * 2.0 * radius;

    println!("Area of circle: {}", area);
    println!("Circumference of circle: {}", circumference);
}
