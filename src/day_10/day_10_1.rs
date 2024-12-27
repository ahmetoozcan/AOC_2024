use std::fs::read_to_string;
use std::time::Instant;

fn path_search(
    index: (usize, usize),
    vec: &Vec<Vec<i32>>,
    num: i32,
    score: &mut Vec<(usize, usize)>,
) {
    if index.0 >= vec.len() || index.1 >= vec.len() {
        return;
    }
    if vec[index.0][index.1] != num {
        return;
    }
    if vec[index.0][index.1] == 9 {
        if !score.contains(&(index.0, index.1)) {
            score.push((index.0, index.1));
        }
        return;
    }

    path_search((index.0 + 1, index.1), vec, num + 1, score);
    path_search((index.0, index.1 + 1), vec, num + 1, score);

    if index.0 > 0 {
        path_search((index.0 - 1, index.1), vec, num + 1, score);
    }
    if index.1 > 0 {
        path_search((index.0, index.1 - 1), vec, num + 1, score);
    }
}

fn calculate(filename: &str) -> i64 {
    let mut sum: i64 = 0;

    let vec: Vec<Vec<i32>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    for x in 0..vec.len() {
        let mut score: Vec<(usize, usize)> = Vec::new();
        for y in 0..vec[0].len() {
            if vec[x][y] == 0 {
                path_search((x, y), &vec, 0, &mut score);
                sum += score.len() as i64;
                score.clear();
            }
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
