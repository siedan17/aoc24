use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut board = Vec::new();
    let mut postition_map = HashMap::new();
    for i in 0..lines.len() {
        let mut row = Vec::new();
        let line: Vec<char> = lines[i].chars().collect();
        for j in 0..line.len() {
            let c = line[j];
            row.push(c);
            if c != '.' {
                if !postition_map.contains_key(&c) {
                    let new_vec = Vec::new();
                    postition_map.insert(c, new_vec);
                }
                postition_map.entry(c).and_modify(|v| v.push((j as i32, i as i32)));
            }
        }
        board.push(row);
    }
    // println!("pos_map: {:?}", postition_map);
    // println!("board: {:?}", board);

    let mut list_of_nodes = HashMap::new();
    for (_key, value) in &postition_map {
        calc_num_nodes(value.to_vec(), &mut list_of_nodes, board[0].len() as i32, board.len() as i32);
    }

    println!("result: {}", list_of_nodes.len());
    // println!("board:");
    // for (key, _v) in &list_of_nodes {
    //     let c = board[key.1 as usize][key.0 as usize];
    //     if c == '.' {
    //         board[key.1 as usize][key.0 as usize] = '#';
    //     }
    // }
    // for line in &board {
    //     println!("{:?}", line);
    // }
}

fn calc_num_nodes(input: Vec<(i32, i32)>, map: &mut HashMap<(i32, i32), ()>, max_width: i32, max_height: i32) {
    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            let a = input[i];
            let b = input[j];

            let mut new_a = a;
            let mut new_b = b;
            while new_a.0 < max_width && new_a.0 > -1 && new_a.1 < max_height && new_a.1 > -1 {
                map.insert(new_a, ());
                new_a = (new_a.0 + (a.0 - b.0), new_a.1 + (a.1 - b.1));
            }
            while new_b.0 < max_width && new_b.0 > -1 && new_b.1 < max_height && new_b.1 > -1 {
                map.insert(new_b,());
                new_b = (new_b.0 + (b.0 - a.0), new_b.1 + (b.1 - a.1));
            }
        }
    }
}
