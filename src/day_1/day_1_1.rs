use std::fs::read_to_string;

fn read_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut vec1, mut vec2) = (Vec::new(), Vec::new());

    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        vec1.push(numbers[0].parse().unwrap());
        vec2.push(numbers[1].parse().unwrap());
    }

    (vec1, vec2)
}

fn main() {
    let (mut vec1, mut vec2) = read_lines("input_day_1.txt");

    vec1.sort();
    vec2.sort();

    let mut sum: i32 = 0;

    for it in vec1.iter_mut().zip(vec2.iter_mut()) {
        let diff = *it.0 - *it.1;

        sum += diff.abs();
    }

    println!("{}", sum);
}
