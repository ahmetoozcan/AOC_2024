use std::fs::read_to_string;
use std::time::Instant;

fn freq(
    antehna: char,
    antehna_index: (usize, usize),
    map: &Vec<Vec<char>>,
    antinode_map: &mut Vec<Vec<char>>,
) {
    for x in antehna_index.0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == antehna && (x, y) != antehna_index {
                let diff: (usize, usize) =
                    (x.abs_diff(antehna_index.0), y.abs_diff(antehna_index.1));
                let mut freq_first: (i32, i32) = (0, 0);
                let mut freq_second: (i32, i32) = (0, 0);
                if antehna_index.1 > y {
                    freq_first = (
                        (antehna_index.0 as i32 - diff.0 as i32) as i32,
                        (antehna_index.1 as i32 + diff.1 as i32) as i32,
                    );
                    freq_second = (
                        (x as i32 + diff.0 as i32) as i32,
                        (y as i32 - diff.1 as i32) as i32,
                    );
                } else {
                    freq_first = (
                        (antehna_index.0 as i32 - diff.0 as i32) as i32,
                        (antehna_index.1 as i32 - diff.1 as i32) as i32,
                    );
                    freq_second = (
                        (x as i32 + diff.0 as i32) as i32,
                        (y as i32 + diff.1 as i32) as i32,
                    );
                }

                if freq_first.0 >= 0
                    && freq_first.1 >= 0
                    && freq_first.0 < antinode_map.len() as i32
                    && freq_first.1 < antinode_map[0].len() as i32
                {
                    println!("{}-{}", freq_first.0, freq_first.1);
                    antinode_map[freq_first.0 as usize][freq_first.1 as usize] = '#';
                }
                if freq_second.0 >= 0
                    && freq_second.1 >= 0
                    && freq_second.0 < antinode_map.len() as i32
                    && freq_second.1 < antinode_map[0].len() as i32
                {
                    println!("{}-{}", freq_second.0, freq_second.1);
                    antinode_map[freq_second.0 as usize][freq_second.1 as usize] = '#';
                }
            }
        }
    }
}

fn calculate(filename: &str) -> i64 {
    let map: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut antinode_map = vec![vec!['.'; map[0].len()]; map.len()];

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != '.' {
                freq(map[x][y], (x, y), &map, &mut antinode_map);
            }
        }
    }
    let mut count = 0;
    for x in 0..antinode_map.len() {
        // println!("{:?}", antinode_map[x]);
        for y in 0..antinode_map[0].len() {
            if antinode_map[x][y] == '#' {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let start = Instant::now();
    println!("{}", calculate("src/input.txt"));
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
