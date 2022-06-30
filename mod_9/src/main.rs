// Normal struct
#[derive(Debug)]
struct Human {
    name: String,
    age: u128,
    address: String,
}
// Tuple struct.
struct BlockParameter(u16, u16, u16);

// Unit struct.
struct AccountId;
fn main() {
    let mut data1 = String::new();
    // Init tuple struct.
    let block = BlockParameter(12, 10, 160);
    // Init normal struct.
    let human = Human {
        name: String::from("Siddharth"),
        age: 22,
        address: String::from("Flat"),
    };
    println!("Value of struct {:#?}", human);
    dbg!(human);
    println!("Value of student: {}", block.0);
}

fn calculate_rectrangle_area(height: u128, breath: u128) -> u128 {
    height* breath

}
