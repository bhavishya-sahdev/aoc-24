use std::fs;

pub fn main() {
    let content =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");

    let content: Vec<&str> = content.split('\n').collect();
    let mut reports: Vec<Vec<i32>> = vec![];

    for line in &content {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().expect("not a number"))
            .collect();
        reports.push(nums);
    }

    println!("{}", count_safe_reports(&reports));
}

fn is_report_safe(prev_diff: &Option<i32>, diff: &i32) -> bool {
    if !(1..=3).contains(&diff.abs()) {
        return false;
    }

    if let Some(prev_diff) = prev_diff {
        if (prev_diff.is_negative() && diff.is_positive())
            || (prev_diff.is_positive() && diff.is_negative())
        {
            return false;
        }
    }

    true
}

pub fn count_safe_reports(reports: &[Vec<i32>]) -> i32 {
    let mut selected = 0;
    let n = reports.len();
    let mut res = 0;
    while selected < n {
        let mut prev_diff: Option<i32> = None;
        let mut is_safe = true;
        let mut skip = 0;
        while skip < reports[selected].len() {
            let mut report = Vec::new();
            report.extend_from_slice(&reports[selected][0..skip]);
            report.extend_from_slice(&reports[selected][skip + 1..]);
            for i in 0..report.len() - 1 {
                let diff = report[i] - report[i + 1];

                is_safe = is_report_safe(&prev_diff, &diff);

                if !is_safe {
                    break;
                }
                prev_diff = Some(diff);
            }
            if is_safe {
                break;
            }
            skip += 1;
        }

        res += if is_safe { 1 } else { 0 };
        selected += 1;
    }

    res
}
