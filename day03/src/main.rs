use std::fs;

use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = fs::read_to_string("./input/03").unwrap();
    let res = re
        .captures_iter(&input)
        .map(|caps| caps.extract())
        .map(|(_, [x, y])| x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("Result: {}", res);
}
