use tracing::{debug, info};

use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

struct Grid {
    cols: usize,
    rows: usize,
    data: Vec<char>,
    guard_pos: usize,
    guard_char: char,
    visited: HashMap<usize, HashSet<char>>,
}

#[derive(Debug)]
enum Directions {
    N,
    S,
    E,
    W,
}

fn rotate(c: char) -> Option<char> {
    match c {
        '^' => Some('>'),
        '>' => Some('v'),
        'v' => Some('<'),
        '<' => Some('^'),
        _ => {
            return None;
        }
    }
}
impl Grid {
    fn show(&self) -> String {
        let mut outstr = "".to_owned();
        let mut ctr = 0;
        for c in &self.data {
            ctr += 1;
            outstr.push(*c);
            if (ctr % self.cols) == 0 {
                outstr.push('\n');
            }
        }
        outstr
    }
    pub fn new(input: &str) -> Self {
        let cols = input.lines().next().unwrap_or("").len();
        let rows = input.lines().count();
        let data: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
        let guard_pos = data
            .iter()
            .position(|c| vec!['^', 'v', '>', '<'].contains(&c))
            .unwrap();
        let guard_char = data[guard_pos];

        let g = Self {
            cols,
            rows,
            data,
            guard_pos,
            guard_char,
            visited: HashMap::new(),
        };

        debug!(
            "Grid dimensions: cols={}, rows={}, guard at {} {:?}",
            cols,
            rows,
            guard_pos,
            g.linear_to_coord(guard_pos)
        );

        g
    }
    pub fn move_guard(&mut self) -> Option<usize> {
        if self
            .visited
            .entry(self.guard_pos)
            .or_default()
            .contains(&self.guard_char)
        {
            debug!(
                "We've been here before... {} {:?}",
                self.guard_char,
                self.visited.entry(self.guard_pos).or_default()
            );
            return Some(2);
        }
        self.data[self.guard_pos] = self.guard_char;
        self.visited
            .entry(self.guard_pos)
            .or_default()
            .insert(self.guard_char);
        // Move guard according to rules
        // debug!(
        //     "Pos {:?} ({}) {}",
        //     self.linear_to_coord(self.guard_pos),
        //     self.guard_pos,
        //     self.guard_char
        // );
        let dir = match self.guard_char {
            '^' => Directions::N,
            'v' => Directions::S,
            '>' => Directions::E,
            '<' => Directions::W,
            _ => {
                panic!("! no valid direction indicator found");
            }
        };
        let a = self.linear_to_coord(self.guard_pos);
        let b = self.direction_offset_vec(&dir);
        let (x, y) = (a.0 as i32 + b.0, a.1 as i32 + b.1);

        let next_sq_ind = self.guard_pos as i32 + self.direction_offset(&dir);

        if !self.is_in_bounds_vec(x, y) {
            debug!("Out of bounds!");
            return None;
        }
        let next_sq = self.data[next_sq_ind as usize];
        match next_sq {
            '.' | '>' | '<' | '^' | 'v' => {
                // Move in direction
                self.guard_pos = next_sq_ind as usize;
                // Increase count of squares
            }
            '#' | 'O' => {
                // Just spin right
                let new_g = rotate(self.guard_char).expect("unrotatable char");
                self.guard_char = new_g;
            }
            _ => {
                panic!("! invalid char found!");
            }
        }
        Some(1)
    }

    pub fn direction_offset(&self, direction: &Directions) -> i32 {
        let c = self.cols as i32;
        match direction {
            Directions::N => -c,
            Directions::S => c,
            Directions::E => 1,
            Directions::W => -1,
        }
    }
    pub fn direction_offset_vec(&self, direction: &Directions) -> (i32, i32) {
        match direction {
            Directions::N => (0, -1),
            Directions::S => (0, 1),
            Directions::E => (1, 0),
            Directions::W => (-1, 0),
        }
    }

    fn is_in_bounds_vec(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.cols as i32 && y >= 0 && y < self.rows as i32
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
}

pub fn solve() -> SolutionPair {
    // While in bounds, move
    let content = read_to_string("./input/day6.txt").unwrap_or_default();
    let mut grid = Grid::new(&content);
    loop {
        let n = grid.move_guard();
        if n.is_none() {
            break;
        }
    }
    let p1 = grid.visited.keys().count() as u64;
    let mut total = 0;
    debug!("PART 2");
    let mut ctr = 0;
    for p in grid.visited.keys() {
        ctr += 1;
        let mut grid2 = Grid::new(&content);
        grid2.data[*p] = 'O';
        info!("Trying pos {}/{}", ctr, p1);

        loop {
            let n = grid2.move_guard();
            if n.is_some_and(|x| x == 2) {
                debug!(
                    "Found position creating loop! {:?} {}",
                    grid2.linear_to_coord(*p),
                    p
                );
                debug!("\n{}\n", grid2.show());
                total += 1;
                break;
            };
            if n.is_none() {
                break;
            };
        }
    }

    let sol1: u64 = grid.visited.keys().count() as u64;
    let sol2: u64 = total;

    (Solution::from(sol1), Solution::from(sol2))
}
