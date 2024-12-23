use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

const ROBOT: char = '@';
const WALL: char = '#';
const BOX: char = 'O';
const BLANK: char = '.';

pub fn main() {
    let content = fs::read_to_string("./src/day15/input.txt")
        .expect("Should have been able to read the file");

    let content = content.split("\n\n").collect::<Vec<&str>>();
    let mut grid: Vec<Vec<char>> = vec![];

    let mut moves = VecDeque::new();

    for line in content[0].split("\n").collect::<Vec<&str>>() {
        grid.push(line.chars().collect());
    }

    for m in content[1].chars() {
        if m != '\n' {
            moves.push_back(m);
        }
    }
    println!("{}", move_robot(&mut grid, &mut moves));
}

pub fn move_robot(grid: &mut Vec<Vec<char>>, moves: &mut VecDeque<char>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut robot = (0, 0);
    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            if grid[i][j] == ROBOT {
                robot = (i, j);
            }
        });
    });

    while let Some(m) = moves.pop_front() {
        if m == '<' {
            let mut x = robot.1;
            // let mut y = robot.0;
            while grid[robot.0][x] != WALL && grid[robot.0][x] != BLANK {
                x -= 1;
            }

            if grid[robot.0][x] == BLANK {
                while x != robot.1 {
                    grid[robot.0].swap(x, x + 1);
                    x += 1;
                }
                robot = (robot.0, x - 1);
            }
        } else if m == '>' {
            let mut x = robot.1;
            // let mut y = robot.0;
            while grid[robot.0][x] != WALL && grid[robot.0][x] != BLANK {
                x += 1;
            }

            println!("{x} {}", robot.1);
            if grid[robot.0][x] == BLANK {
                while x != robot.1 {
                    grid[robot.0].swap(x, x - 1);
                    x -= 1;
                }
                robot = (robot.0, x + 1);
            }
        } else if m == '^' {
            let mut y = robot.0;
            while grid[y][robot.1] != WALL && grid[y][robot.1] != BLANK {
                y -= 1;
            }

            if grid[y][robot.1] == BLANK {
                while y != robot.0 {
                    let temp = grid[y][robot.1];
                    grid[y][robot.1] = grid[y + 1][robot.1];
                    grid[y + 1][robot.1] = temp;
                    y += 1;
                }
                robot = (y - 1, robot.1);
            }
        } else if m == 'v' {
            let mut y = robot.0;
            while grid[y][robot.1] != WALL && grid[y][robot.1] != BLANK {
                y += 1;
            }

            if grid[y][robot.1] == BLANK {
                while y != robot.0 {
                    let temp = grid[y][robot.1];
                    grid[y][robot.1] = grid[y - 1][robot.1];
                    grid[y - 1][robot.1] = temp;
                    y -= 1;
                }

                robot = (y + 1, robot.1);
            }
        }
    }

    let mut total = 0;
    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            if grid[i][j] == BOX {
                total += ((100 * i) + j) as i32;
            }
        });
    });

    total
}
