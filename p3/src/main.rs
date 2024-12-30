use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    let mut valid: bool = true;
    for cap in re.captures_iter(&contents) {
        if let Some(mul_cap) = cap.get(1) {
            let mul_str = mul_cap.as_str();
            if let Some(mul_caps) = mul_re.captures(mul_str) {
                let num1 = mul_caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let num2 = mul_caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

                // println!("Number 1: {}, Number 2: {}", num1, num2);
                if valid {
                    sum += num1 * num2;
                }
            }
        } else if let Some(_do_cap) = cap.get(2) {
            valid = true;
        } else if let Some(_dont_cap) = cap.get(3) {
            valid = false;
        }
    }
    println!("result: {}", sum)
}