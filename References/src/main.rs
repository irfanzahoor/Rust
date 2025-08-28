use std::io;

// Function jo sirf read karega (immutable reference)
fn show_books(books: &Vec<String>) {
    println!("\n📚 Library Books:");
    for (i, book) in books.iter().enumerate() {
        println!("{}: {}", i + 1, book);
    }
}

// Function jo ek new book add karega (mutable reference)
fn add_book(books: &mut Vec<String>, book_name: String) {
    books.push(book_name);
    println!("✅ Book added successfully!");
}

// Function jo ek book ko edit karega (mutable reference)
fn edit_book(books: &mut Vec<String>, index: usize, new_name: String) {
    if index < books.len() {
        books[index] = new_name;
        println!("✏️ Book updated successfully!");
    } else {
        println!("❌ Invalid book index!");
    }
}

fn main() {
    let mut books = vec![
        String::from("The Rust Book"),
        String::from("Programming in C"),
        String::from("Learn Python"),
    ];

    loop {
        println!("\n===== Library Menu =====");
        println!("1. Show Books");
        println!("2. Add Book");
        println!("3. Edit Book");
        println!("4. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice: i32 = choice.trim().parse().unwrap_or(0);

        if choice == 1 {
            show_books(&books); // immutable reference
        } else if choice == 2 {
            println!("Enter book name to add: ");
            let mut book_name = String::new();
            io::stdin()
                .read_line(&mut book_name)
                .expect("Failed to read");
            add_book(&mut books, book_name.trim().to_string()); // mutable reference
        } else if choice == 3 {
            println!("Enter book number to edit: ");
            let mut index = String::new();
            io::stdin().read_line(&mut index).expect("Failed to read");
            let index: usize = index.trim().parse().unwrap_or(0) - 1;

            println!("Enter new book name: ");
            let mut new_name = String::new();
            io::stdin()
                .read_line(&mut new_name)
                .expect("Failed to read");

            edit_book(&mut books, index, new_name.trim().to_string()); // mutable reference
        } else if choice == 4 {
            println!("👋 Exiting Library System!");
            break;
        } else {
            println!("❌ Invalid choice!");
        }
    }
}

// ✅ Is project me tumne dekha:

// &Vec<String> → immutable reference → sirf read karna

// &mut Vec<String> → mutable reference → add ya edit karna

// Real-world example: Library = books, aur tum references ka use karke borrow kar rahe ho