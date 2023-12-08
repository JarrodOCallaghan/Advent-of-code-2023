use std::collections::HashSet;

// struct Schematic {
//     min: Coordinate,
//     max: Coordinate
// }

#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    row: i32,
    col: i32,
}

// Gets data var with Vector of Vectors of chars a.k.a 2d char array
pub fn solve(data: &Vec<Vec<char>>) {
    let mut visited_coords: HashSet<Coordinate> = HashSet::new();
    let mut total: i32 = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if !c.is_digit(10) && !(c == &'.') {
                println!("{c}");
                let sum = 0;
                let (sum, _fin_visited) = traverse_and_sum(
                    &data,
                    sum,
                    &mut visited_coords,
                    Coordinate {
                        row: i as i32,
                        col: j as i32,
                    },
                );
                print!("{total} + {sum}");
                total += sum;
                println!(" = {total}");
                
            }
        }
    }
    println!("{total}");
}

fn traverse_and_sum<'a>( data: &Vec<Vec<char>>, sum: i32, mut visited_coords: &'a mut HashSet<Coordinate>, current: Coordinate) -> (i32, &'a mut HashSet<Coordinate>) {
    let mut sum: i32 = sum;
    // Skip nodes we have already visited
    if !visited_coords.contains(&current) {
        //Lets get the current char:
        let current_char = data[current.row as usize][current.col as usize];
        visited_coords.insert(Coordinate {
            row: current.row,
            col: current.col,
        });

        if !(current_char == '.') {
            // Lets do our code here

            print!("Found: {current_char} @ {}x{}\n",current.col + 1,current.row + 1);
            if current_char.is_digit(10) {
                // println!("{sum} + {num}");
                sum += current_char.to_digit(10).unwrap_or(0) as i32;
            }
            for i in -1..2 {
                for j in -1..2 {
                    let next_coord = Coordinate {
                        row: current.row + i,
                        col: current.col + j,
                    };
                    // print!("{},{}|||", next_coord.row, next_coord.col);
                    if (0 < next_coord.row && next_coord.row < data[0].len() as i32)
                        && (0 < next_coord.col && next_coord.col < data.len() as i32)
                    {
                        (sum, visited_coords) = traverse_and_sum(data, sum, visited_coords, next_coord);
                    }
                }
            }
        }
    }
    (sum, visited_coords)
}
