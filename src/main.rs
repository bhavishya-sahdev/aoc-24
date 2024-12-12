pub mod day1;

use std::fs;

fn main() {
    let content =
        fs::read_to_string("./src/day1.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];

    for line in &content {
        let nums: Vec<&str> = line.split_whitespace().collect();
        nums1.push(nums[0].parse::<i32>().expect("not a number"));
        nums2.push(nums[1].parse::<i32>().expect("not a number"));
    }

    println!(
        "part 1: {}",
        day1::find_shortest_distance(&mut nums1, &mut nums2)
    );
    println!(
        "part 2: {}",
        day1::find_similarity_score(&mut nums1, &mut nums2)
    );
}
