use add_one::add_one;
use rand::Rng;
fn main() {
    let mut value = rand::thread_rng();
   
    println!("Value -- {}", add_one(value.gen_range(1..100)));
}
