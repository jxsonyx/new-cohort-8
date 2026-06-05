pub fn test_ownership() {
    let name = String::from("Alice"); // `name` is the owner of this String
    // name.push('r');
    println!("name is: {}", name);
}

pub fn call_name() {
    let name: &str = "Yusrah";
    // name.
    
    println!("name is: {}", name);
}
