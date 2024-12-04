use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////
fn multiply_str(inp: &str) -> i32 {
    let nums: Vec<i32> = inp[4..inp.len() - 1] // Strip "mul(" and ")"
        .split(',')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    nums.get(0).copied().unwrap_or(0) * nums.get(1).copied().unwrap_or(0)
}

pub fn solve() -> SolutionPair {
    let re = regex::Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let file_path = "./input/day3.txt";
    let content = match read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => return (Solution::from(0), Solution::from(0)), // Graceful fallback for I/O errors
    };

    let mut total = 0;
    let mut total2 = 0;
    let mut active = true;

    for mat in re.find_iter(&content) {
        let matched = mat.as_str();
        if matched.starts_with("mul(") {
            let mul_val = multiply_str(matched);
            total += mul_val;
            if active {
                total2 += mul_val;
            }
        } else if matched == "do()" {
            active = true;
        } else if matched == "don't()" {
            active = false;
        }
    }

    (Solution::from(total as u64), Solution::from(total2 as u64))
}
