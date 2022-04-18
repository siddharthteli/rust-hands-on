use rand::Rng;
use std::io;

fn main() {
    let mut data = String::new();
    let mut number: i64;
    let  random_value: i64;

    random_value = return_random_number();
    println!("Enter Guess-");

    loop {
        io::stdin().read_line(&mut data).expect("Invalid input");
        number = data.trim().parse::<i64>().expect("Can't convert");
        if (number == random_value) {
            println!("Congratulation you guessed it right");
            break;
        }
        if (number > random_value) {
            println!("----Guessed value is big----");
        } else {
            println!("----Guessed value is small----");
        }
        data.clear();
    }
}

fn return_random_number() -> i64 {
    let mut value = rand::thread_rng();
    value.gen_range(1..100)
}
