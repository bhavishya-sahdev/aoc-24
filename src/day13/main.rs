use regex::Regex;
use std::{collections::HashMap, fs};

type IndexPair = (u64, u64);
type Grid = HashMap<(u64, u64), [IndexPair; 2]>;

pub fn main() {
    let content = fs::read_to_string("./src/day13/input.txt")
        .expect("Should have been able to read the file");

    let content = content.split("\n\n").collect::<Vec<&str>>();
    let mut grid: Grid = HashMap::new();

    // Create regex patterns for button coordinates
    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    for group in content {
        let lines: Vec<&str> = group.lines().collect();
        if let (Some(button_a), Some(button_b), Some(prize)) = (
            button_regex.captures(lines[0]),
            button_regex.captures(lines[1]),
            prize_regex.captures(lines[2]),
        ) {
            let button_a_pos = (
                button_a[1].parse::<u64>().unwrap(),
                button_a[2].parse::<u64>().unwrap(),
            );
            let button_b_pos = (
                button_b[1].parse::<u64>().unwrap(),
                button_b[2].parse::<u64>().unwrap(),
            );
            let prize_pos = (
                prize[1].parse::<u64>().unwrap(),
                prize[2].parse::<u64>().unwrap(),
            );

            grid.insert(prize_pos, [button_a_pos, button_b_pos]);
        }
    }
    println!("{}", move_claw(&grid));
}

fn move_claw(grid: &Grid) -> i64 {
    let mut total = 0;
    let offset: u64 = 10_000_000_000_000; // 10^13

    for (target, buttons) in grid {
        let a = buttons[0];
        let b = buttons[1];
        let target = (target.0 + offset, target.1 + offset);

        let denominator = (b.1 as f64 * a.0 as f64) - (b.0 as f64 * a.1 as f64);

        if denominator == 0.0 {
            continue;
        }

        let times_b =
            ((target.1 as f64 * a.0 as f64) - (target.0 as f64 * a.1 as f64)) / denominator;
        let times_a = (target.0 as f64 - (b.0 as f64 * times_b)) / (a.0 as f64);

        if times_a.fract() == 0.0 && times_b.fract() == 0.0 && times_a >= 0.0 && times_b >= 0.0 {
            total += times_a as i64 * 3 + times_b as i64;
        }
    }
    total
}
