use regex::Regex;
use std::fs;

pub fn main() {
    let content =
        fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");
    println!("{}", extract_product(content));
}

fn extract_product(content: String) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    for cap in regex.captures_iter(&content) {
        let num1: u32 = cap[1].parse().unwrap();
        let num2: u32 = cap[2].parse().unwrap();
        res += num1 * num2;
    }
    res
}
