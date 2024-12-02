use crate::{Solution, SolutionPair};
use std::{cmp, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn is_report_safe(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();
    let minval = *diffs.iter().min().unwrap_or(&0);
    let maxval = *diffs.iter().max().unwrap_or(&0);

    if maxval > 3 || minval.abs() > 3 {
        return false;
    }

    let all_positive = diffs.iter().all(|&x| x > 0);
    let all_negative = diffs.iter().all(|&x| x < 0);

    all_positive || all_negative
}

pub fn solve() -> SolutionPair {
    let file_path = "./input/day2.txt";
    let content = match read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => return (Solution::from(0), Solution::from(0)), // Graceful fallback for I/O errors.
    };

    let lines = content.lines();

    let (mut safe_reports, mut safe_damp) = (0, 0);

    for line in lines {
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<i32>().ok()) // Gracefully handle parsing errors.
            .collect();

        if is_report_safe(&report) {
            safe_reports += 1;
            safe_damp += 1;
            continue;
        }

        // Check all possible combinations with one element removed.
        for i in 0..report.len() {
            let mut combination = Vec::with_capacity(report.len() - 1);
            combination.extend(&report[..i]);
            combination.extend(&report[i + 1..]);

            if is_report_safe(&combination) {
                safe_damp += 1;
                break;
            }
        }
    }

    (Solution::from(safe_reports as u64), Solution::from(safe_damp as u64))
}
