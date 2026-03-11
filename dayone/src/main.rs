use std::collections::HashMap;
use std::io::{self, Write};

// Student struct - demonstrates heap allocation for String
#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,      // Heap-allocated
    age: u8,           // Stack-allocated
    gpa: f32,          // Stack-allocated
    courses: Vec<String>, // Heap-allocated vector
}

impl Student {
    // Constructor that takes ownership of String
    fn new(id: u32, name: String, age: u8, gpa: f32) -> Self {
        Student {
            id,
            name,
            age,
            gpa,
            courses: Vec::new(),
        }
    }

    // Borrowing &self immutably - no ownership transfer
    fn display(&self) {
        println!("\n--- Student Record ---");
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("GPA: {:.2}", self.gpa);
        println!("Courses: {}", self.courses.join(", "));
        println!("---------------------");
    }

    // Mutable borrow &mut self - can modify without ownership
    fn add_course(&mut self, course: String) {
        self.courses.push(course);
    }

    // Mutable borrow to update GPA
    fn update_gpa(&mut self, new_gpa: f32) {
        self.gpa = new_gpa;
    }

    // Returns reference with explicit lifetime
    fn get_name(&self) -> &str {
        &self.name
    }

    // Returns reference to courses with lifetime tied to self
    fn get_courses(&self) -> &[String] {
        &self.courses
    }
}

// StudentRegistry demonstrates ownership management
struct StudentRegistry {
    students: HashMap<u32, Student>, // Owns all Student instances
    next_id: u32,
}

impl StudentRegistry {
    fn new() -> Self {
        StudentRegistry {
            students: HashMap::new(),
            next_id: 1,
        }
    }

    // Takes ownership of Student and moves it into HashMap
    fn add_student(&mut self, mut student: Student) -> u32 {
        let id = self.next_id;
        student.id = id;
        self.students.insert(id, student);
        self.next_id += 1;
        id
    }

    // Returns immutable reference - demonstrates borrowing
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    // Returns mutable reference - exclusive access
    fn get_student_mut(&mut self, id: u32) -> Option<&mut Student> {
        self.students.get_mut(&id)
    }

    // Removes and returns Student - transfers ownership out
    fn remove_student(&mut self, id: u32) -> Option<Student> {
        self.students.remove(&id)
    }

    // Demonstrates lifetime annotation: returned references are tied to self
    fn find_by_name<'a>(&'a self, name: &str) -> Vec<&'a Student> {
        self.students
            .values()
            .filter(|student| student.name.contains(name))
            .collect()
    }

    // Returns reference to all students - lifetime tied to registry
    fn list_all(&self) -> impl Iterator<Item = &Student> {
        self.students.values()
    }

    // Demonstrates proper scoping with mutable borrows
    fn batch_update_gpa(&mut self, updates: Vec<(u32, f32)>) {
        for (id, new_gpa) in updates {
            // Mutable borrow scope limited to this block
            if let Some(student) = self.students.get_mut(&id) {
                student.update_gpa(new_gpa);
            }
            // Borrow ends here, allowing next iteration
        }
    }
}

// Demonstrates function that borrows immutably
fn calculate_average_gpa(students: &[&Student]) -> f32 {
    if students.is_empty() {
        return 0.0;
    }
    let sum: f32 = students.iter().map(|s| s.gpa).sum();
    sum / students.len() as f32
}

// Demonstrates explicit lifetime parameters
fn find_top_student<'a>(students: &'a [&'a Student]) -> Option<&'a Student> {
    students.iter().max_by(|a, b| a.gpa.partial_cmp(&b.gpa).unwrap()).copied()
}

// Utility function demonstrating String ownership
fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string() // Returns owned String
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_string(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}

fn read_f32(prompt: &str) -> f32 {
    loop {
        let input = read_string(prompt);
        match input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}

fn main() {
    let mut registry = StudentRegistry::new();
    
    println!("=== Rust Student Record System ===");
    println!("Demonstrating Ownership & Borrowing\n");

    loop {
        println!("\n--- Menu ---");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Update GPA");
        println!("4. Add Course");
        println!("5. List All Students");
        println!("6. Search by Name");
        println!("7. Calculate Average GPA");
        println!("8. Find Top Student");
        println!("9. Remove Student");
        println!("0. Exit");
        
        let choice = read_string("Enter choice: ");

        match choice.as_str() {
            "1" => {
                // Demonstrates ownership transfer
                let name = read_string("Enter name: ");
                let age = read_string("Enter age: ").parse().unwrap_or(18);
                let gpa = read_f32("Enter GPA: ");
                
                // Student is created and ownership transferred to add_student
                let student = Student::new(0, name, age, gpa);
                let id = registry.add_student(student);
                // student is moved and no longer accessible here
                println!("✓ Student added with ID: {}", id);
            }
            "2" => {
                let id = read_u32("Enter student ID: ");
                // Immutable borrow - multiple concurrent reads allowed
                match registry.get_student(id) {
                    Some(student) => student.display(),
                    None => println!("✗ Student not found"),
                }
                // Borrow ends here
            }
            "3" => {
                let id = read_u32("Enter student ID: ");
                let new_gpa = read_f32("Enter new GPA: ");
                
                // Mutable borrow - exclusive access
                match registry.get_student_mut(id) {
                    Some(student) => {
                        student.update_gpa(new_gpa);
                        println!("✓ GPA updated");
                    }
                    None => println!("✗ Student not found"),
                }
                // Mutable borrow ends here
            }
            "4" => {
                let id = read_u32("Enter student ID: ");
                let course = read_string("Enter course name: ");
                
                // Demonstrates scoped mutable borrow
                {
                    if let Some(student) = registry.get_student_mut(id) {
                        student.add_course(course); // course ownership moved
                        println!("✓ Course added");
                    } else {
                        println!("✗ Student not found");
                    }
                } // Mutable borrow scope ends
            }
            "5" => {
                println!("\n=== All Students ===");
                // Iterator borrows immutably
                for student in registry.list_all() {
                    println!("ID {}: {} (GPA: {:.2})", student.id, student.name, student.gpa);
                }
            }
            "6" => {
                let search = read_string("Enter name to search: ");
                // Demonstrates lifetime-annotated function
                let results = registry.find_by_name(&search);
                
                if results.is_empty() {
                    println!("No students found");
                } else {
                    println!("\n=== Search Results ===");
                    for student in results {
                        student.display();
                    }
                }
            }
            "7" => {
                // Collect references to calculate average
                let students: Vec<&Student> = registry.list_all().collect();
                let avg = calculate_average_gpa(&students);
                println!("Average GPA: {:.2}", avg);
            }
            "8" => {
                let students: Vec<&Student> = registry.list_all().collect();
                match find_top_student(&students) {
                    Some(top) => {
                        println!("\n=== Top Student ===");
                        top.display();
                    }
                    None => println!("No students in registry"),
                }
            }
            "9" => {
                let id = read_u32("Enter student ID to remove: ");
                // Ownership transferred out of registry
                match registry.remove_student(id) {
                    Some(student) => {
                        println!("✓ Removed student: {}", student.name);
                        // student is dropped here when it goes out of scope
                    }
                    None => println!("✗ Student not found"),
                }
            }
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
    
    // registry is dropped here, which drops all Student instances it owns
}

// Additional demonstrations of ownership concepts

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_transfer() {
        let mut registry = StudentRegistry::new();
        let student = Student::new(0, "Alice".to_string(), 20, 3.8);
        
        let id = registry.add_student(student);
        // student is moved, cannot use here
        
        assert!(registry.get_student(id).is_some());
    }

    #[test]
    fn test_borrowing() {
        let student = Student::new(1, "Bob".to_string(), 21, 3.5);
        
        // Multiple immutable borrows are OK
        let name1 = student.get_name();
        let name2 = student.get_name();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_mutable_borrowing() {
        let mut student = Student::new(1, "Charlie".to_string(), 22, 3.2);
        
        student.update_gpa(3.7);
        assert_eq!(student.gpa, 3.7);
        
        // Can borrow immutably after mutable borrow ends
        let gpa = student.gpa;
        assert_eq!(gpa, 3.7);
    }
}
// new beginings