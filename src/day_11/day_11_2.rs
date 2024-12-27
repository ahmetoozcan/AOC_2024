use count_digits::CountDigits;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn split_number(num: i64, cache: &mut HashMap<i64, (i64, i64)>) -> (i64, i64) {
    if let Some(&result) = cache.get(&num) {
        return result;
    }
    let num_str = num.to_string();
    let split_pos = num_str.len() / 2;
    let (left, right) = num_str.split_at(split_pos);
    let left_num = left.parse::<i64>().unwrap();
    let right_num = right.parse::<i64>().unwrap();
    let result = (left_num, right_num);
    cache.insert(num, result);
    result
}

fn stones(
    stone_counter: &mut i64,
    steps: i64,
    stone: i64,
    split_cache: &mut HashMap<i64, (i64, i64)>,
    counter_cache: &mut HashMap<(i64, i64), i64>,
) {
    if steps == 0 {
        return;
    }

    if let Some(&cached_result) = counter_cache.get(&(steps, stone)) {
        *stone_counter += cached_result;
        return;
    }

    let initial_counter = *stone_counter;

    if stone == 0 {
        stones(stone_counter, steps - 1, 1, split_cache, counter_cache);
    } else if stone.count_digits() % 2 == 0 {
        let (stone1, stone2) = split_number(stone, split_cache);
        *stone_counter += 1;
        stones(stone_counter, steps - 1, stone1, split_cache, counter_cache);
        stones(stone_counter, steps - 1, stone2, split_cache, counter_cache);
    } else {
        stones(
            stone_counter,
            steps - 1,
            stone * 2024,
            split_cache,
            counter_cache,
        );
    }

    let result = *stone_counter - initial_counter;
    counter_cache.insert((steps, stone), result);
}

fn calculate(filename: &str) -> i64 {
    let vec: Vec<i64> = read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let mut split_cache: HashMap<i64, (i64, i64)> = HashMap::new();
    let mut counter_cache: HashMap<(i64, i64), i64> = HashMap::new();
    let mut stone_counter: i64 = vec.len() as i64;
    for x in 0..vec.len() {
        stones(
            &mut stone_counter,
            75,
            vec[x],
            &mut split_cache,
            &mut counter_cache,
        );
    }

    stone_counter
}

fn main() {
    let start = Instant::now();
    println!("{}", calculate("src/input.txt"));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
