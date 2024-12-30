use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::vec;


fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut board = Vec::<Vec<char>>::new();
    for line in lines {
        let mut row = Vec::<char>::new();
        for (_, c) in line.chars().enumerate() {
            row.push(c);
        }
        board.push(row);
    }
    // println!("{:?}", board);

    let mut total: i32 = 0;
    let mut search_result = search_next(&mut board);
    while search_result.2 != '.' {
        // println!("search_result: {:?}", search_result);
        let mut area: i32 = 0;
        // let mut perimeter: i32 = 0;
        let mut sides: i32 = 0;
        let mut new_list = vec![search_result];
        let mut done_list: HashMap<(usize, usize), char> = HashMap::new();
        while new_list.len() > 0 {
            // println!("new list: {:?}", new_list);
            let mut new_new_list: HashMap<(usize, usize, char), char> = HashMap::new();
            for pos in new_list {
                done_list.insert((pos.0, pos.1), pos.2);
                area += 1;
                // perimeter += 4;

                let trial_pos = get_neighbors(board.clone(), pos);
                // println!("pos, trial_pos: {:?}, {:?}", pos, trial_pos);

                for try_pos in trial_pos {
                    // perimeter -= 1;
                    if !done_list.contains_key(&(try_pos.0, try_pos.1)) && !new_new_list.contains_key(&try_pos) {
                        new_new_list.insert(try_pos, try_pos.2);
                    }
                }
            }
            // println!("map: {:?}, {}", new_new_list, perimeter);
            new_list = new_new_list.keys().cloned().collect();
            // println!("nex new list: {:?}", new_list);
        }

        let mut new_board = Vec::<Vec<char>>::new();
        for i in 0..board.len() + 2{
            let mut row = Vec::<char>::new();
            for j in 0..board[0].len() + 2 {
                if i > 0 && j > 0 {
                    if done_list.contains_key(&(j-1, i-1)) {
                        row.push('x');
                        continue;
                    };
                };
                row.push('.');
            }
            new_board.push(row);
        }

        for x in 0..new_board[0].len()-1 {
            for y in 0..new_board.len()-1 {
                let a = new_board[y][x];
                let b = new_board[y][x+1];
                let c = new_board[y+1][x];
                let d = new_board[y+1][x+1];
                let mut counter = 0;
                if a == 'x' {
                    counter +=1;
                }
                if b == 'x' {
                    counter +=1;
                }
                if c == 'x' {
                    counter +=1;
                }
                if d == 'x' {
                    counter +=1;
                }
                if counter == 1 || counter == 3 {
                    sides += 1;
                }
                if counter == 2 {
                    if a == 'x' && d == 'x' {
                        sides += 2;
                    }
                    if b == 'x' && c == 'x' {
                        sides += 2;
                    }
                }
            }
        }


        total += area * sides;
        println!("New: {}, {}", area, sides);
        for key in done_list.keys() {
            board[key.1][key.0] = '.';
        }
        search_result = search_next(&mut board)
    }
    println!("total: {}", total);
}

fn search_next(board: &mut Vec<Vec<char>>) -> (usize, usize, char) {
    for i in 0..board[0].len() {
        for j in 0..board.len() {
            if board[j][i] != '.' {
                let letter = board[j][i];
                return (i, j, letter)
            }
        }
    }
    (9999, 9999, '.')
}

fn get_neighbors(board: Vec<Vec<char>>, pos: (usize, usize, char)) -> Vec<(usize, usize, char)> {
    let mut result =  Vec::<(usize, usize, char)>::new();
    if pos.0 > 0 {
        if board[pos.1][pos.0 - 1] == pos.2 {
            result.push((pos.0 - 1, pos.1, pos.2));
        }
    }
    if pos.0 < board[0].len() - 1 {
        if board[pos.1][pos.0 + 1] == pos.2 {
            result.push((pos.0 + 1, pos.1, pos.2));
        }
    }
    if pos.1 > 0 {
        if board[pos.1 - 1][pos.0] == pos.2 {
            result.push((pos.0, pos.1 - 1, pos.2));
        }
    }
    if pos.1 < board.len() - 1 {
        if board[pos.1 + 1][pos.0] == pos.2 {
            result.push((pos.0, pos.1 + 1, pos.2));
        }
    }
    result
}
