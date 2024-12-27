use std::fs::read_to_string;

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn check_position(
    mut direction: Direction,
    mut map: Vec<Vec<char>>,
    mut guard_position: (usize, usize),
) -> bool {
    while guard_position.0 < map.len() - 1
        && guard_position.1 < map[0].len() - 1
        && guard_position.0 >= 1
        && guard_position.1 >= 1
    {
        match direction {
            Direction::Up => {
                if map[guard_position.0 - 1][guard_position.1] == '#' {
                    direction = Direction::Right;
                    if map[guard_position.0][guard_position.1] == '+'
                        && !(map[guard_position.0][guard_position.1 - 1] == '#')
                    {
                        return true;
                    }
                    map[guard_position.0][guard_position.1] = '+';
                    continue;
                }
                guard_position.0 -= 1;
            }
            Direction::Right => {
                if map[guard_position.0][guard_position.1 + 1] == '#' {
                    direction = Direction::Down;
                    if map[guard_position.0][guard_position.1] == '+'
                        && !(map[guard_position.0 - 1][guard_position.1] == '#')
                    {
                        return true;
                    }
                    map[guard_position.0][guard_position.1] = '+';
                    continue;
                }
                guard_position.1 += 1;
            }
            Direction::Down => {
                if map[guard_position.0 + 1][guard_position.1] == '#' {
                    direction = Direction::Left;
                    if map[guard_position.0][guard_position.1] == '+'
                        && !(map[guard_position.0][guard_position.1 + 1] == '#')
                    {
                        return true;
                    }
                    map[guard_position.0][guard_position.1] = '+';
                    continue;
                }
                guard_position.0 += 1;
            }
            Direction::Left => {
                if map[guard_position.0][guard_position.1 - 1] == '#' {
                    direction = Direction::Up;
                    if map[guard_position.0][guard_position.1] == '+'
                        && !(map[guard_position.0 + 1][guard_position.1] == '#')
                    {
                        return true;
                    }
                    map[guard_position.0][guard_position.1] = '+';
                    continue;
                }
                guard_position.1 -= 1;
            }
        }
    }

    false
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

    let direction: Direction = Direction::Up;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '.' {
                map[x][y] = '#';
                if check_position(direction.clone(), map.clone(), guard_position.clone()) {
                    count += 1;
                }

                map[x][y] = '.';
            }
        }
    }

    count
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
