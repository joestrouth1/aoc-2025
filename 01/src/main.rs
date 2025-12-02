use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut dial_value = 50;
    let mut zeroes_seen = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let sign = if line.starts_with('L') { -1 } else { 1 };
        let rotation_amount: i32 = line[1..].parse().unwrap();
        dial_value = dial_value + (sign * rotation_amount);
        if dial_value % 100 == 0 {
            zeroes_seen += 1;
        }
    }
    println!("Zeroes seen: {}", zeroes_seen)
}
