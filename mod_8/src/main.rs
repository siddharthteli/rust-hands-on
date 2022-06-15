fn main() {
    // println!("Value : {}", return_string("sidD".to_string()));
    let data: String = "Siddharth".to_string();
    let mut data1: String = "Siddharth".to_string();

    return_string_reference(&data);
    println!("{}", data);
}

fn return_string(data: String) -> String {
    let mut result = data.split_ascii_whitespace();
    result.next().unwrap().to_string()
}

fn return_string_reference(data: &String) -> String {
    let mut result = data.split_ascii_whitespace();
    result.next().unwrap().to_string()
}

fn takes_mut_reference(data: &mut String) {}

fn test() {
    let test = String::from("Siddharth");
    let test1 = test.clone();
    println!("Value of string {}", test);
}

fn return_first_word(data: &String) -> &str {
    let bytes_value = data.bytes();

    for (i, new1) in bytes_value.enumerate() {
        if new1 == b' ' {
            return &data[0..i];
        }
    }

    data
}
