// ===== Mini Project: Text Editor (Ownership + Borrowing Demo) =====

// Function jo ownership leta hai (string ko kha jata hai 😅)
fn take_ownership(text: String) {
    println!("🔑 Ownership moved here: {}", text);
    // text yahan drop ho jata hai end of scope pe
}

// Function jo sirf borrow karta hai (sirf read)
fn read_text(text: &String) {
    println!("📖 Reading borrowed text: {}", text);
}

// Function jo mutable borrow karta hai (edit karne ke liye)
fn edit_text(text: &mut String) {
    text.push_str(" (edited)");
    println!("✏️ Edited text inside function");
}

fn main() {
    // Step 1: Ownership
    let s1 = String::from("Hello Rust");
    println!("Original text: {}", s1);

    // Ownership transfer
    take_ownership(s1);
    // println!("{}", s1); // ❌ ERROR: s1 ab valid nahi hai

    // Step 2: Borrowing
    let s2 = String::from("Borrow me");
    read_text(&s2); // borrow kiya
    println!("Still accessible after borrow: {}", s2);

    // Step 3: Mutable Borrowing
    let mut s3 = String::from("Change me");
    edit_text(&mut s3); // mutable borrow
    println!("After edit: {}", s3);

    // Step 4: Multiple borrows
    let s4 = String::from("Multiple borrows example");
    let r1 = &s4; // immutable borrow
    let r2 = &s4; // another immutable borrow
    println!("Two borrows at same time: {}, {}", r1, r2);

    // Step 5: Mutable borrow ke rules
    let mut s5 = String::from("Rust rules");
    {
        let r3 = &mut s5; // ek mutable borrow allowed
        println!("Mutable borrow inside scope: {}", r3);
    } // scope khatam → ab wapas borrow kar sakte ho

    let r4 = &mut s5;
    println!("Mutable borrow again: {}", r4);
}
