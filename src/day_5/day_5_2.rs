use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn calculate(filename: &str) -> i32 {
    let mut count: i32 = 0;

    let mut before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut collecting = false;

    // File read 💀😭
    for line in read_to_string(filename).unwrap().lines() {
        // Second part of input after the empty line
        if collecting {
            updates.push(
                line.split(",")
                    .map(|num| num.trim().parse::<i32>().unwrap())
                    .collect(),
            );
            continue;
        }
        if line.trim().is_empty() {
            collecting = true;
            continue;
        }
        // Second part of input after the empty line

        // First part of input until the empty line
        let mut iter = line.split('|').map(|s| s.trim().parse::<i32>().unwrap());
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        after.entry(first).or_insert_with(Vec::new).push(second);
        before.entry(second).or_insert_with(Vec::new).push(first);
        // First part of input until the empty line
    }

    for mut update in updates {
        let mut is_not_legit: bool = false;
        let mut x: isize = 0;
        while x < update.len() as isize {
            for y in 0..update.len() {
                if y == x as usize {
                    continue;
                }
                if y > x as usize {
                    match after.get(&update[x as usize]) {
                        Some(value) => {
                            if !value.contains(&update[y]) {
                                is_not_legit = true;
                                let temp = update[y];
                                update.remove(y);
                                update.insert(x as usize, temp);

                                x -= 1;

                                break;
                            }
                        }
                        None => continue,
                    }
                } else {
                    match before.get(&update[x as usize]) {
                        Some(value) => {
                            if !value.contains(&update[y]) {
                                is_not_legit = true;
                                let temp = update[y];
                                update.remove(y);
                                update.insert(x as usize, temp);

                                x -= 1;

                                break;
                            }
                        }
                        None => continue,
                    }
                }
            }
            x += 1;
        }

        if is_not_legit {
            count += update[update.len() / 2];
        }
    }

    count
}

fn main() {
    println!("{}", calculate("src/input.txt"));
}
