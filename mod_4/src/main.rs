use std::io;
use std::io::Read;
fn main() {
    let mut var: Vec<u64> = Vec::new();
    let mut data = String::new();
    let mut test = String::new();
    let count: u64;
    io::stdin().read_line(&mut test).expect("Invalid, Not num");
    let mut value: u64 = 0;
    count = test.trim().parse::<u64>().expect("Can't Convert");

    for i in 0..count {
        data.clear();
        var.clear();
        io::stdin().read_line(&mut data).expect("Invalid, Not num");
        for i in data.split_whitespace() {
            value = i.parse::<u64>().expect("Can't convert");
            var.push(value);
        }
        println!("{}", win_or_lose(var[0], var[1], var[2], var[3]));
    }
}

fn win_or_lose(n: u64, mut a: u64, mut b: u64, k: u64) -> &'static str {
    let mut count: u64 = 0;
    if (b > a) {
        count = b;
        b = a;
        a = count;
    }
    count = (n / a + n / b) - 2 * (n / lowest_common_factor(a, b));
    if (count >= k) {
        "Win"
    } else {
        "Lose"
    }
}

fn lowest_common_factor(mut a: u64, mut b: u64) -> u64 {
    (a / highest_common_factor(a, b)) * b
}

fn highest_common_factor(mut a: u64, mut b: u64) -> u64 {
    let mut temp: u64;
    while (a % b != 0) {
        temp = a % b;
        a = b;
        b = temp;
    }
    return b;
}
