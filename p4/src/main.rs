use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let lines = contents.split("\n");

    let mut matrix = Vec::<Vec<char>>::new();
    for line in lines {
        let mut row =  Vec::<char>::new();
        for (_, c) in line.chars().enumerate() {
            row.push(c);
        }
        matrix.push(row);
    }

    let mut counter = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'A' {
            //     horizontal_right(matrix.clone(), j, i, &mut counter);
            //     horizontal_left(matrix.clone(), j, i, &mut counter);
            //     vertical_down(matrix.clone(), j, i, &mut counter);
            //    vertical_up(matrix.clone(), j, i, &mut counter);
            //    left_down(matrix.clone(), j, i, &mut counter);
            //    left_up(matrix.clone(), j, i, &mut counter);
            //     right_down(matrix.clone(), j, i, &mut counter);
            //     right_up(matrix.clone(), j, i, &mut counter);
            if is_x(matrix.clone(), j, i) {
                counter += 1;
            }
            }
        }
    }
    println!("{}", counter);
}

fn is_x(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize) -> bool {
    let width = matrix[0].len();
    let height = matrix.len();

    if pos_x + 2 > width || pos_x < 1 || pos_y < 1 || pos_y + 2 > height {
        return false
    }
    if (matrix[pos_y-1][pos_x-1] == 'M' && matrix[pos_y+1][pos_x+1] == 'S' ||
        matrix[pos_y-1][pos_x-1] == 'S' && matrix[pos_y+1][pos_x+1] == 'M') && (
            matrix[pos_y+1][pos_x-1] == 'M' && matrix[pos_y-1][pos_x+1] == 'S' ||
        matrix[pos_y+1][pos_x-1] == 'S' && matrix[pos_y-1][pos_x+1] == 'M'
        ) {
        return true
    }
    return false
}

// fn horizontal_right(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let width = matrix[0].len();
//     if pos_x + 4 > width {
//         return
//     }
//     if matrix[pos_y][pos_x+1] == 'M' && matrix[pos_y][pos_x+2] == 'A' && matrix[pos_y][pos_x+3] == 'S' {
//         *counter += 1;
//     }
// }

// fn horizontal_left(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let w: usize = 3;
//     if pos_x < w {
//         return
//     }
//     if matrix[pos_y][pos_x-1] == 'M' && matrix[pos_y][pos_x-2] == 'A' && matrix[pos_y][pos_x-3] == 'S' {
//         *counter += 1;
//     }
// }

// fn vertical_up(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let h: usize = 3;
//     if pos_y < h {
//         return
//     }
//     if matrix[pos_y-1][pos_x] == 'M' && matrix[pos_y-2][pos_x] == 'A' && matrix[pos_y-3][pos_x] == 'S' {
//         *counter += 1;
//     }

// }

// fn vertical_down(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let height = matrix.len();
//     if pos_y + 4 > height {
//         return
//     }
//     if matrix[pos_y+1][pos_x] == 'M' && matrix[pos_y+2][pos_x] == 'A' && matrix[pos_y+3][pos_x] == 'S' {
//         *counter += 1;
//     }

// }

// fn right_down(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let height = matrix.len();
//     let width = matrix[0].len();
//     if pos_y + 4 > height || pos_x + 4 > width {
//         return
//     }
//     if matrix[pos_y+1][pos_x+1] == 'M' && matrix[pos_y+2][pos_x+2] == 'A' && matrix[pos_y+3][pos_x+3] == 'S' {
//         *counter += 1;
//     }

// }

// fn left_up(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let z: usize = 3;
//     if pos_y < z || pos_x < z {
//         return
//     }
//     if matrix[pos_y-1][pos_x-1] == 'M' && matrix[pos_y-2][pos_x-2] == 'A' && matrix[pos_y-3][pos_x-3] == 'S' {
//         *counter += 1;
//     }

// }

// fn left_down(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let z: usize = 3;
//     let height = matrix.len();
//     if pos_y + 4 > height || pos_x < z {
//         return
//     }
//     if matrix[pos_y+1][pos_x-1] == 'M' && matrix[pos_y+2][pos_x-2] == 'A' && matrix[pos_y+3][pos_x-3] == 'S' {
//         *counter += 1;
//     }

// }

// fn right_up(matrix: Vec<Vec<char>>, pos_x: usize, pos_y: usize, counter: &mut i32) {
//     let z: usize = 3;
//     let width = matrix[0].len();
//     if pos_y < z || pos_x + 4 > width {
//         return
//     }
//     if matrix[pos_y-1][pos_x+1] == 'M' && matrix[pos_y-2][pos_x+2] == 'A' && matrix[pos_y-3][pos_x+3] == 'S' {
//         *counter += 1;
//     }

// }

