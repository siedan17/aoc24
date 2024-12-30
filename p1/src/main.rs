use std::fs::File;
use std::io::prelude::*;

fn count_occurrences(number: i32, numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&n| *n == number).count() as i32
}

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.split("\n");

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();


    for line in lines {
        let entries: Vec<&str> = line.split_whitespace().collect();
        if let [first, second] = entries.as_slice() {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                left_vec.push(num1);
                right_vec.push(num2);
            }
        }
    }

    left_vec.sort();
    right_vec.sort();

    let mut result = 0;
    for num in left_vec {
        result += num * count_occurrences(num, &right_vec)
    }

    println!("result: {}", result)
}
