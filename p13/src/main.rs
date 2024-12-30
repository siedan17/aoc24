use std::fs;

const PRIZE_A: i64 = 3;
const PRIZE_B: i64 = 1;

fn main() {
    let content = fs::read_to_string("input.txt").expect("file not found");
    let mut parsed_data: Vec<((i64, i64), (i64, i64), (i64, i64))> = Vec::new();
    for block in content.split("\n\n") {
        let mut button_a = (0, 0);
        let mut button_b = (0, 0);
        let mut prize = (0, 0);
        for line in block.lines() {
            if line.starts_with("Button A:") {
                button_a = parse_coordinates(line);
            } else if line.starts_with("Button B:") {
                button_b = parse_coordinates(line);
            } else if line.starts_with("Prize:") {
                prize = parse_coordinates(line);
                prize.0 += 10000000000000;
                prize.1 += 10000000000000;
            }
        }
        parsed_data.push((button_a, button_b, prize));
    }

    let mut result: i64 = 0;
    for (_i, (a, b, p)) in parsed_data.iter().enumerate() {
        if !are_linearly_independent(*a, *b) {
            if !are_linearly_independent(*b, *p) {
                if is_divisible(p.0, b.0) {
                    result += (p.0 / b.0) * PRIZE_B;
                } else if is_divisible(p.0, a.0) {
                    result += (p.0 / a.0) * PRIZE_A;
                }
            }
            continue
        }
        match solve_linear_system(a.0, b.0, p.0, a.1, b.1, p.1) {
            Some((x, y)) => result += x * PRIZE_A + y * PRIZE_B,
            None => continue,
        }
    }
    println!("result: {}", result);
}

fn are_linearly_independent(vec1: (i64, i64), vec2: (i64, i64)) -> bool {
    vec1.0 * vec2.1 - vec1.1 * vec2.0 != 0
}

fn is_divisible(a: i64, b: i64) -> bool {
    if b == 0 {
        return false;
    }
    a % b == 0
}

fn solve_linear_system(
    a1: i64, b1: i64, c1: i64,
    a2: i64, b2: i64, c2: i64,
) -> Option<(i64, i64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }
    let det_x = c1 * b2 - c2 * b1;
    let det_y = a1 * c2 - a2 * c1;

    if det_x % det == 0 && det_y % det == 0 {
        let x = det_x / det;
        let y = det_y / det;
        Some((x, y))
    } else {
        None // No integer solution
    }
}

fn parse_coordinates(line: &str) -> (i64, i64) {
    let numbers: Vec<i64> = line
        .split(|c: char| !c.is_digit(10) && c != '-')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    (numbers[0], numbers[1])
}

