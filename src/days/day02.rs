use crate::{Solution, SolutionPair};
use std::{cmp::min, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

fn is_report_safe(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|x| x[1] - x[0]).collect();

    // println!("{:?}", diffs);
    let minval = diffs.iter().min().unwrap();
    let maxval = diffs.iter().max().unwrap();
    if maxval > &3 || minval.abs() > 3 {
        return false;
        // prin
    };
    let same_sign = diffs.iter().all(|x| x < &0) || diffs.iter().all(|x| x > &0);
    if !same_sign {
        return false;
    }
    // println!("SAFE");
    return true;
}

pub fn solve() -> SolutionPair {
    let file_path = "./input/day2.txt";
    // let file_path = "./input/day2_short.txt";

    let content = read_to_string(file_path).unwrap();
    let lines = content.split("\n");

    let mut safe_reports = 0;
    let mut safe_damp = 0;
    for l in lines {
        let report: Vec<i32> = l
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if is_report_safe(&report) {
            safe_reports += 1;
            safe_damp += 1;
            continue;
        };
        for i in 0..report.len() {
            // Create a new vector with the ith element removed
            let mut combination = report.clone();
            combination.remove(i);

            // Pass the resulting vector to the function
            if is_report_safe(&combination) {
                safe_damp += 1;
                println!("{:?} safe by removing {:?}", report, report[i]);
                break;
            }
        }
    }

    let sol1: u64 = safe_reports;
    let sol2: u64 = safe_damp;

    (Solution::from(sol1), Solution::from(sol2))
}
