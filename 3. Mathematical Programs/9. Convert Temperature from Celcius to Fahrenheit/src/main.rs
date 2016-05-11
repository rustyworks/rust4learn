use std::io;
use std::io::Write;

fn main() {
    let mut celcius: String = String::new();

    print!("Enter temp in Celcius: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut celcius)
        .expect("Cannot read");
    let celcius: f64 = celcius.trim().parse()
        .expect("Cannot parse to number");

    // Should add the zero for implicit parse the 32 to float
    println!("Temperature in Fahrenheit: {}", (1.8 * celcius) + 32.0);
}
