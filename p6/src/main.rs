use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let lines = contents.split("\n");
    let mut board = Vec::new();

    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        board.push(row);
    }

    // println!("{:?}", board[position.1 as usize][position.0 as usize]);
    let mut obstructions = 0;
    let mut counter = 0;

    // only iterate over fist path.
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let mut direction: i32 = 0;
            let mut position: (i32, i32) = (91, 53);
            // let mut position: (i32, i32) = (4, 6);
            let mut visited: HashMap<(i32, i32, i32), ()> = HashMap::new();
            visited.insert((position.0, position.1, direction), ());

            let temp: char = board[row][col];
            if temp == '#' || temp == '^' {
                continue
            }
            board[row][col] = '#';
            // perform custom check if this could even lead to a loop. The I don't have to simulate.

            while !update(board.clone(), &mut direction, &mut position) {
                if visited.contains_key(&(position.0, position.1, direction)) {
                    obstructions += 1;
                    println!("in here");
                    break
                }
                visited.insert((position.0, position.1, direction), ());
            }
            counter += 1;
            println!("out there, {}", counter);
            board[row][col] = temp;
        }
    }


    println!("result: {}", obstructions);


}

fn update(board: Vec<Vec<char>>, direction: &mut i32, position: &mut (i32, i32)) -> bool {
    let mut new_pos: (i32, i32) = *position;
    match *direction {
        0 => new_pos.1 -= 1,
        90 => new_pos.0 += 1,
        180 => new_pos.1 += 1,
        270 => new_pos.0 -= 1,
        _ => todo!(),
    };
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= board[0].len().try_into().unwrap() || new_pos.1 >= board.len().try_into().unwrap() {
        return true
    };
    let c: char = board[new_pos.1 as usize][new_pos.0 as usize];
    if c == '#' {
        *direction = (*direction + 90) % 360;
        return false
    }
    *position = new_pos;
    false
}