use std::{collections::HashSet, fs};

pub fn main() {
    let content =
        fs::read_to_string("./src/day5/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split("\n\n").collect();
    let section1: Vec<&str> = content[0].split_whitespace().collect();
    let section2: Vec<&str> = content[1].split_whitespace().collect();
    let mut order: HashSet<(i32, i32)> = HashSet::new();
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in &section1 {
        let data: Vec<&str> = line.split("|").collect();
        order.insert((
            data[0].parse::<i32>().expect("Expected a number"),
            data[1].parse::<i32>().expect("Expected a number"),
        ));
    }

    for line in &section2 {
        let data: Vec<&str> = line.split(",").collect();
        let mut update: Vec<i32> = vec![];
        for val in data {
            update.push(val.parse::<i32>().expect("Expected a number"));
        }
        updates.push(update);
    }

    println!("{:?}", find_good_updates(&order, &updates));
}

fn find_good_updates(order: &HashSet<(i32, i32)>, updates: &[Vec<i32>]) -> i32 {
    let mut res = 0;
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                let page1 = update[i];
                let page2 = update[j];

                if order.contains(&(page2, page1)) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }
        if valid {
            res += update[update.len() / 2]
        }
    }
    res
}
