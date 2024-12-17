use std::fs;

type DiskType = Vec<i32>;
pub fn main() {
    let content =
        fs::read_to_string("./src/day9/input.txt").expect("Should have been able to read the file");

    let content: Vec<char> = content.chars().collect();
    let mut disk: DiskType = vec![];
    for c in content.iter() {
        if c.is_ascii_digit() {
            disk.push((*c as u8 - b'0') as i32);
        }
    }
    println!("{:?}", disk_fragmenter(&disk));
}

/**
 * referred https://github.com/nitekat1124/advent-of-code-2024/blob/main/solutions/day09.py
 */
pub fn disk_fragmenter(content: &DiskType) -> i64 {
    let mut blocks: Vec<(usize, Vec<i32>, i32)> = Vec::new();
    let mut file_id = 0;
    let mut space_indices: Vec<usize> = Vec::new();

    for (i, &count) in content.iter().enumerate() {
        if i % 2 == 0 {
            if count > 0 {
                blocks.push((1, vec![file_id; count as usize], 0));
                file_id += 1;
            }
        } else if count > 0 {
            space_indices.push(blocks.len());
            blocks.push((0, Vec::new(), count));
        }
    }

    let mut curr_block_idx = blocks.len() - 1;
    while curr_block_idx > 0 {
        if blocks[curr_block_idx].0 == 0 {
            curr_block_idx -= 1;
            continue;
        }

        let curr_block_len = blocks[curr_block_idx].1.len();

        for &space_idx in space_indices.iter() {
            if space_idx >= curr_block_idx {
                break;
            }

            if blocks[space_idx].2 >= curr_block_len as i32 {
                let (left, right) = blocks.split_at_mut(curr_block_idx);
                left[space_idx].1.append(&mut right[0].1);
                left[space_idx].2 -= curr_block_len as i32;

                right[0].0 = 0;
                blocks[curr_block_idx].2 = curr_block_len as i32;

                break;
            }
        }

        curr_block_idx -= 1;
    }

    let mut checksum = 0i64;
    let mut pos = 0i64;

    for block in blocks {
        for file in block.1 {
            checksum += pos * file as i64;
            pos += 1;
        }
        if block.0 == 0 {
            pos += block.2 as i64;
        }
    }

    checksum
}
