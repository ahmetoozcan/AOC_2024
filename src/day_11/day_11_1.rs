use count_digits::CountDigits;
use std::fs::read_to_string;
use std::time::Instant;

fn split_number(num: i64) -> (i64, i64) {
    let num_str = num.to_string();
    let split_pos = num_str.len() / 2;
    let (left, right) = num_str.split_at(split_pos);
    let left_num = left.parse::<i64>().unwrap();
    let right_num = right.parse::<i64>().unwrap();
    (left_num, right_num)
}

fn stones(stone_counter: &mut i64, steps: i64, stone: i64) {
    if steps == 0 {
        return;
    }

    if stone == 0 {
        stones(stone_counter, steps - 1, 1);
    } else if stone.count_digits() % 2 == 0 {
        let (stone1, stone2) = split_number(stone);
        *stone_counter += 1;
        stones(stone_counter, steps - 1, stone1);
        stones(stone_counter, steps - 1, stone2);
    } else {
        stones(stone_counter, steps - 1, stone * 2024);
    }
}

fn calculate(filename: &str) -> i64 {
    let mut sum: i64 = 0;

    let vec: Vec<i64> = read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let mut stone_counter: i64 = vec.len() as i64;
    for x in 0..vec.len() {
        stones(&mut stone_counter, 25, vec[x]);
    }

    stone_counter
}

fn main() {
    let start = Instant::now();
    println!("{}", calculate("src/input.txt"));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
