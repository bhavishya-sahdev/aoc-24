use std::fs;

pub fn main() {
    let content =
        fs::read_to_string("./src/day4/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in content {
        let parsed_line: Vec<char> = line.chars().collect();
        matrix.push(parsed_line);
    }

    println!("{:?}", find_x_mas_count(&matrix));
}

fn find_word_count(word: &str, hay: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let word_len = word.len();
    let height = hay.len();
    let width = hay[0].len();

    for i in 0..height {
        for j in 0..width {
            if hay[i][j] == word.chars().next().unwrap() {
                if j + word_len <= width {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i][j + it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if j >= word_len - 1 {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i][j - it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i + word_len <= height {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i + it][j] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i >= word_len - 1 {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i - it][j] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i + word_len <= height && j + word_len <= width {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i + it][j + it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i >= word_len - 1 && j + word_len <= width {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i - it][j + it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i + word_len <= height && j >= word_len - 1 {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i + it][j - it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }

                if i >= word_len - 1 && j >= word_len - 1 {
                    let mut valid = true;
                    for it in 0..word_len {
                        if hay[i - it][j - it] != word.chars().nth(it).unwrap() {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn find_x_mas_count(hay: &[Vec<char>]) -> i32 {
    let mut count = 0;
    let height = hay.len();
    let width = hay.len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if hay[i][j] == 'A'
                && ((hay[i - 1][j - 1] == 'M' && hay[i + 1][j + 1] == 'S')
                    || (hay[i - 1][j - 1] == 'S' && hay[i + 1][j + 1] == 'M'))
                && ((hay[i - 1][j + 1] == 'M' && hay[i + 1][j - 1] == 'S')
                    || (hay[i - 1][j + 1] == 'S' && hay[i + 1][j - 1] == 'M'))
            {
                count += 1;
            }
        }
    }
    count
}
