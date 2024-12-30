use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.split("\n");

    let mut counter = 0;
    for line in lines {
        let entries: Vec<&str> = line.split_whitespace().collect();
        let result: Result<Vec<i32>, _> = entries.iter().map(|s| s.parse::<i32>()).collect();
        let mut found = false;
        match result {
            Ok(ints) => {
                if is_increasing(ints.clone()) {
                    counter += 1;
                    println!("{:?}", ints);
                    found = true
                }
                let mut reversed = ints.clone();
                reversed.reverse();
                if is_increasing(reversed.clone()) && !found {
                    // println!("reversed {:?}, {:?}", ints, reversed.clone());
                    counter += 1
                }
            }
            Err(msg) => {
                eprintln!("Error parsing integers: {}", msg);
            }
        }

    }

    println!("result {}", counter)
}

fn is_increasing(vec: Vec<i32>) -> bool {
    let mut c = 0;
    for i in 0..vec.len() {
        let mut new = vec.clone();
        new.remove(i);
        for j in 1..new.len() {
            let a = new[j-1];
            let b = new[j];
            let diff = b-a;
            if diff < 1 || diff > 3 {
                if c > vec.len()-2 {
                    return false
                }
                c += 1;
                break;
            }
        }
    }
    true
}
