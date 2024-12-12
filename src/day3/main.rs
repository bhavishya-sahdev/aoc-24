use regex::Regex;
use std::fs;

pub fn main() {
    let content =
        fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");
    println!("{}", extract_product(content));
}

fn extract_product(content: String) -> u32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut res = 0;
    let mut enabled = true;
    for cap in regex.captures_iter(&content) {
        if let Some(mul_match) = cap.get(1) {
            if !enabled {
                continue;
            }

            let num1: u32 = mul_match.as_str().parse().unwrap();
            let num2: u32 = cap[2].parse().unwrap();
            res += num1 * num2;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else if cap.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        }
    }
    res
}
