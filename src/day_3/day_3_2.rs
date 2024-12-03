use regex::Regex;
use std::fs::read_to_string;

fn calculate(filename: &str) -> i64 {
    let re = Regex::new(r"(don't\(\)|do\(\))|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum: i64 = 0;
    let mut is_do: bool = true;
    for line in read_to_string(filename).unwrap().lines() {
        for cap in re.captures_iter(line) {
            if let Some(do_or_dont) = cap.get(1) {
                if do_or_dont.as_str() == "do()" {
                    is_do = true;
                } else if do_or_dont.as_str() == "don't()" {
                    is_do = false;
                }
            } else if is_do {
                continue;
            } else if let (Some(first_number), Some(second_number)) = (cap.get(2), cap.get(3)) {
                sum += first_number.as_str().parse::<i64>().unwrap()
                    * second_number.as_str().parse::<i64>().unwrap();
            }
        }
    }

    sum
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
