use std::io;
use std::io::Write;

fn main() {
    let mut radius: String = String::new();

    print!("Enter the radius of Circle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut radius)
        .expect("Cannot read");
    let radius: f64 = radius.trim().parse()
        .expect("Cannot parse to number");

    println!("Area of Circle: {:.2}", radius * radius * std::f64::consts::PI);
}
