use regex::Regex;
use std::fs::read_to_string;

fn calculate(filename: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum: i64 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        for [first_number, second_number] in re.captures_iter(line).map(|cap| cap.extract().1) {
            sum += first_number.parse::<i64>().unwrap() * second_number.parse::<i64>().unwrap();
        }
    }

    sum
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
