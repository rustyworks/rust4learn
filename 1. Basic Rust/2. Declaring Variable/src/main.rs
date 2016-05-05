fn main() {
    let immutable_variable = 10;
    let immutable_variable_with_type: i32 = 42;
    let mut mutable_variable = 99;

    println!("immutable_variable: {}", immutable_variable);
    println!("immutable_variable_with_type: {}", immutable_variable_with_type);
    println!("mutable_variable: {}", mutable_variable);

    mutable_variable = 255;
    println!("after change the mutable_variable: {}", mutable_variable);
}
