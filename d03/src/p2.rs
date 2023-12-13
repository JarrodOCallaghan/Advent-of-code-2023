#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Coordinate {
    line: i32,
    char_pos: i32,
}

use std::collections::HashMap;
pub fn solve(data: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let mut gears: HashMap<Coordinate, Vec<i32>> = Default::default();
    
    for (i, line) in data.iter().enumerate() {
        let mut is_valid: bool = false;
        let mut number: i32 = 0;
        let mut coord: Option<Coordinate> = Some(Coordinate{char_pos: 0, line: 0});
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                let n = c.to_digit(10).unwrap_or(0) as i32;
                if number == 0 {
                    number = n;
                } else {
                    number = number * 10 + n;
                }
                let current = Coordinate {
                    line: i as i32,
                    char_pos: j as i32,  
                };
                // print!("GOT HERE");
                // let coord: Option<Coordinate>;

                let mut validate = false;
                if is_valid == false{
                    (validate, coord) = check_perimeter_for_special_char(data, current);
                }
                
                if validate {
                    is_valid = true;

                    // println!("{is_valid}, {}");
                    // is_valid = true;

                    // println!("True");
                }
                if is_valid{
                    // println!("{number}");
                    let test = coord.unwrap();
                    // println!("FINAL COORD: {} x {}", test.line, test.char_pos);
                }
                // println!("{}", c);
            } else {
                if number > 0 && is_valid {

                    // Lets check if the hashmap has value
                    let coord = coord.unwrap_or(Coordinate{char_pos:0,line:0});
                    if let Some(x) = gears.get_mut(&coord){
                        x.push(number);
                    } else {
                        gears.insert(
                            coord,
                            vec![number]
                        );
                    }
                    
                    // let test = gears.get(&coord);

                    // gears.get(coord.unwrap()).push(number)
                }
                if is_valid {
                    // total += number;
                    is_valid = false;
                }
                number = 0
            }
        }
    }
    for (k,v) in gears{
        let mut sub_total: i32 = 0;
        if v.len() >= 2{
            for item in v{
                println!("{item}");
                if sub_total == 0 {
                    sub_total = item;
                } else {
                    sub_total *= item;
                }
            }
            println!("{sub_total}");
            total += sub_total;
        }
    }
    println!("P2 Total: {total}");
}

fn check_perimeter_for_special_char(data: &Vec<Vec<char>>, current: Coordinate) -> (bool, Option<Coordinate>) {
    // println!("Char ?: {}", data[current.char_pos as usize][current.line as usize]);
    for i in -1..2 {
        for j in -1..2 {
            let next_coord = Coordinate {
                line: current.line + i,
                char_pos: current.char_pos + j,
            };
            if is_coord_in_range(&next_coord, data.len(), data[0].len()) {
                let c = data[next_coord.line as usize][next_coord.char_pos as usize];
                if c == '*' {
                    // println!("CHAR: {c}: ");
                    // println!("{} x {}", next_coord.line, next_coord.char_pos);
                    return (true, Some(next_coord));
                }
            }
        }
    }
    return (false, None);
}

fn is_coord_in_range(next_coord: &Coordinate, max_line: usize, max_char_pos: usize) -> bool {
    if (0 <= next_coord.line && next_coord.line < max_line as i32)
        && (0 <= next_coord.char_pos && next_coord.char_pos < max_char_pos as i32)
    {
        return true;
    }
    return false;
}
