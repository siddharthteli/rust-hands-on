use rand::Rng;
use std::io;

fn main() {
    let mut data = String::new();
    let mut number: i64;
    let  random_value: i64;

    random_value = return_random_number();
    println!("Enter Guess-");

    loop {
        // Read line appendeds to data var. 
        io::stdin().read_line(&mut data).expect("Invalid input");
        // trim to remove white & new line.
        number = data.trim().parse::<i64>().expect("Can't convert");
        // If num is equal loop will end.
        if (number == random_value) {
            println!("Congratulation you guessed it right");
            break;
        }
        // If num is greater user will get promt to enter smller num.
        if (number > random_value) {
            println!("----Guessed value is big----");
        } else {
            println!("----Guessed value is small----");
        }
        // Clear variable data
        data.clear();
    }
}
// Returns random number b/w 1..100
fn return_random_number() -> i64 {
    let mut value = rand::thread_rng();
    value.gen_range(1..100)
}
