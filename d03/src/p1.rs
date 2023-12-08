#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

pub fn solve(data: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    for (i, row) in data.iter().enumerate() {
        let mut is_valid: bool = false;
        let mut number: i32 = 0;
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
                if check_perimeter_for_special_char(data, current) {
                    is_valid = true;
                    // println!("True");
                }
                // println!("{}", c);
            } else {
                if number > 0 && is_valid {
                    // println!("\nValid num: {number}");
                }
                if is_valid {
                    total += number;
                    is_valid = false;
                }
                number = 0
            }
        }
    }
    println!("\nTotal: {total}");
}

fn check_perimeter_for_special_char(data: &Vec<Vec<char>>, current: Coordinate) -> bool {
    // println!("Char ?: {}", data[current.col as usize][current.row as usize]);
    for i in -1..2 {
        for j in -1..2 {
            let next_coord = Coordinate {
                row: current.row + i,
                col: current.col + j,
            };
            if is_coord_in_range(&next_coord, data.len(), data[0].len()) {
                let c = data[next_coord.col as usize][next_coord.row as usize];
                if !(c.is_digit(10)) && !(c == '.') && !(c == '\n') {
                    // println!("CHAR: {c}: ");
                    // println!("{} x {}", next_coord.col, next_coord.row);
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_coord_in_range(next_coord: &Coordinate, max_col: usize, max_row: usize) -> bool {
    if (0 <= next_coord.row && next_coord.row < max_row as i32)
        && (0 <= next_coord.col && next_coord.col < max_col as i32)
    {
        return true;
    }
    return false;
}
