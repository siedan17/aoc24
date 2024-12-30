use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("./input1.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut g = File::open("./input2.txt").expect("filee not found");
    let mut contents2 = String::new();
    g.read_to_string(&mut contents2).expect("went wrong");

    let lines = contents.split("\n");
    let lines2 = contents2.split("\n");


    let mut map: HashMap<(i32, i32), ()> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split('|').collect();
        let left = parts[0].parse::<i32>().expect("left");
        let right = parts[1].parse::<i32>().expect("right");
        let tuple: (i32, i32) = (left, right);
        map.insert(tuple, ());
    }

    let mut matrix = Vec::<Vec<i32>>::new();
    for line in lines2 {
        let mut row = Vec::<i32>::new();

        let parts: Vec<&str> = line.split(',').collect();
        for part in parts {
            let num = part.parse::<i32>().expect("num");
            row.push(num);
        }
        matrix.push(row);
    }

    let mut counter: i32 = 0;
    for row in matrix {
        let (b, k) = check_row(row.clone(), map.clone());
        if b {
            continue
            // let c: i32 = row[row.len()/2];
            // counter += c;
        } else {
            let mut new_row = row.clone();
            let mut bo = b;
            let mut idx = k;
            while !bo {
                new_row.swap(idx.0, idx.1);
                let (x, y) = check_row(new_row.clone(), map.clone());
                bo = x;
                idx = y;
            }
            let c: i32 = new_row[new_row.len()/2];
            counter += c;
        }
    }

    println!("{}", counter);


}

fn check_row(row: Vec<i32>, keys: HashMap<(i32, i32), ()>) -> (bool, (usize, usize)) {
    for i in 0..row.len() {
        for j in i+1..row.len() {
            let tuple: (i32, i32) = (*row.get(j).unwrap_or(&0), *row.get(i).unwrap_or(&0));
            if keys.contains_key(&tuple) {
                return (false, (i, j))
            }
        }
    }
    (true, (0, 0))
}


