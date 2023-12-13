#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn solve(data: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let mut gear_ratios: HashMap<Coordinate, Vec<i32>> = HashMap::new();

    // Our plan for this one is to scan for whole numbers that have an adjacent gear
    // When we find a number with adjacent gear. Insert gear coord into hashmap as key, value is vector of numbers.
    // Once finished, each gear with 2 numbers in its vector will be multiplied, any with 1 or 0 are ignored

    // For each line, remember whole number can only exist on a single line
    for (i, row) in data.iter().enumerate() {
        let mut is_valid: bool = false;
        let mut found_number: i32 = 0;
        let mut saved_coord: Coordinate = Coordinate{col: 0, row: 0};

        for (j, character) in row.iter().enumerate() {
            if character.is_digit(10) {
                let number = character.to_digit(10).unwrap_or(0) as i32;
                if found_number == 0 {
                    found_number = number;
                } else {
                    found_number = found_number * 10 + number;
                }

                let current = Coordinate {
                    row: i as i32,
                    col: j as i32,
                };

                let (validity, coord) = check_perimeter_for_gear(data, current);
                saved_coord = coord;

                // gear_ratios.insert(
                //     saved_coord,
                //     Vec::new(),
                // );

                if validity {
                    is_valid = true;
                }
                

            } else {
                // Here is where we need to fix the incorrect insert stuff
                // When we hit a non-number, lets reset variables and set the gear location
                if is_valid {
                    // If we have a valid number
                    // Need to insert into the vector
                    println!("FOUND: {found_number}");

                    match gear_ratios.entry(saved_coord) {
                        Entry::Vacant(e) => {
                            e.insert(vec![found_number]);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(found_number);
                        }
                    }
                }
                is_valid = false;
                found_number = 0;
            }
        }
    }
    for (k, v) in gear_ratios{
        println!("{}, {}", k.row, k.col);
        // println!("{}", v.len());
        for item in v{
            println!("{item}");
        }
    }
    // println!("P2 total: {total}");
}

fn check_perimeter_for_gear(data: &Vec<Vec<char>>, current: Coordinate) -> (bool, Coordinate) {
    // let mut validity: bool;

    for i in -1..2 {
        for j in -1..2 {
            let next_coord = Coordinate {
                col: current.col + i,
                row: current.row + j,
            };

            if is_coord_in_range(&next_coord, data.len(), data[0].len()) {
                let c = data[next_coord.col as usize][next_coord.row as usize];
                if c == '*' {
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
