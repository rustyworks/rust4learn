use std::io;
use std::io::Write;

fn main() {
    let mut num: String = String::new();

    print!("Enter the value of num: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num)
        .expect("Cannot read");

    let num: i64 = num.trim().parse()
        .expect("Not a number!");

    let factorial_result: i64 = factorial(num);
    println!("Factorial is {}", factorial_result);
}

fn factorial(n: i64) -> i64 {
    // use no semi colon for expression
    // will return the number
    if n == 0 {
        1 // implicitly: return 1
    } else {
        n * factorial(n - 1)
    }
}
