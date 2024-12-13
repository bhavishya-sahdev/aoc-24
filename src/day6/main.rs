use std::{collections::HashSet, fs};

pub fn main() {
    let content =
        fs::read_to_string("./src/day6/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in content {
        let parsed_line: Vec<char> = line.chars().collect();
        grid.push(parsed_line);
    }

    println!("{:?}", find_path(&grid));
}

fn rotate_90deg(dir: &(i32, i32)) -> (i32, i32) {
    if dir.0 == 0 && dir.1 == -1 {
        return (1, 0);
    } else if dir.0 == 1 && dir.1 == 0 {
        return (0, 1);
    } else if dir.0 == 0 && dir.1 == 1 {
        return (-1, 0);
    }
    (0, -1)
}

fn find_path(grid: &[Vec<char>]) -> usize {
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let height = grid.len();
    let width = grid[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut dir: (i32, i32) = (0, -1);

    for (i, _) in grid.iter().enumerate().take(height) {
        for j in 0..width {
            if grid[i][j] == '^' {
                pos_y = i as i32;
                pos_x = j as i32;

                break;
            }
        }
    }

    visited.insert((pos_x as usize, pos_y as usize));

    loop {
        let curr_x = pos_x + dir.0;
        let curr_y = pos_y + dir.1;

        if curr_x < 0 || (curr_x as usize) >= width || curr_y < 0 || (curr_y as usize) >= height {
            break;
        } else if grid[(curr_y) as usize][(curr_x) as usize] == '#' {
            dir = rotate_90deg(&dir);
            continue;
        }

        visited.insert((curr_x as usize, curr_y as usize));

        pos_x += dir.0;
        pos_y += dir.1;
    }

    visited.len()
}
