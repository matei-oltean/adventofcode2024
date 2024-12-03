use std::fs;

use regex::Regex;

fn sum_mult(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|caps| caps.extract())
        .map(|(_, [x, y])| x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn main() {
    let input = fs::read_to_string("./input/03")
        .unwrap()
        .lines()
        .collect::<String>();

    println!("Result: {}", sum_mult(input.as_str()));

    let re = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    let result = re
        .captures_iter(&input)
        .map(|caps| caps.extract())
        .map(|(_, [s])| sum_mult(s))
        .sum::<i32>();
    println!("Result: {}", result);
}
