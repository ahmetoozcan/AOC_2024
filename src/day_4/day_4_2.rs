use std::fs::read_to_string;

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let vec: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|c| c.trim().chars().collect())
        .collect();

    for i in 1..vec.len() - 1 {
        for j in 1..vec[i].len() - 1 {
            if vec[i][j] == 'A' {
                let ch = [
                    vec[i + 1][j + 1],
                    vec[i - 1][j - 1],
                    vec[i + 1][j - 1],
                    vec[i - 1][j + 1],
                ];

                if ch.iter().filter(|c| **c == 'S').count() == 2
                    && ch.iter().filter(|c| **c == 'M').count() == 2
                    && ch[0] != ch[1]
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
