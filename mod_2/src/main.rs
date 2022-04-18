use rand::Rng;

fn main() {
    let  random_value = rand::thread_rng();
    println!("Random Value-- {}", random_value.clone().gen::<u8>());
}


