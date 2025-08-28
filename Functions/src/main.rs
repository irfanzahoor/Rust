// ====== Functions Only Demo ======

// Function without parameters
fn say_hello() {
    println!("👋 Hello from Rust!");
}

// Function with parameters
fn greet(name: &str, age: u8) {
    println!("Hello {}, you are {} years old!", name, age);
}

// Function with return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function to check even/odd
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

// Function calling another function
fn number_info(num: i32) {
    println!("Number: {}", num);
    println!("Square: {}", square(num));
    println!("Is Even: {}", is_even(num));
}

// Function to calculate square
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    // Simple hello
    say_hello();

    // Greeting function
    greet("Alice", 25);

    // Addition
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    // Number info (uses multiple functions)
    number_info(7);
    number_info(12);
}
