use std::io;
use std::io::Read;
fn main() {
    let mut var: Vec<i64> = Vec::new();
    let mut data = String::new();
    let mut test = String::new();
    let count: i64;
    io::stdin().read_line(&mut test).expect("Invalid, Not num");
    let mut value: i64 = 0;
    count = test.trim().parse::<i64>().expect("Can't Convert");

    for i in 0..count {
        data.clear();
        var.clear();
        io::stdin().read_line(&mut data).expect("Invalid, Not num");
        for i in data.split_whitespace() {
            value = i.parse::<i64>().expect("Can't convert");
            var.push(value);
        }

        println!("{}", win_or_lose(var[0], var[1], var[2], var[3]));
    }
}

fn win_or_lose(n: i64, a: i64, b: i64, k: i64) -> &'static str {
    let mut count: i64 = 0;

    for i in 1..n {
        if (i % a  == 0 && i % b == 0) {
            continue;
        }
        if (i % a == 0) {
            count += 1;
        } else if (i % b == 0) {
            count += 1;
        }
    }
    if (count >= k) {
        "Win"
    } else {
        "Lose"
    }
}
