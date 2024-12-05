use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, ops::Add};
use tracing::debug;

///////////////////////////////////////////////////////////////////////////////

struct Grid {
    cols: usize,
    rows: usize,
    data: Vec<char>,
}

#[derive(Debug)]
enum Directions {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Directions {
    pub fn all() -> &'static [Directions] {
        &[
            Directions::N,
            Directions::S,
            Directions::E,
            Directions::W,
            Directions::NE,
            Directions::NW,
            Directions::SE,
            Directions::SW,
        ]
    }
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap_or("").len();
        let rows = input.lines().count();
        let data = input.chars().filter(|&c| c != '\n').collect();
        debug!("Grid dimensions: cols={}, rows={}", cols, rows);
        Self { cols, rows, data }
    }

    pub fn direction_offset(&self, direction: &Directions) -> i32 {
        let c = self.cols as i32;
        match direction {
            Directions::N => -c,
            Directions::S => c,
            Directions::E => 1,
            Directions::W => -1,
            Directions::NE => -c + 1,
            Directions::NW => -c - 1,
            Directions::SE => c + 1,
            Directions::SW => c - 1,
        }
    }

    fn is_in_bounds(&self, index: i32) -> bool {
        index >= 0 && index < self.data.len() as i32
    }

    fn check_letter(&self, direction: &Directions, index: usize, letter: char) -> Option<usize> {
        let offset = self.direction_offset(direction);
        let next_index = (index as i32).add(offset);
        if self.is_in_bounds(next_index) && self.data[next_index as usize] == letter {
            Some(next_index as usize)
        } else {
            None
        }
    }

    pub fn check_xmas(&self, direction: &Directions, start: usize) -> bool {
        let mut index = start;
        for &letter in &['M', 'A', 'S'] {
            if let Some(next_index) = self.check_letter(direction, index, letter) {
                index = next_index;
            } else {
                return false;
            }
        }
        true
    }

    pub fn check_x_mas(&self, start: usize) -> bool {
        let offsets = [
            self.direction_offset(&Directions::NE),
            self.direction_offset(&Directions::NW),
            self.direction_offset(&Directions::SE),
            self.direction_offset(&Directions::SW),
        ];

        let indices: Vec<_> = offsets
            .iter()
            .map(|&offset| (start as i32).add(offset))
            .collect();

        if indices.iter().any(|&index| !self.is_in_bounds(index)) {
            return false;
        }

        let letters: Vec<_> = indices.iter().map(|&i| self.data[i as usize]).collect();
        let m_count = letters.iter().filter(|&&c| c == 'M').count();
        let s_count = letters.iter().filter(|&&c| c == 'S').count();

        m_count == 2 && s_count == 2 && letters[0] != letters[3] && letters[1] != letters[2]
    }
}

pub fn solve() -> SolutionPair {
    let content = read_to_string("./input/day4.txt").unwrap_or_default();
    let grid = Grid::new(&content);

    let mut total_xmas = 0;
    let mut total_x_mas = 0;

    for (i, &c) in grid.data.iter().enumerate() {
        if c == 'X' {
            for direction in Directions::all() {
                if grid.check_xmas(direction, i) {
                    total_xmas += 1;
                }
            }
        } else if c == 'A' && grid.check_x_mas(i) {
            total_x_mas += 1;
        }
    }

    (Solution::from(total_xmas), Solution::from(total_x_mas))
}
