use std::{collections::HashSet, fs};

type EquationType = (i64, Vec<i64>);

pub fn main() {
    let content =
        fs::read_to_string("./src/day7/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut equations: Vec<EquationType> = vec![];

    for line in &content {
        let eq_str: Vec<&str> = line.split(": ").collect();
        let test_nums_str: Vec<&str> = eq_str[1].split_whitespace().collect();
        let test_val = eq_str[0].parse::<i64>().expect("not a number");
        let mut test_num_val: Vec<i64> = vec![];
        for num in test_nums_str {
            test_num_val.push(num.parse::<i64>().expect("not a number"));
        }
        equations.push((
            eq_str[0].parse::<i64>().expect("not a number"),
            test_num_val,
        ));
    }

    println!("{}", eval_expressions(&mut equations));
}

fn solve(equation: &mut EquationType, solved: &mut bool, curr: i64, index: usize) {
    if *solved {
        return;
    }
    if index >= equation.1.len() {
        if curr == equation.0 {
            *solved = true;
            return;
        } else {
            return;
        }
    }

    let val = (curr.to_string() + &equation.1[index].to_string())
        .parse::<i64>()
        .expect("not a number");
    solve(equation, solved, curr + equation.1[index], index + 1);
    solve(equation, solved, curr * equation.1[index], index + 1);
    solve(equation, solved, val, index + 1);
}

fn eval_expressions(equations: &mut Vec<EquationType>) -> i64 {
    let mut sum = 0;

    for equation in equations {
        let mut solved = false;
        let res = equation.0;
        solve(equation, &mut solved, 0, 0);

        if solved {
            sum += res;
        }
    }

    sum
}
