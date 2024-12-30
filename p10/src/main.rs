use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut board = Vec::new();
    for i in 0..lines.len() {
        let line: Vec<char> = lines[i].chars().collect();
        let ints: Vec<i32> = line.into_iter().map(|c| c.to_digit(10).unwrap() as i32).collect();
        board.push(ints);
    }
    // println!("{:?}", board);
    let mut starts: Vec<(i32, i32)> = vec![];
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == 0 {
                starts.push((i as i32, j as i32));
            }
        }
    }
    // println!("{:?}", starts);

    let mut total_score: i32 = 0;
    for i in 0..starts.len() {
        let score = calc_score(board.clone(), starts[i].clone());
        // println!("new score: {}", score);
        total_score += score;
    }
    println!("total score: {}", total_score);
}

fn calc_score(board: Vec<Vec<i32>>, start: (i32, i32)) -> i32 {
    let mut current_positions: Vec<(i32, i32)> = vec![start.clone()];
    let mut end_positions: Vec<(i32, i32)> = vec![];
    while current_positions.len() > 0 {
        let mut new_positions: Vec<(i32, i32)> = vec![];
        for i in 0..current_positions.len() {
            let value = board[current_positions[i].0 as usize][current_positions[i].1 as usize];
            if current_positions[i].0 + 1 < board.len() as i32 {
                if board[(current_positions[i].0 + 1) as usize][(current_positions[i].1) as usize] == value + 1 {
                    if value + 1 == 9 {
                        end_positions.push((current_positions[i].0 + 1, current_positions[i].1));
                    } else {
                        new_positions.push((current_positions[i].0 + 1, current_positions[i].1))
                    }
                }
            }
            if current_positions[i].0 - 1 > -1 {
                if board[(current_positions[i].0 - 1) as usize][(current_positions[i].1) as usize] == value + 1 {
                    if value + 1 == 9 {
                        end_positions.push((current_positions[i].0 - 1, current_positions[i].1));
                    } else {
                        new_positions.push((current_positions[i].0 - 1, current_positions[i].1))
                    }
                }
            }
            if current_positions[i].1 - 1 > -1 {
                if board[(current_positions[i].0) as usize][(current_positions[i].1 - 1) as usize] == value + 1 {
                    if value + 1 == 9 {
                        end_positions.push((current_positions[i].0, current_positions[i].1 - 1));
                    } else {
                        new_positions.push((current_positions[i].0, current_positions[i].1 - 1))
                    }
                }
            }
            if current_positions[i].1 + 1 < board[0].len() as i32 {
                if board[(current_positions[i].0) as usize][(current_positions[i].1 + 1) as usize] == value + 1 {
                    if value + 1 == 9 {
                        end_positions.push((current_positions[i].0, current_positions[i].1 + 1));
                    } else {
                        new_positions.push((current_positions[i].0, current_positions[i].1 + 1))
                    }
                }
            }
        }
        current_positions = new_positions;
    }
    end_positions.len() as i32
}