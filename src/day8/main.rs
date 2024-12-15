use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main() {
    let content =
        fs::read_to_string("./src/day8/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in content {
        let parsed_line: Vec<char> = line.chars().collect();
        grid.push(parsed_line);
    }

    println!("{:?}", find_pairs(&mut grid));
}

fn find_pairs(grid: &mut [Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            if grid[i][j].is_alphanumeric() {
                match antennas.get_mut(&grid[i][j]) {
                    Some(v) => {
                        v.push((i, j));
                    }
                    None => {
                        antennas.insert(grid[i][j], vec![(i, j)]);
                    }
                }
            }
        });
    });

    for values in antennas.values() {
        (0..values.len()).for_each(|i| {
            (i + 1..values.len()).for_each(|j| {
                let distance: (i32, i32) = (
                    values[j].0 as i32 - values[i].0 as i32,
                    values[j].1 as i32 - values[i].1 as i32,
                );

                for &(idx, dir) in &[(i, -1), (j, 1)] {
                    let mut pos = (values[idx].0 as i32, values[idx].1 as i32);
                    while pos.0 >= 0 && pos.0 < height as i32 && pos.1 >= 0 && pos.1 < width as i32
                    {
                        antinodes.insert((pos.0 as usize, pos.1 as usize));
                        pos.0 += distance.0 * dir;
                        pos.1 += distance.1 * dir;
                    }
                }
            });
        });
    }
    antinodes.len()
}
