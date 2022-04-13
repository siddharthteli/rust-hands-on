use std::{env, fmt};

// Advance enum type with nested struct.
enum Person {
    Location {
        country: String,
        state: String,
        pin_code: u128,
    },
    Attribute {
        age: u8,
        gf_count: u8,
        height: u128,
        contact: String,
    },
}

// Had to impl display triat so println knowns how to display this enum values.
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Person::Location {
                country,
                state,
                pin_code,
            } => write!(
                f,
                "Country: {}, State: {}, Pin Code {}",
                country, state, pin_code
            ),
            Person::Attribute {
                age,
                gf_count,
                height,
                contact,
            } => write!(f, "{}, {}, {}, {}", age, gf_count, height, contact),
        }
    }
}

fn main() {
    // Reading all cli arguments
    let args: Vec<String> = env::args().collect();
    // Converting cli argument from string to u128
    let data: u128 = args[3]
        .clone()
        .parse::<u128>()
        .expect("Phat gaya, Which means cli argument passed can't be converted to u128");
    // Location  type of person.
    println!(
        "Full Location struct here---- {}",
        return_location(args[1].clone(), args[2].clone(), data)
    );
    // Attribute  type of person.
    println!("Full Attribute struct here--- {}", return_attribute());
}

// Returns Location which is off Person type.
fn return_location(country: String, state: String, pin_code: u128) -> Person {
    Person::Location {
        country: country,
        state: state,
        pin_code: pin_code,
    }
}

// Hard coded returns Attribute off Person type.
fn return_attribute() -> Person {
    Person::Attribute {
        age: 18,
        gf_count: 123,
        height: 170,
        contact: "+91238984398".to_string(),
    }
}
