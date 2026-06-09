use crate::student_struct;
// ── 4. REGISTRY STRUCT ───────────────────────────────────────
// The registry owns a Vec<Student> — a growable list of students.
// Vec<T> lives on the heap; its length can change at runtime.
pub struct Registry {
    students: Vec<Student>, // Vec<Student> = "a list of Student values"
    next_id: u32,           // auto-increment counter for IDs
}

impl Registry {
    // Create an empty registry.
    fn new() -> Registry {
        Registry {
            students: Vec::new(), // empty Vec — no heap allocation yet
            next_id: 1,
        }
    }

    // Add a student. We take ownership of `student` and push it into the Vec.
    // The Vec now owns the Student — that's why we don't need &student here.
    fn add(&mut self, name: &str, age: u8) {
        let id = self.next_id;
        let student = student_struct::Student::new(id, name, age);
        println!("  ✅  Added: {} (ID {})", student.name, student.id);
        self.students.push(student); // Vec takes ownership of `student`
        self.next_id += 1;
    }

    // Display every student. We borrow each element with `&` — no ownership moves.
    fn list_all(&self) {
        if self.students.is_empty() {
            println!("  (no students enrolled yet)");
            return;
        }
        println!(
            "  {:>5}  {:<20}  {:<6}    {}",
            "ID", "Name", "Age", "Letter"
        );
        println!("  {}", "-".repeat(62));
        for student in &self.students {
            // `&` = borrow, not move
            student.display();
            println!(
                "  {}{:<58}{}",
                " ".repeat(49),
                // format!("{}", student.letter_grade()),
                ""
            );
        }
    }
}
