use std::{collections::HashMap, iter::zip};

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
