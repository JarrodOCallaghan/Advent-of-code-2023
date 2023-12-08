#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

use std::collections::HashMap;

pub fn solve(data: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let mut gear_ratios: HashMap<Coordinate, Vec<i32>> = HashMap::new();

    for (i, row) in data.iter().enumerate() {
        let mut is_valid: bool = false;
        let mut number: i32 = 0;
        let mut saved_coord: Coordinate;
        for (j, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                let n = c.to_digit(10).unwrap_or(0) as i32;
                if number == 0 {
                    number = n;
                } else {
                    number = number * 10 + n;
                }
                let current = Coordinate {
                    col: i as i32,
                    row: j as i32,
                };
                // print!("GOT HERE");
                let (validity, coord) = check_perimeter_for_gear(data, current);
                saved_coord = coord;

                if validity {
                    is_valid = true;
                    // gear_ratios.insert(coord, Vec::new());
                }

            } else {
                if is_valid {
                    // let mut key_vector: Vec<i32> = gear_ratios.get(&saved_coord);
                    // key_vector.push(number);
                    // gear_ratios.insert(saved_coord, key_vector);
                    is_valid = false;
                }
                number = 0
            }
        }
    }
    println!("P2 total: {total}");
}


fn check_perimeter_for_gear(data: &Vec<Vec<char>>, current: Coordinate) -> (bool, Coordinate) {
    let mut validity: bool;

    for i in -1..2 {
        for j in -1..2 {
            let next_coord = Coordinate {
                col: current.col + i,
                row: current.row + j,
            };

            if is_coord_in_range(&next_coord, data.len(), data[0].len()) {
                let c = data[next_coord.col as usize][next_coord.row as usize];
                if c == '*'{
                    return (true, next_coord);
                }
            }

        }
    }
    return (false, current);
}

fn is_coord_in_range(next_coord: &Coordinate, max_col: usize, max_row: usize) -> bool {
    if (0 <= next_coord.row && next_coord.row < max_row as i32)
        && (0 <= next_coord.col && next_coord.col < max_col as i32)
    {
        return true;
    }
    return false;
}
