use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let lines = contents.split("\n");
    let mut map = HashMap::new();
    for line in lines {
        parse_string(line, &mut map);
    }

    let mut counter = 0;

    for (key, value) in map.iter() {
        if check_row(*key, value.clone()) {
            counter += key;
        }
    }

    println!("result: {}", counter)

}

fn check_row(key: i64, values: Vec<i64>) -> bool {
    let options: Vec<char> = ['+', '*', '|'].to_vec();
    let mut combinations = Vec::new();

    let mut current_combination = Vec::new();
    generate_combinations(&options, values.len()-1, &mut current_combination, &mut combinations);

    // println!("{:?}, {:?}", combinations, values);

    let mut check_value: i64;
    for comb in combinations {
        check_value = values[0];
        for (i, op) in comb.iter().enumerate() {
            if *op == '+' {
                check_value += values[i+1];
            }
            if *op == '*' {
                check_value *= values[i+1];
            }
            if *op == '|' {
                check_value = concat_int(check_value, values[i+1]);
            }
        }
        if check_value == key {
            return true
        }
    }
    false
}

fn concat_int(a: i64, b: i64) -> i64 {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let combined_str = a_str + &b_str;

    combined_str.parse::<i64>().unwrap()
}

fn generate_combinations(options: &Vec<char>, length: usize, current: &mut Vec<char>, combinations: &mut Vec<Vec<char>>) {
    if current.len() == length {
        combinations.push(current.clone());
        return;
    }

    for &option in options {
        current.push(option);
        generate_combinations(options, length, current, combinations);
        current.pop();
    }
}


fn parse_string(input: &str, map: &mut HashMap<i64, Vec<i64>>) {
    let mut parts = input.split(':');

    let key_str = parts.next().unwrap().trim();
    let value_str = parts.next().unwrap().trim();
    let key: i64 = key_str.parse().unwrap();

    let values: Vec<i64> = value_str
        .split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    map.entry(key)
        .or_insert_with(Vec::new)
        .extend(values.into_iter());
}