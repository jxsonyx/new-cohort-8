mod student_struct;
mod registry_struct;
mod registry;


fn main() {

    // println!("")

}



// // ── 1. ENUM ─────────────────────────────────────────────────
// // An enum lists every possible variant of a type.
// // Here a student can only be in ONE of these three years.
// #[derive(Debug)] // lets us print it with {:?}
// #[derive(PartialEq)] // lets us compare two Grade values with ==
// enum Grade {
//     First,
//     Second,
//     Third,
// }

// // Add a helper method to Grade so we can turn it into a readable string.
// // `impl` means "implement behaviour for this type".
// impl Grade {
//     fn as_str(&self) -> &str {
//         match self {
//             // match checks EVERY variant — compiler
//             Grade::First => "1st Year", // forces you to handle them all
//             Grade::Second => "2nd Year",
//             Grade::Third => "3rd Year",
//         }
//     }
// }

// // ── 2. STRUCT ────────────────────────────────────────────────
// // A struct groups related pieces of data under one name.
// // Think of it as a custom data type you design yourself.
// struct Student {
//     id: u32,      // u32  = unsigned 32-bit integer (no negatives)
//     name: String, // String = heap-allocated, growable text
//     age: u8,      // u8   = 0–255, plenty for an age
//     grade: Grade, // our own enum type from above
//     score: f32,   // f32  = 32-bit floating point number
// }

// // ── 3. IMPL BLOCK FOR STUDENT ────────────────────────────────
// // We add methods to Student here. Methods always take `&self`
// // (borrow the student) or `&mut self` (borrow mutably to change it).
// impl Student {
//     // `new` is the conventional constructor name in Rust.
//     // It takes plain values and returns a fully built Student.
//     fn new(id: u32, name: &str, age: u8, grade: Grade, score: f32) -> Student {
//         Student {
//             id,                     // shorthand: field `id` = variable `id`
//             name: name.to_string(), // &str → owned String
//             age,
//             grade,
//             score,
//         }
//     }

//     // Immutable borrow: we only read the student, never change it.
//     fn display(&self) {
//         println!(
//             "  [{:>3}]  {:<20}  age: {:>2}  grade: {:<9}  score: {:.1}",
//             self.id,
//             self.name,
//             self.age,
//             self.grade.as_str(),
//             self.score,
//         );
//     }

//     // Mutable borrow: we need to change score, so we take &mut self.
//     fn update_score(&mut self, new_score: f32) {
//         println!("  ✏  {} | score {} → {}", self.name, self.score, new_score);
//         self.score = new_score; // `self.score` is directly mutated
//     }

//     // Returns a letter grade — a &str with 'static lifetime (lives forever).
//     fn letter_grade(&self) -> &str {
//         // match on a range — the `..=` means "inclusive range"
//         match self.score as u32 {
//             70..=100 => "A",
//             60..=69 => "B",
//             50..=59 => "C",
//             40..=49 => "D",
//             _ => "F", // `_` is the catch-all arm
//         }
//     }
// }

// // ── 4. REGISTRY STRUCT ───────────────────────────────────────
// // The registry owns a Vec<Student> — a growable list of students.
// // Vec<T> lives on the heap; its length can change at runtime.
// struct Registry {
//     students: Vec<Student>, // Vec<Student> = "a list of Student values"
//     next_id: u32,           // auto-increment counter for IDs
// }

// impl Registry {
//     // Create an empty registry.
//     fn new() -> Registry {
//         Registry {
//             students: Vec::new(), // empty Vec — no heap allocation yet
//             next_id: 1,
//         }
//     }

//     // Add a student. We take ownership of `student` and push it into the Vec.
//     // The Vec now owns the Student — that's why we don't need &student here.
//     fn add(&mut self, name: &str, age: u8, grade: Grade, score: f32) {
//         let id = self.next_id;
//         let student = Student::new(id, name, age, grade, score);
//         println!("  ✅  Added: {} (ID {})", student.name, student.id);
//         self.students.push(student); // Vec takes ownership of `student`
//         self.next_id += 1;
//     }

//     // Display every student. We borrow each element with `&` — no ownership moves.
//     fn list_all(&self) {
//         if self.students.is_empty() {
//             println!("  (no students enrolled yet)");
//             return;
//         }
//         println!(
//             "  {:>5}  {:<20}  {:<6}  {:<9}  {:<6}  {}",
//             "ID", "Name", "Age", "Grade", "Score", "Letter"
//         );
//         println!("  {}", "-".repeat(62));
//         for student in &self.students {
//             // `&` = borrow, not move
//             student.display();
//             println!(
//                 "  {}{:<58}{}",
//                 " ".repeat(49),
//                 format!("{}", student.letter_grade()),
//                 ""
//             );
//         }
//     }

//     // Search by name — returns Option<&Student>.
//     // Option means: either Some(value) or None. No null pointers!
//     fn find_by_name(&self, name: &str) -> Option<&Student> {
//         for student in &self.students {
//             // to_lowercase() so the search is case-insensitive
//             if student.name.to_lowercase() == name.to_lowercase() {
//                 return Some(student); // found — wrap in Some
//             }
//         }
//         None // not found — return None (no crash, no null)
//     }

//     // Search by ID — same Option pattern.
//     fn find_by_id(&self, id: u32) -> Option<&Student> {
//         // iter().find() walks the Vec and returns the first match wrapped in Option
//         self.students.iter().find(|s| s.id == id)
//     }

//     // Update score — needs &mut because we change data inside the Vec.
//     fn update_score(&mut self, id: u32, new_score: f32) {
//         // iter_mut() gives mutable references to each element
//         if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
//             student.update_score(new_score);
//         } else {
//             println!("  ❌  Student with ID {} not found.", id);
//         }
//     }

//     // Remove a student by ID.
//     // retain() keeps only the elements where the closure returns true.
//     fn remove(&mut self, id: u32) {
//         let before = self.students.len();
//         self.students.retain(|s| s.id != id); // keep everyone EXCEPT id
//         if self.students.len() < before {
//             println!("  🗑  Student ID {} removed.", id);
//         } else {
//             println!("  ❌  Student with ID {} not found.", id);
//         }
//     }

//     // Show a simple summary using iterator methods.
//     fn summary(&self) {
//         let count = self.students.len(); // usize = index/count type
//         if count == 0 {
//             println!("  No students to summarise.");
//             return;
//         }

//         // .iter().map().sum() — functional style: transform then accumulate
//         let total: f32 = self.students.iter().map(|s| s.score).sum();
//         let average = total / count as f32; // cast usize → f32 for division

//         // .iter().max_by() — find student with highest score
//         let top = self
//             .students
//             .iter()
//             .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

//         println!("  Total students : {}", count);
//         println!("  Average score  : {:.1}", average);
//         if let Some(t) = top {
//             // if let unwraps Some(t), ignores None
//             println!("  Top student    : {} ({:.1})", t.name, t.score);
//         }
//     }
// }

// // ── 5. MAIN ──────────────────────────────────────────────────
// fn main() {
//     print_banner();

//     // Create the registry — it owns the Vec<Student> on the heap.
//     let mut registry = Registry::new();

//     // ── Demo: Adding students ──────────────────────────────
//     section("1. Adding students (struct + Vec::push)");
//     registry.add("Amara Osei", 20, Grade::First, 78.5);
//     registry.add("Kofi Mensah", 22, Grade::Second, 64.0);
//     registry.add("Esi Boateng", 21, Grade::First, 91.0);
//     registry.add("Kwame Asante", 23, Grade::Third, 55.5);
//     registry.add("Abena Darko", 20, Grade::Second, 43.0);

//     // ── Demo: Listing all ─────────────────────────────────
//     section("2. Listing all students (Vec iteration + borrowing)");
//     registry.list_all();

//     // ── Demo: Searching — Option<T> ───────────────────────
//     section("3. Search by name (Option<&Student>)");

//     // `if let` is the idiomatic way to unwrap an Option when you
//     // only care about the Some case.
//     if let Some(s) = registry.find_by_name("Kofi Mensah") {
//         println!(
//             "  Found → ID: {}  Grade: {}  Score: {}",
//             s.id,
//             s.grade.as_str(),
//             s.score
//         );
//     }

//     // match handles BOTH Some and None explicitly — good for teaching.
//     match registry.find_by_name("Yaw Owusu") {
//         Some(s) => println!("  Found: {}", s.name),
//         None => println!("  'Yaw Owusu' not found in registry."),
//     }

//     // ── Demo: Search by ID ────────────────────────────────
//     section("4. Search by ID");
//     match registry.find_by_id(3) {
//         Some(s) => println!("  ID 3 → {} | letter grade: {}", s.name, s.letter_grade()),
//         None => println!("  ID 3 not found."),
//     }

//     // ── Demo: Update score — mutable borrow ───────────────
//     section("5. Updating a score (&mut self)");
//     registry.update_score(2, 72.0); // Kofi improves!
//     registry.update_score(99, 80.0); // ID that doesn't exist

//     // ── Demo: Remove ──────────────────────────────────────
//     section("6. Removing a student (Vec::retain)");
//     registry.remove(5); // remove Abena
//     registry.remove(99); // ID that doesn't exist

//     // ── Demo: Final list ──────────────────────────────────
//     section("7. Registry after update + removal");
//     registry.list_all();

//     // ── Demo: Summary — iterators ─────────────────────────
//     section("8. Summary (iterators: map, sum, max_by)");
//     registry.summary();

//     // ── Demo: Option<T> deep dive ─────────────────────────
//     section("9. Option<T> — Some vs None");
//     demo_option();

//     println!("\n  Done! See src/main.rs for full annotations.\n");
// }

// // ── EXTRA: Option demo isolated for clarity ──────────────────
// fn demo_option() {
//     // Option<T> has two variants: Some(value)  or  None
//     // It replaces null — you cannot forget to check it.
//     let found: Option<&str> = Some("Esi Boateng");
//     let not_found: Option<&str> = None;

//     // unwrap_or gives you the value, or a default if None
//     println!("  found     → {}", found.unwrap_or("nobody"));
//     println!("  not_found → {}", not_found.unwrap_or("nobody"));

//     // if let — only runs the block if it's Some
//     if let Some(name) = found {
//         println!("  if let extracted: {}", name);
//     }

//     // match — forces you to handle both arms
//     match not_found {
//         Some(n) => println!("  Got: {}", n),
//         None => println!("  match: correctly handled the None case"),
//     }
// }

// // ── HELPERS ──────────────────────────────────────────────────
// fn print_banner() {
//     println!();
//     println!("  ╔══════════════════════════════════════╗");
//     println!("  ║      RUST STUDENT REGISTRY v1.0      ║");
//     println!("  ║  struct · Vec · enum · Option · impl ║");
//     println!("  ╚══════════════════════════════════════╝");
//     println!();
// }

// fn section(title: &str) {
//     println!();
//     println!("  ┌─ {} ", title);
// }
