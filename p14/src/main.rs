use std::fs;
use std::io;

const SIZE_X: i32 = 101;
const SIZE_Y: i32 = 103;
fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let result: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let p = parse_tuple(parts[0]);
            let v = parse_tuple(parts[1]);
            (p, v)
        })
        .collect();

    // let mut n = 1; x = 28; y = 55;
    let mut n = 50;
    loop {
        let mut positions: Vec<(i32, i32)> = Vec::new();
        for (p, v) in &result {
            let mut new_p: (i32, i32) = p.clone();
            let new_pos = simulate(n, &mut new_p, *v);
            positions.push(new_pos);
        }
        pretty_print_matrix((SIZE_X as usize, SIZE_Y as usize), positions.clone());

        println!("n: {}",n);

        println!("Press Enter to continue, or type 'exit' to quit.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting the program.");
            break;
        }
        n += 1;
    }

    // let mut first: i32 = 0;
    // let mut second: i32 = 0;
    // let mut third: i32 = 0;
    // let mut fourth: i32 = 0;
    // for pos in positions {
        // println!("pos: {:?}", pos);

        // if in_quadrant((1, 1), (SIZE_X/2, SIZE_Y/2), pos) {
        //     first += 1;
        // }
        // if in_quadrant((SIZE_X/2 + 1, 1), (SIZE_X, SIZE_Y/2), pos) {
        //     second += 1;
        // }
        // if in_quadrant((1, SIZE_Y/2 + 2), (SIZE_X/2, SIZE_Y), pos) {
        //     third += 1;
        // }
        // if in_quadrant((SIZE_X/2 + 2, SIZE_Y/2 + 2), (SIZE_X, SIZE_Y), pos) {
        //     fourth += 1;
        // }
    // }
    // println!("quads: {}, {}, {}, {}", first, second, third, fourth);
    // println!("result {}", first * second * third * fourth);
    Ok(())
}

fn simulate(n: i32, p: &mut (i32, i32), v: (i32, i32)) -> (i32, i32) {
    for _i in 0..n {
        let mut new_x = p.0 + v.0;
        if new_x < 0 {
            new_x += SIZE_X;
        } else if new_x > SIZE_X - 1 {
            new_x -= SIZE_X
        }
        let mut new_y = p.1 + v.1;
        if new_y < 0 {
            new_y += SIZE_Y;
        } else if new_y > SIZE_Y - 1 {
            new_y -= SIZE_Y
        }
        *p = (new_x, new_y);
    }
    *p
}

// fn in_quadrant(q1: (i32, i32), q2: (i32, i32), x: (i32, i32)) -> bool {
//     if x.0 + 1 >= q1.0 && x.0 + 1 <= q2.0 && x.1 + 1 >= q1.1 && x.1 + 1 <= q2.1 {
//         return true;
//     }
//     false
// }

fn parse_tuple(input: &str) -> (i32, i32) {
    let coords: Vec<i32> = input[2..] // Skip the "p=" or "v=" prefix
        .split(',') // Split into "x" and "y"
        .filter_map(|num| num.parse::<i32>().ok()) // Parse each number
        .collect();
    (coords[0], coords[1])
}


fn pretty_print_matrix(matrix_size: (usize, usize), positions: Vec<(i32, i32)>) {
    let (cols, rows) = matrix_size;

    // Create a 2D vector initialized with '.' for the matrix
    let mut display_matrix: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    // Count occurrences of each position
    use std::collections::HashMap;
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    for pos in positions {
        *counts.entry(pos).or_insert(0) += 1;
    }

    // Fill the display matrix with the count values as characters
    for ((row, col), count) in counts {
        let row_index = (row) as usize;
        let col_index = (col) as usize;

        if row_index < rows && col_index < cols {
            display_matrix[row_index][col_index] = match count {
                1 => '1',
                2 => '2',
                3 => '3',
                _ => '3', // Limit display to '3' if count exceeds 3
            };
        }
    }

    // Print the matrix
    for row in display_matrix {
        println!("{}", row.iter().collect::<String>());
    }
}

