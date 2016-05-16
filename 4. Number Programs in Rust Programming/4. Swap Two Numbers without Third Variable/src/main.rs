use std::io;
use std::io::Write;

fn main() {
    let mut first: String = String::new();
    let mut second: String = String::new();

    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first)
        .expect("Cannot read");

    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second)
        .expect("Cannot read");

    let mut first: i32 = first.trim().parse()
        .expect("Cannot parse to number");
    let mut second: i32 = second.trim().parse()
        .expect("Cannot parse to number");

    first = first ^ second;
    second = first ^ second;
    first = first ^ second;

    println!("The first value is: {}, and the second value is: {}",
             first, second);
}
