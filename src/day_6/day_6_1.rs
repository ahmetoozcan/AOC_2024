use std::fs::read_to_string;

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let mut map: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut guard_position: (usize, usize) = (0, 0);

    for (i, row) in map.iter().enumerate() {
        if let Some(index) = row.iter().position(|&c| c == '^') {
            guard_position.0 = i;
            guard_position.1 = index;
        }
    }

    let mut direction: Direction = Direction::Up;
    map[guard_position.0][guard_position.1] = 'X'; // Adding starting position

    while guard_position.0 < map.len() - 1
        && guard_position.1 < map[0].len() - 1
        && guard_position.0 > 0
        && guard_position.1 > 0
    {
        match direction {
            Direction::Up => {
                if map[guard_position.0 - 1][guard_position.1] == '#' {
                    direction = Direction::Right;
                    continue;
                }
                guard_position.0 -= 1;
                map[guard_position.0][guard_position.1] = 'X';
            }
            Direction::Right => {
                if map[guard_position.0][guard_position.1 + 1] == '#' {
                    direction = Direction::Down;
                    continue;
                }
                guard_position.1 += 1;
                map[guard_position.0][guard_position.1] = 'X';
            }
            Direction::Down => {
                if map[guard_position.0 + 1][guard_position.1] == '#' {
                    direction = Direction::Left;
                    continue;
                }
                guard_position.0 += 1;
                map[guard_position.0][guard_position.1] = 'X';
            }
            Direction::Left => {
                if map[guard_position.0][guard_position.1 - 1] == '#' {
                    direction = Direction::Up;
                    continue;
                }
                guard_position.1 -= 1;
                map[guard_position.0][guard_position.1] = 'X';
            }
        }
    }

    for row in map.iter() {
        for &c in row.iter() {
            if c == 'X' {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
