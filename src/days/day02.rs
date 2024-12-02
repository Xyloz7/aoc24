use crate::{Solution, SolutionPair};
use std::{cmp, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn is_report_safe(report: &[i32]) -> bool {
    let mut min_diff = i32::MAX;
    let mut max_diff = i32::MIN;
    let mut all_positive = true;
    let mut all_negative = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        min_diff = cmp::min(min_diff, diff);
        max_diff = cmp::max(max_diff, diff);
        all_positive &= diff > 0;
        all_negative &= diff < 0;

        // Early exit if unsafe
        if max_diff > 3 || min_diff < -3 || (!all_positive && !all_negative) {
            return false;
        }
    }

    true
}

pub fn solve() -> SolutionPair {
    let file_path = "./input/day2.txt";
    let content = match read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => return (Solution::from(0), Solution::from(0)), // Graceful fallback for I/O errors.
    };

    let lines = content.lines();

    let mut safe_reports = 0;
    let mut safe_damp = 0;

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
        let mut found_safe = false;
        for i in 0..report.len() {
            if i > 0 && report[i] == report[i - 1] {
                // Skip duplicates for performance
                continue;
            }

            // Check safety without creating a new vector
            let is_safe = is_report_safe(&[
                &report[..i],    // Elements before the removed one
                &report[i + 1..] // Elements after the removed one
            ]
            .concat());
            if is_safe {
                safe_damp += 1;
                found_safe = true;
                break;
            }
        }

        if !found_safe {
            continue;
        }
    }

    (Solution::from(safe_reports as u64), Solution::from(safe_damp as u64))
}
