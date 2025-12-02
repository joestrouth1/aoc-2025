use std::io;

fn main() {
    let stdin = io::stdin();
    let mut invalid_id_count: u64 = 0;
    let mut invalid_id_sum: u64 = 0;

    let mut line = String::new();
    let _ = stdin.read_line(&mut line);
    for range_string in line.split(',') {
        let range_bounds: Vec<&str> = range_string.split('-').collect();
        let lower_bound: u64 = range_bounds[0].parse().unwrap();
        let upper_bound: u64 = range_bounds[1].parse().unwrap();

        let mut current_id = lower_bound;
        while current_id <= upper_bound {
            let current_id_string = current_id.to_string();
            if current_id_string.len() % 2 == 0 {
                let midpoint = current_id_string.len() / 2;
                let first_half = &current_id_string[..midpoint];
                let second_half = &current_id_string[midpoint..];
                if first_half == second_half {
                    invalid_id_sum += current_id;
                    invalid_id_count += 1;
                }
            }

            current_id += 1;
        }
    }
    println!(
        "{} invalid IDs found, sum to: {}",
        invalid_id_count, invalid_id_sum
    )
}
