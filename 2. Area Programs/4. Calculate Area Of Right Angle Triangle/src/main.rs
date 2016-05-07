use std::io;
use std::io::Write;

struct Triangle {
    base: f32,
    height: f32,
}

impl Triangle {
    fn area(&self) -> f32 {
        self.base * self.height * 0.5
    }
}

fn main() {
    let mut base: String = String::new();
    let mut height: String = String::new();

    print!("Enter the base of Right Angle Triangle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base).expect("Cannot read");

    print!("Enter the height of Right Angle Triangle: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut height).expect("Cannot read");

    let triangle = Triangle {
        base: base.trim().parse().expect("Not a number"),
        height: height.trim().parse().expect("Not a number"),
    };

    println!("Area of Right Angle Triangle: {}", triangle.area());
}
