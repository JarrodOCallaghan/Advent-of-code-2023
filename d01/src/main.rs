use std::fs::read_to_string;
use std::str;

fn main(){
    let data = read_data("data.txt");
    part_1(&data);
    part_2(&data);
}

fn read_data(filename: &str) -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        res.push(line.to_string())
    }

    res
}

fn part_1(data: &Vec<String>){
    let mut found_numbers: Vec<u32> = Vec::new();
    for line in data {
        let mut first_num: u32 = 0;
        let mut last_num: u32 = 0;
        for c in line.chars(){
            if c.is_digit(10){
                first_num = c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10){
                last_num = c.to_digit(10).unwrap();
                break;
            }
        }
        found_numbers.push(first_num + last_num);
    }

    let mut sum: u32 = 0;
    for num in found_numbers{
        sum += num;
    }
    println!("Part 1: {}", sum);
}

fn part_2(data: &Vec<String>){
    let mut found_numbers: Vec<u32> = Vec::new();
    for line in data {
        let line = &str::replace(&line.to_lowercase(), "one", "one1one");
        let line = &str::replace(&line.to_lowercase(), "two", "two2two");
        let line = &str::replace(&line.to_lowercase(), "three", "three3three");
        let line = &str::replace(&line.to_lowercase(), "four", "four4four");
        let line = &str::replace(&line.to_lowercase(), "five", "five5five");
        let line = &str::replace(&line.to_lowercase(), "six", "six6six");
        let line = &str::replace(&line.to_lowercase(), "seven", "seven7seven");
        let line = &str::replace(&line.to_lowercase(), "eight", "eight8eight");
        let line = &str::replace(&line.to_lowercase(), "nine", "nine9nine");
        let line = &str::replace(&line.to_lowercase(), "zero", "zero0zero");
        let mut first_num: u32 = 0;
        let mut last_num: u32 = 0;
        for c in line.chars(){
            if c.is_digit(10){
                first_num = c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10){
                last_num = c.to_digit(10).unwrap();
                break;
            }
        }
        found_numbers.push(first_num + last_num);
    }

    let mut sum: u32 = 0;
    for num in found_numbers{
        sum += num;
    }
    println!("Part 2: {}", sum);
}