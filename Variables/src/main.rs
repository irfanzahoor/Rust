// ===== Rust Data Types Cheat Sheet =====

fn main() {
    // -------- Scalar Types --------

    // 1. Integers
    let a: i32 = -42;   // signed 32-bit integer
    let b: u64 = 100;   // unsigned 64-bit integer (sirf positive values)

    // 2. Floating-point numbers
    let pi: f64 = 3.1415; // 64-bit floating point (default)
    let small: f32 = 2.5; // 32-bit floating point

    // 3. Boolean
    let is_rust_fun: bool = true; // true/false

    // 4. Character
    let letter: char = 'R';  // single character
    let emoji: char = '😎';  // Unicode supported (emoji bhi)

    // -------- Compound Types --------

    // 5. Tuple (different types ek saath)
    let tup: (i32, f64, char) = (100, 6.9, 'Z');
    let (x, y, z) = tup; // destructuring
    println!("Tuple destructured -> x = {}, y = {}, z = {}", x, y, z);

    // 6. Array (same type, fixed size)
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("Array first element = {}", arr[0]);

    // 7. Slice (array ka ek part)
    let slice = &arr[1..3]; // elements 1 and 2 (2,3)
    println!("Slice of array = {:?}", slice);

    // -------- Strings --------

    let s1: &str = "Hello";             // string literal (immutable, fixed)
    let s2: String = String::from("Rust"); // growable, heap-allocated
    println!("String literal: {}", s1);
    println!("Growable String: {}", s2);

    // -------- Special Types --------

    // 8. Unit type `()` (no value, used when function returns nothing)
    let unit: () = (); 
    println!("Unit type: {:?}", unit);

    // 9. Option<T> (safe null handling)
    let some_num: Option<i32> = Some(42);
    let no_num: Option<i32> = None;
    println!("Option with value: {:?}", some_num);
    println!("Option None: {:?}", no_num);

    // 10. Result<T, E> (error handling)
    let ok_result: Result<i32, &str> = Ok(200);
    let err_result: Result<i32, &str> = Err("Something went wrong");
    println!("Result Ok: {:?}", ok_result);
    println!("Result Err: {:?}", err_result);

    // -------- Print Scalars --------
    println!("Integer a = {}, Unsigned b = {}", a, b);
    println!("Float pi = {}, small = {}", pi, small);
    println!("Boolean is_rust_fun = {}", is_rust_fun);
    println!("Char letter = {}, emoji = {}", letter, emoji);
}
