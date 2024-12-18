use std::{collections::HashMap, fs};

pub fn main() {
    let content = fs::read_to_string("./src/day11/input.txt")
        .expect("Should have been able to read the file");

    let content: Vec<&str> = content.split_whitespace().collect();
    let mut stones: Vec<i64> = Vec::new();
    for num in content {
        stones.push(num.parse().expect("Failed to parse number"));
    }

    println!("{}", blink_stones(&stones, 75));
}

fn check_and_mutate_stone(
    stone: &i64,
    blinks: i32,
    states: &mut HashMap<(i64, i32), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(&val) = states.get(&(*stone, blinks)) {
        return val;
    }

    let digits = if *stone == 0 {
        1
    } else {
        stone.abs().to_string().len() as i32
    };

    let size = if *stone == 0 {
        check_and_mutate_stone(&1, blinks - 1, states)
    } else if digits % 2 == 0 {
        let half_digits = digits / 2;
        let power = 10_i64.pow(half_digits as u32);
        let left = stone / power;
        let right = stone % power;

        check_and_mutate_stone(&left, blinks - 1, states)
            + check_and_mutate_stone(&right, blinks - 1, states)
    } else {
        check_and_mutate_stone(&(*stone * 2024), blinks - 1, states)
    };

    states.insert((*stone, blinks), size);

    size
}

fn blink_stones(stones: &[i64], blinks: i32) -> usize {
    let mut states: HashMap<(i64, i32), usize> = HashMap::new();

    let mut size = 0;
    for stone in stones {
        size += check_and_mutate_stone(stone, blinks, &mut states);
    }

    size
}
