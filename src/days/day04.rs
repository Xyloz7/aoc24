use tracing::debug;

use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, ops::Add};

///////////////////////////////////////////////////////////////////////////////

struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<char>,
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
    pub fn all_dirs() -> Vec<Directions> {
        vec![
            Self::N,
            Self::S,
            Self::E,
            Self::W,
            Self::NE,
            Self::NW,
            Self::SE,
            Self::SW,
        ]
    }
}

impl Grid {
    // (0,0) (1,0), (2,0)
    // (1,0) (1,1), (2,1)
    // (2,0) (1,2), (2,2)
    pub fn new(input_str: String) -> Self {
        let width = input_str.split("\n").next().unwrap().len();
        let height = input_str.len() / width;
        debug!("cols, rows {} {}", width, height);
        Self {
            cols: width,
            rows: height,
            data: input_str.chars().collect(),
        }
    }
    pub fn direction_to_linear_diff(&self, dir: &Directions) -> i32 {
        let c = self.cols + 1;
        match dir {
            Directions::N => -1 * c as i32,
            Directions::S => c as i32,
            Directions::E => 1,
            Directions::W => -1,
            Directions::NE => -1 * c as i32 + 1,
            Directions::NW => -1 * c as i32 - 1,
            Directions::SE => c as i32 + 1,
            Directions::SW => c as i32 - 1,
        }
    }
    pub fn coord_to_linear(&self, x: usize, y: usize) -> usize {
        // 1,1 should return 4, if width =3
        (y * self.cols) + x
    }
    pub fn linear_to_coord(&self, i: usize) -> (usize, usize) {
        // 1,1 should return 4, if width =3
        let x = i % self.cols;
        let y = (i - x) / self.cols;
        (x, y)
    }

    fn check_letter(
        &self,
        direction: &Directions,
        starting_linear_index: usize,
        letter: char,
    ) -> Option<i32> {
        let m_vec = self.direction_to_linear_diff(direction);
        let m_ind = (starting_linear_index as i32).add(m_vec);
        debug!("checking {} {} {}", starting_linear_index, m_vec, m_ind);
        if m_ind < 0 || m_ind >= self.data.len() as i32 {
            debug!("Out of bounds");
            return None;
        };

        let m_char = self.data[m_ind as usize];
        if m_char != letter {
            debug!("Wrong letter {} {}", m_char, letter);
            return None;
        };
        Some(m_ind)
    }
    pub fn check_xmas(&self, direction: &Directions, starting_linear_index: usize) -> bool {
        debug!("Checking {:?}", direction);
        let mut i = starting_linear_index;
        for c in vec!['M', 'A', 'S'] {
            let next_letter = self.check_letter(direction, i, c);
            // debug!("{} {:?}",c, next_letter);
            if let Some(n) = next_letter {
                i = n as usize;
                // debug!("{} {} {}", c, i, self.data[i])
            } else {
                return false;
            }
        }

        return true;
    }
    pub fn check_x_mas(&self, starting_linear_index: usize) -> bool {
        let northeast =
            starting_linear_index as i32 + self.direction_to_linear_diff(&Directions::NE);
        let northwest =
            starting_linear_index as i32 + self.direction_to_linear_diff(&Directions::NW);
        let southeast =
            starting_linear_index as i32 + self.direction_to_linear_diff(&Directions::SE);
        let southwest =
            starting_linear_index as i32 + self.direction_to_linear_diff(&Directions::SW);
        if northeast < 0 || northwest < 0 || southeast < 0 || southwest < 0 {
            debug!("out of bounds");
            return false;
        };
        if northeast >= self.data.len() as i32
            || northwest >= self.data.len() as i32
            || southeast >= self.data.len() as i32
            || southwest >= self.data.len() as i32
        {
            debug!("out of bounds");
            return false;
        }
        let letters: Vec<char> = vec![northeast, northwest, southeast, southwest]
            .iter()
            .map(|x| self.data[*x as usize])
            .collect();
        let ms = letters.iter().filter(|x| **x == 'M').count();
        let ss = letters.iter().filter(|x| **x == 'S').count();
        if ms != 2 || ss != 2 {
            return false;
        };
        if self.data[northeast as usize] == self.data[southwest as usize] {
            return false;
        };
        if self.data[northwest as usize] == self.data[southeast as usize] {
            return false;
        }
        return true;
    }
}

pub fn solve() -> SolutionPair {
    let file_path = "./input/day4.txt";
    // let file_path = "./input/day4_short.txt";
    let content = match read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => return (Solution::from(0), Solution::from(0)), // Graceful fallback for I/O errors
    };

    let grid = Grid::new(content);
    let mut total = 0;
    let mut total2 = 0;
    for (i, c) in grid.data.iter().enumerate() {
        if c == &'X' {
            debug!("Hit an X, seeing if XMAS!");
            for dir in Directions::all_dirs() {
                if grid.check_xmas(&dir, i) {
                    debug!(
                        "Found an XMAS in dir {:?} {:?}",
                        dir,
                        grid.linear_to_coord(i)
                    );
                    total += 1;
                }
            }
        }
        if c == &'A' {
            debug!("Hit an A, seeing if X-MAS!");

            if grid.check_x_mas(i) {
                debug!("Found an XMAS in dir {:?}", grid.linear_to_coord(i));
                total2 += 1;
            }
        }
    }

    let sol1: u64 = total;
    let sol2: u64 = total2;

    (Solution::from(sol1), Solution::from(sol2))
}
