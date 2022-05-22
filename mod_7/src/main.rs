fn main() {
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

}
