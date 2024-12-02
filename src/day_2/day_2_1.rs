use std::fs::read_to_string;

fn is_safe(numbers: Vec<i32>, is_remove: Option<bool>) -> bool {
    let dec: bool = numbers[0] > numbers[1];
    for index in 0..numbers.len() - 1 {
        let num1: i32 = numbers[index];
        let num2: i32 = numbers[index + 1];
        if (num2 > num1 && dec)             // Is breaking dec order rule
                || (num2 < num1 && !dec)    // Is breaking inc order rule
                || (num1 - num2).abs() < 1  // Is breaking max level rule
                || (num1 - num2).abs() > 3
        {
            return false;
        }
    }

    return true;
}

fn calculate(filename: &str) -> i32 {
    let mut safe: i32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe(numbers, None) {
            safe += 1;
        }
    }

    safe
}

fn main() {
    println!("{}", calculate("input_day_2.txt"));
}
