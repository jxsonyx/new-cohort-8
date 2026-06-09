// ── 2. STRUCT ────────────────────────────────────────────────
// A struct groups related pieces of data under one name.
// Think of it as a custom data type you design yourself.
pub struct Student {
    id: u32,      // u32  = unsigned 32-bit integer (no negatives)
    name: String, // String = heap-allocated, growable text
    age: u8,      // u8   = 0–255, plenty for an age
    // // grade: Grade, // our own enum type from above
    // score: f32,   // f32  = 32-bit floating point number
}


// This is the implementation of the student struct with its corresponding methods
impl Student {
    fn new(_id: u32, _name: String, _age: u8) -> Student {
        Student {
            id: _id, 
            name: _name,
            age: _age
        }
    }
}
