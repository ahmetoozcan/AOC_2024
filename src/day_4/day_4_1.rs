use std::fs::read_to_string;

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let mut vec: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        vec.push(line.into());
    }

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i].chars().nth(j).unwrap() == 'X' {
                // Could've done better
                // down vertical
                if i + 3 < vec.len()
                    && vec[i + 1].chars().nth(j).unwrap() == 'M'
                    && vec[i + 2].chars().nth(j).unwrap() == 'A'
                    && vec[i + 3].chars().nth(j).unwrap() == 'S'
                {
                    count += 1;
                }

                // up vertical
                if i >= 3
                    && vec[i - 1].chars().nth(j).unwrap() == 'M'
                    && vec[i - 2].chars().nth(j).unwrap() == 'A'
                    && vec[i - 3].chars().nth(j).unwrap() == 'S'
                {
                    count += 1;
                }

                // right horizontal
                if j + 3 < vec[i].len()
                    && vec[i].chars().nth(j + 1).unwrap() == 'M'
                    && vec[i].chars().nth(j + 2).unwrap() == 'A'
                    && vec[i].chars().nth(j + 3).unwrap() == 'S'
                {
                    count += 1;
                }

                // left horizontal
                if j >= 3
                    && vec[i].chars().nth(j - 1).unwrap() == 'M'
                    && vec[i].chars().nth(j - 2).unwrap() == 'A'
                    && vec[i].chars().nth(j - 3).unwrap() == 'S'
                {
                    count += 1;
                }

                // right down diagonal
                if i + 3 < vec.len()
                    && j + 3 < vec[i].len()
                    && vec[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && vec[i + 2].chars().nth(j + 2).unwrap() == 'A'
                    && vec[i + 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    count += 1;
                }

                // left down diagonal
                if i + 3 < vec.len()
                    && j >= 3
                    && vec[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && vec[i + 2].chars().nth(j - 2).unwrap() == 'A'
                    && vec[i + 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    count += 1;
                }

                // right up diagonal
                if i >= 3
                    && j + 3 < vec[i].len()
                    && vec[i - 1].chars().nth(j + 1).unwrap() == 'M'
                    && vec[i - 2].chars().nth(j + 2).unwrap() == 'A'
                    && vec[i - 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    count += 1;
                }

                // left up diagonal
                if i >= 3
                    && j >= 3
                    && vec[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && vec[i - 2].chars().nth(j - 2).unwrap() == 'A'
                    && vec[i - 3].chars().nth(j - 3).unwrap() == 'S'
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
