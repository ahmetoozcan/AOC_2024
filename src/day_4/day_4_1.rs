use std::fs::read_to_string;

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let vec: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|c| c.trim().chars().collect())
        .collect();

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] == 'X' {
                // Could've done better

                // left horizontal
                if j >= 3 && vec[i][j - 1] == 'M' && vec[i][j - 2] == 'A' && vec[i][j - 3] == 'S' {
                    count += 1;
                }

                // up vertical
                if i >= 3 && vec[i - 1][j] == 'M' && vec[i - 2][j] == 'A' && vec[i - 3][j] == 'S' {
                    count += 1;
                }

                // down vertical
                if i + 3 < vec.len()
                    && vec[i + 1][j] == 'M'
                    && vec[i + 2][j] == 'A'
                    && vec[i + 3][j] == 'S'
                {
                    count += 1;
                }

                // right horizontal
                if j + 3 < vec[i].len()
                    && vec[i][j + 1] == 'M'
                    && vec[i][j + 2] == 'A'
                    && vec[i][j + 3] == 'S'
                {
                    count += 1;
                }

                // right down diagonal
                if i + 3 < vec.len()
                    && j + 3 < vec[i].len()
                    && vec[i + 1][j + 1] == 'M'
                    && vec[i + 2][j + 2] == 'A'
                    && vec[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }

                // left down diagonal
                if i + 3 < vec.len()
                    && j >= 3
                    && vec[i + 1][j - 1] == 'M'
                    && vec[i + 2][j - 2] == 'A'
                    && vec[i + 3][j - 3] == 'S'
                {
                    count += 1;
                }

                // right up diagonal
                if i >= 3
                    && j + 3 < vec[i].len()
                    && vec[i - 1][j + 1] == 'M'
                    && vec[i - 2][j + 2] == 'A'
                    && vec[i - 3][j + 3] == 'S'
                {
                    count += 1;
                }

                // left up diagonal
                if i >= 3
                    && j >= 3
                    && vec[i - 1][j - 1] == 'M'
                    && vec[i - 2][j - 2] == 'A'
                    && vec[i - 3][j - 3] == 'S'
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
