use std::fs::read_to_string;
use std::time::Instant;

fn compact_n_checksum(vec: &mut Vec<i64>) -> i64 {
    let mut max_id: i64 = (vec.len() / 2) as i64;
    let mut index: i64 = 0;
    let mut id: i64 = 0;
    let mut checksum: i64 = 0;
    for x in 0..vec.len() {
        if id > max_id {
            break;
        }
        if x % 2 == 0 {
            for _ in 0..vec[x] {
                checksum += id * index;
                index += 1;
            }
            id += 1;
        } else {
            for _ in 0..vec[x] {
                if vec[(max_id * 2) as usize] == 0 {
                    max_id -= 1;
                }
                checksum += (max_id as i64) * index;
                index += 1;
                vec[(max_id * 2) as usize] -= 1;
            }
        }
    }

    checksum
}

fn calculate(filename: &str) -> i64 {
    let mut vec: Vec<i64> = read_to_string(filename)
        .unwrap()
        .chars()
        .map(|c| (c as i64) - 48)
        .collect();

    compact_n_checksum(&mut vec)
}

fn main() {
    let start = Instant::now();
    println!("{}", calculate("src/input.txt"));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
