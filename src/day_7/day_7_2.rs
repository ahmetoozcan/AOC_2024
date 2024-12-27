use std::fs::read_to_string;
use std::time::Instant;

fn check_pos(vec: &Vec<usize>, index: usize, current: usize, target: usize) -> bool {
    if current == target && index == vec.len() {
        return true;
    }
    if current > target || index >= vec.len() {
        return false;
    }

    // Slower
    /* let concatenated = format!("{}{}", current, vec[index])
    .parse::<usize>()
    .unwrap(); */

    let b = vec[index];
    let concatenated = current * 10_usize.pow(b.ilog10() + 1) + b;

    check_pos(vec, index + 1, current + vec[index], target)
        || check_pos(vec, index + 1, current * vec[index], target)
        || check_pos(vec, index + 1, concatenated, target)
}

fn calculate(filename: &str) -> usize {
    let mut sum: usize = 0;

    let mut vec: Vec<(usize, Vec<usize>)> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let key: usize = parts[0].trim().parse().unwrap();
        vec.push((
            key,
            parts[1]
                .split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect(),
        ));
    }

    for pair in vec {
        if check_pos(&pair.1, 1, pair.1[0], pair.0) {
            sum += pair.0;
        }
    }

    sum
}

fn main() {
    let start = Instant::now();
    println!("{}", calculate("src/input.txt"));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
