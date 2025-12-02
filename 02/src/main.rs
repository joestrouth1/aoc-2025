use std::io;

struct Score {
    count: u64,
    sum: u64,
}

impl Score {
    fn add(&mut self, id: u64) {
        self.count += 1;
        self.sum += id;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut score = Score { count: 0, sum: 0 };

    let mut line = String::new();
    let _ = stdin.read_line(&mut line);
    for range_string in line.split(',') {
        let range_bounds: Vec<&str> = range_string.split('-').collect();
        let lower_bound: u64 = range_bounds[0].parse().unwrap();
        let upper_bound: u64 = range_bounds[1].parse().unwrap();

        let mut current_id = lower_bound;
        while current_id <= upper_bound {
            let current_id_string = &current_id.to_string();
            // pt 1
            // if current_id_string.len() % 2 == 0 {
            //     let midpoint = current_id_string.len() / 2;
            //     let first_half = &current_id_string[..midpoint];
            //     let second_half = &current_id_string[midpoint..];
            //     if first_half == second_half {
            //         score.add(current_id)
            //     }
            // }

            // pt2
            if current_id_string.len() >= 2 {
                let slice_lengths_range = 1..(current_id_string.len() / 2 + 1);
                let sliceable_lengths =
                    slice_lengths_range.filter(|len| current_id_string.len() % len == 0);
                for slice_length in sliceable_lengths.rev() {
                    let first_slice = &current_id_string[..slice_length];
                    let slice_matches: Vec<&str> = current_id_string.matches(first_slice).collect();
                    if slice_matches.len() == (current_id_string.len() / slice_length) {
                        score.add(current_id);
                        break;
                    }
                }
            }
            current_id += 1;
        }
    }
    println!("{} invalid IDs found, sum to: {}", score.count, score.sum)
}
