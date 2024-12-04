use std::fs::read_to_string;

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let mut vec: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        vec.push(line.into());
    }

    for i in 1..vec.len() - 1 {
        for j in 1..vec[i].len() - 1 {
            if vec[i].chars().nth(j).unwrap() == 'A' {
                if vec[i].chars().filter(|c| *c == 'S').count() == 2
                    && vec[i].chars().filter(|c| *c == 'M').count() == 2
                    && vec[i - 1].chars().nth(j - 1).unwrap()
                        != vec[i + 1].chars().nth(j + 1).unwrap()
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
