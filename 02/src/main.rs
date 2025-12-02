use std::io;

fn is_repeating_pattern(num_str: &str) -> bool {
    if num_str.len() < 2 {
        return false;
    }

    for pattern_len in 1..=(num_str.len() / 2) {
        if num_str.len() % pattern_len == 0 {
            let pattern = &num_str[..pattern_len];
            if pattern.repeat(num_str.len() / pattern_len) == num_str {
                return true;
            }
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut count = 0u64;
    let mut sum = 0u64;

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read input");

    for range_string in line.trim().split(',') {
        let parts: Vec<&str> = range_string.trim().split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let lower: u64 = parts[0].parse().unwrap();
        let upper: u64 = parts[1].parse().unwrap();

        for id in lower..=upper {
            if is_repeating_pattern(&id.to_string()) {
                count += 1;
                sum += id;
            }
        }
    }

    println!("{} invalid IDs found, sum to: {}", count, sum);
}
