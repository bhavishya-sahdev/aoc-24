use std::{collections::HashSet, fs};

type Grid = Vec<Vec<i32>>;
pub fn main() {
    let content = fs::read_to_string("./src/day10/input.txt")
        .expect("Should have been able to read the file");

    let content: Vec<&str> = content.split("\n").collect();
    let mut grid: Grid = vec![];
    content.into_iter().for_each(|line| {
        let digits: Vec<char> = line.chars().collect();
        grid.push(digits.iter().map(|c| (*c as u8 - b'0') as i32).collect());
    });
    path_finder(&grid);
}

fn dfs(pos: &(usize, usize), grid: &Grid, visited: &mut HashSet<(usize, usize)>) -> i32 {
    if grid[pos.0][pos.1] == 9 {
        return 1;
    }

    let mut sum = 0;
    let left = (pos.0, pos.1.saturating_sub(1));
    let up = (pos.0.saturating_sub(1), pos.1);
    let right = (pos.0, (grid[0].len() - 1).min(pos.1 + 1));
    let down = ((grid.len() - 1).min(pos.0 + 1), pos.1);

    if left.ne(pos) && !visited.contains(&left) && grid[left.0][left.1] == grid[pos.0][pos.1] + 1 {
        visited.insert(left);
        sum += dfs(&left, grid, visited);
    }
    if right.ne(pos)
        && !visited.contains(&right)
        && grid[right.0][right.1] == grid[pos.0][pos.1] + 1
    {
        visited.insert(right);
        sum += dfs(&right, grid, visited);
    }
    if up.ne(pos) && !visited.contains(&up) && grid[up.0][up.1] == grid[pos.0][pos.1] + 1 {
        visited.insert(up);
        sum += dfs(&up, grid, visited);
    }
    if down.ne(pos) && !visited.contains(&down) && grid[down.0][down.1] == grid[pos.0][pos.1] + 1 {
        visited.insert(down);
        sum += dfs(&down, grid, visited);
    }

    sum
}

pub fn path_finder(grid: &Grid) {
    let height = grid.len();
    let width = grid[0].len();
    let mut sum = 0;

    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            if grid[i][j] == 0
                && (i > 0 && grid[i - 1][j] == 1
                    || i + 1 < height && grid[i + 1][j] == 1
                    || j > 0 && grid[i][j - 1] == 1
                    || j + 1 < width && grid[i][j + 1] == 1)
            {
                let mut visited = HashSet::new();
                visited.insert((i, j));
                sum += dfs(&(i, j), grid, &mut visited);
            }
        })
    });

    println!("{sum:?}");
}
