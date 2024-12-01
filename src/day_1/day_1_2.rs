use std::{collections::HashMap, fs::read_to_string};

fn read_lines(filename: &str) -> (Vec<i32>,HashMap<i32,i32>) {
    let mut vec = Vec::new();
    let mut count: HashMap<i32,i32> = HashMap::new();


    for line in read_to_string(filename).unwrap().lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        
        vec.push(numbers[0].parse().expect("error while parsing number 1")); 
        *count.entry(numbers[1].parse().expect("error while parsing number 2")).or_default() += 1;
    }

    (vec,count)
}

fn main() {
    let (vec, count) =read_lines("input_day_1.txt");


    let mut sum: i32 = 0; 

    for number in vec {
        sum += count.get(&number).unwrap_or_else(|| {
            return  &0;
        }) * number;
    }
    

    println!("{}", sum);
}
