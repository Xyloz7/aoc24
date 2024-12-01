use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let file_path = "./input/day1.txt";
    // let file_path = "./input/day1_short.txt";

    let content = read_to_string(file_path).unwrap();
    let lines = content.split("\n");
    let mut lhs = vec![];
    let mut rhs = vec![];
    for l in lines {
        let mut ii = l.split_ascii_whitespace();
        let a = ii.next().unwrap().parse::<i32>().unwrap();
        let b = ii.next().unwrap().parse::<i32>().unwrap();
        lhs.push(a);
        rhs.push(b);
    }

    lhs.sort();
    rhs.sort();
    let mut d = 0;
    let mut simscore =0;
    for (ctr, lhv) in lhs.iter().enumerate() {
        let rhv = rhs[ctr];
        d += (rhv - lhv).abs();
        let count = rhs.iter().filter(|x| x == &lhv).count();
        simscore+=count as i32*lhv;
    };



    // Your solution here...
    let sol1: u64 = d as u64;
    let sol2: u64 = simscore as u64;

    (Solution::from(sol1), Solution::from(sol2))
}
