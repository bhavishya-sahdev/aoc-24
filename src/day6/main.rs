use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main() {
    let content =
        fs::read_to_string("./src/day6/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut grid: Vec<Vec<char>> = vec![];

    for line in content {
        let parsed_line: Vec<char> = line.chars().collect();
        grid.push(parsed_line);
    }

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;
    for (i, _) in grid.iter().enumerate().take(grid.len()) {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                start_y = i;
                start_x = j;

                break;
            }
        }
    }
    println!("{:?}", find_loops(&mut grid, (start_x, start_y)));
}

type Direction = (i32, i32);
type Position = (usize, usize);

fn rotate_90deg(dir: &Direction) -> Direction {
    match dir {
        (0, -1) => (1, 0),  // left -> down
        (1, 0) => (0, 1),   // down -> right
        (0, 1) => (-1, 0),  // right -> up
        (-1, 0) => (0, -1), // up -> left
        _ => panic!("Invalid direction"),
    }
}

fn find_path(
    grid: &[Vec<char>],
    start: Position,
    initial_dir: Option<Direction>,
) -> (
    HashSet<Position>,
    HashMap<Position, (Position, Direction)>,
    bool,
) {
    let mut pos = (start.0 as i32, start.1 as i32);
    let height = grid.len();
    let width = grid[0].len();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut visited_entry: HashMap<Position, (Position, Direction)> = HashMap::new();
    let mut dir = initial_dir.unwrap_or((0, -1));

    visited.insert(start);

    loop {
        let curr = (pos.0 + dir.0, pos.1 + dir.1);

        if curr.0 < 0 || (curr.0 as usize) >= width || curr.1 < 0 || (curr.1 as usize) >= height {
            return (visited, visited_entry, true);
        }

        if grid[curr.1 as usize][curr.0 as usize] == '#' {
            dir = rotate_90deg(&dir);
            continue;
        }

        let curr_pos = (curr.0 as usize, curr.1 as usize);
        let prev_pos = (pos.0 as usize, pos.1 as usize);
        visited.insert(curr_pos);

        if let std::collections::hash_map::Entry::Vacant(e) = visited_entry.entry(curr_pos) {
            e.insert((prev_pos, dir));
        } else if visited_entry.get(&curr_pos) == Some(&(prev_pos, dir)) {
            return (visited, visited_entry, false);
        }

        pos = curr;
    }
}

/**
 * Inspired by https://github.com/nitekat1124/advent-of-code-2024/blob/main/solutions/day06.py
 */
fn find_loops(grid: &mut Vec<Vec<char>>, start: Position) -> i32 {
    let (mut visited, visited_entry, _) = find_path(grid, start, None);
    visited.remove(&start);
    let mut loop_count = 0;

    for pos in visited {
        let mut test_grid = grid.clone();
        test_grid[pos.1][pos.0] = '#';

        if let Some(&(prev_pos, prev_dir)) = visited_entry.get(&pos) {
            let (_, _, is_leaving) = find_path(&test_grid, prev_pos, Some(prev_dir));
            if !is_leaving {
                loop_count += 1;
            }
        }
    }

    loop_count
}
