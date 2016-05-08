use std::io;
use std::io::Write;

fn main() {
    let mut length: String = String::new();
    let mut breadth: String = String::new();

    print!("Enter the length of rectangle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut length)
        .expect("Cannot read");
    let length: f64 = length.trim().parse()
        .expect("Cannot parse to number");

    print!("Enter the breadth of rectangle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut breadth)
        .expect("Cannot read");
    let breadth: f64 = breadth.trim().parse()
        .expect("Cannot parse to number");

    println!("Area of rectangle: {:.2}", breadth * length);
}
