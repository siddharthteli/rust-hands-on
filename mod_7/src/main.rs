fn main() {
    let string_data = String::from("Siddharth");
    let int_data: i32 = 12;

    makes_copy(int_data);
    takes_ownership(string_data);

    // Error below
    // println!("Value {}", string_data);
    print!("Value {}", int_data);
}

fn makes_copy(value: i32) {
    println!("Value {}", value);
}

fn takes_ownership(value: String) {
    print!("Value {}", value);
}

fn copy_and_clone() {
    let mut x = 5;
    let y = x;
    // Even if we change value of x y is not affect.
    // Coz y has copy of x.
    x = 100;
    let mut name1 = String::from("Siddharth");
    let name2 = name1;
    // This line will throw error
    // Coz name1 was moved into name2 & compiler
    // invalidates name1.
    //  println!("Value of strings name1 {}", name1);
    println!("Value of y {}", y);
    // This deeply copies name2 to name3 not just stack value.
    let name3 = name2.clone();
    println!("Value of name3 {}", name3);
}
