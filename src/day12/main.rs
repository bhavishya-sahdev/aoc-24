use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs, i32,
};

type Grid = Vec<Vec<char>>;

pub fn main() {
    let content = fs::read_to_string("./src/day12/input.txt")
        .expect("Should have been able to read the file");

    let content = content.split("\n").collect::<Vec<&str>>();
    let mut grid: Grid = vec![];
    for line in content {
        grid.push(line.chars().collect());
    }

    println!("{}", count_perimeter(&grid))
}

fn count_perimeter(grid: &Grid) -> i32 {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let height = grid.len();
    let width = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;

    let mut borders: HashMap<(i32, i32), HashMap<usize, BTreeSet<usize>>> = HashMap::new();

    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            let curr = grid[i][j];
            let mut area = 0;
            let mut perimeter = 0;
            queue.push_back((i, j));
            borders = HashMap::new();

            while let Some(value) = queue.pop_front() {
                if visited.contains(&value) {
                    continue;
                }

                visited.insert(value);
                area += 1;

                let mut neighbors = 0;

                let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

                for (x, y) in directions {
                    let new_i = value.0 as i32 + x;
                    let new_j = value.1 as i32 + y;
                    // range guard
                    if new_i < 0 || new_i >= height as i32 || new_j < 0 || new_j >= width as i32 {
                        add_border(&mut borders, value.0, value.1, x, y);
                        continue;
                    }

                    if grid[new_i as usize][new_j as usize] == curr {
                        neighbors += 1;
                        if !visited.contains(&(new_i as usize, new_j as usize)) {
                            queue.push_back((new_i as usize, new_j as usize));
                        }
                    } else {
                        add_border(&mut borders, value.0, value.1, x, y);
                    }
                }

                perimeter += 4 - neighbors;
            }

            let mut count = 0;
            for values in borders.values() {
                for set in values.values() {
                    let mut prev = -2;

                    for curr in set {
                        if prev != *curr as i32 - 1 {
                            count += 1;
                        }
                        prev = *curr as i32;
                    }
                }
            }

            sum += area * count;
        })
    });

    sum
}

fn add_border(
    borders: &mut HashMap<(i32, i32), HashMap<usize, BTreeSet<usize>>>,
    i: usize,
    j: usize,
    x: i32,
    y: i32,
) {
    let borders_entry = borders.entry((x, y)).or_default();
    let is_vertical = x != 0;
    let key = if is_vertical { i } else { j };
    let value = if is_vertical { j } else { i };
    let inner_set = borders_entry.entry(key).or_default();

    inner_set.insert(value);
}
