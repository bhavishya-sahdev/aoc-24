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

pub fn disk_fragmenter(content: &DiskType) -> i64 {
    let mut content = content.clone();
    let mut disk: Vec<i32> = vec![];

    let mut c: i32 = 0;
    let mut i = 0;
    let mut y = if content.len() % 2 == 0 {
        content.len() - 2
    } else {
        content.len() - 1
    };

    while i <= y {
        if i % 2 == 0 {
            while content[i] > 0 {
                disk.push(c);
                content[i] -= 1;
            }
            c += 1;
        } else {
            while content[i] > 0 && y > i {
                if content[y] > 0 {
                    disk.push(y as i32 / 2);
                    content[y] -= 1;
                } else {
                    y -= 2;
                    continue;
                }
                content[i] -= 1;
            }
        }
        i += 1;
    }

    let mut sum: i64 = 0;
    (0..disk.len()).for_each(|i| {
        sum += (i as i64) * (disk[i] as i64);
    });

    sum
}
