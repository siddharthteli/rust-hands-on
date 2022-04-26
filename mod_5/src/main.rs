fn main() {
    let var: (i32, String, u64) = (14, "Siddharth".to_string(), 454);
    let array = [10; 40];
    let derived_value = {
        let x = 100;
        x * 200
    };

    let conditional_value = if (derived_value >= array[0]) {
        100
    } else {
        500
    };

    loop {
        println!("Hello");
        break;
    }
    let mut count = 10;
    'outer_loop: loop {
        println!("Outer loop");

        loop {
            count -= 1;
            println!("Inner loop");
            if count == 5 {
                break;
            }
            if count == 2 {
                break 'outer_loop;
            }
            
        }
    }

    println!("Value of conditional var: {}", conditional_value);
    println!("Value of derived value: {}", derived_value);
    println!("Value of var {}", var.1);
    println!("Length of array {}", array.len());
}
