
// # 📌 Mini Project: Student Management System (Structs in Action)

use std::io;

// Student struct
struct Student {
    id: u32,
    name: String,
    grade: String,
}

impl Student {
    // Display student details (method with &self = immutable borrow)
    fn display(&self) {
        println!("🆔 ID: {}, 👤 Name: {}, 🎓 Grade: {}", self.id, self.name, self.grade);
    }

    // Update grade (method with &mut self = mutable borrow)
    fn update_grade(&mut self, new_grade: String) {
        self.grade = new_grade;
        println!("✅ Grade updated successfully!");
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("\n===== Student Management System =====");
        println!("1. Add Student");
        println!("2. Show All Students");
        println!("3. Update Student Grade");
        println!("4. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice: i32 = choice.trim().parse().unwrap_or(0);

        if choice == 1 {
            // Add Student
            let mut id = String::new();
            println!("Enter ID:");
            io::stdin().read_line(&mut id).expect("Failed to read");
            let id: u32 = id.trim().parse().unwrap_or(0);

            let mut name = String::new();
            println!("Enter Name:");
            io::stdin().read_line(&mut name).expect("Failed to read");

            let mut grade = String::new();
            println!("Enter Grade:");
            io::stdin().read_line(&mut grade).expect("Failed to read");

            let student = Student {
                id,
                name: name.trim().to_string(),
                grade: grade.trim().to_string(),
            };
            students.push(student);

            println!("✅ Student added successfully!");
        } 
        else if choice == 2 {
            // Show Students
            if students.is_empty() {
                println!("📭 No students found!");
            } else {
                println!("\n📋 All Students:");
                for s in &students {
                    s.display();
                }
            }
        } 
        else if choice == 3 {
            // Update Grade
            let mut id = String::new();
            println!("Enter student ID to update grade:");
            io::stdin().read_line(&mut id).expect("Failed to read");
            let id: u32 = id.trim().parse().unwrap_or(0);

            let mut found = false;
            for s in &mut students {
                if s.id == id {
                    let mut new_grade = String::new();
                    println!("Enter new grade:");
                    io::stdin().read_line(&mut new_grade).expect("Failed to read");
                    s.update_grade(new_grade.trim().to_string());
                    found = true;
                    break;
                }
            }
            if !found {
                println!("❌ Student not found!");
            }
        } 
        else if choice == 4 {
            println!("👋 Exiting Student Management System!");
            break;
        } 
        else {
            println!("❌ Invalid choice!");
        }
    }
}

// s
// ## ✅ Is project me tumne seekha:

// * `struct Student` → apna custom datatype banaya
// * `impl` block → methods (class-like behavior) add kiye
// * `Vec<Student>` → multiple students store kiye
// * `&self` → immutable borrow for reading
// * `&mut self` → mutable borrow for updating


