use std::fs::read_to_string;
mod p1;
mod p2;

fn main() {
    let data = read_data("data.txt");
    p1::solve(&data);
    p2::solve(&data);
}


fn read_data(filename: &str) -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string())
    }

    res
}