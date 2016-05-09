use std::io;
use std::io::Write;

fn main() {
    let mut number: String = String::new();

    print!("Enter any number to be reversed: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number)
        .expect("Failed to read");

    let mut number: i32 = number.trim().parse()
        .expect("Not a number");
    let mut remainder: i32;
    let mut reversed: i32 = 0;

    while number >= 1 {
        remainder = number % 10;
        reversed = reversed * 10 + remainder;
        number = number / 10;
    }

    println!("Reversed Number: {}", reversed);
}
