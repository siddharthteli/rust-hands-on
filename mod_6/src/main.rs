

fn main() {
    // This value is hardcoded in executable file.
    let mut string_slice = "Siddharth";
    // Stored on heap.
    // Mem location of this string is stored on stack
    // while this itself stored on heap.
    let mut string = String::from("Siddharth");
    // Its easy to allocate more memory on heap.
    string.push_str("test");
    // String value is moved Hence dropped.
    // Has no effect on types impl Copy trait.
    drop(string);
    // Below line throws error coz value is moved.
    //  println!("Value of string {}", string); // Throws error.
}

