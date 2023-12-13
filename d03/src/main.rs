use std::fs::read_to_string;
mod p1;
mod p2;

fn main() {
    let data = read_data("data.txt");
    p1::solve(&data);
    p2::solve(&data);
}

fn read_data(filename: &str) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    // let test = read_to_string(filename).expect("Didn't work");
    for line in read_to_string(filename).unwrap().lines() {
        let line = &(line.to_string() + ".");
        res.push(line.to_string().chars().collect());
    }

    res
}
