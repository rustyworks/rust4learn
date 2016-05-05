use std::io;
use std::io::Write;

// I decide to try using struct and impl
struct Triangle {
    side: f64,
    r_4: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        self.r_4 * self.side * self.side
    }
}

fn main() {
    let mut side: String = String::new();

    print!("Enter the Length of Side: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut side)
        .expect("Cannot read");
    let side: f64 = side.trim().parse()
        .expect("Cannot parse to number");

    let triangle = Triangle {
        side: side,
        // Finally I found how to convert explicit the number
        r_4: (3.0_f64).sqrt() / 4.0,
    };

    println!("{:.2}", triangle.area());
}
