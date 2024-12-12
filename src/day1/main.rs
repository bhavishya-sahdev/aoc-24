use std::{collections::HashMap, fs, iter::zip};

pub fn main() {
    let content =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];

    for line in &content {
        let nums: Vec<&str> = line.split_whitespace().collect();
        nums1.push(nums[0].parse::<i32>().expect("not a number"));
        nums2.push(nums[1].parse::<i32>().expect("not a number"));
    }

    println!("part 1: {}", find_shortest_distance(&mut nums1, &mut nums2));
    println!("part 2: {}", find_similarity_score(&mut nums1, &mut nums2));
}

pub fn find_shortest_distance(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> i32 {
    nums1.sort();
    nums2.sort();

    let mut res = 0;
    for (a, b) in zip(nums1, nums2) {
        res += (*a - *b).abs();
    }

    res
}

pub fn find_similarity_score(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> i32 {
    let mut nums2_map: HashMap<i32, i32> = HashMap::new();
    for i in nums2 {
        nums2_map.entry(*i).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut res = 0;
    for j in nums1 {
        res += match nums2_map.get(j) {
            Some(v) => *j * v,
            None => 0,
        }
    }

    res
}
