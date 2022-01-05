use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Board {
    data: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(data: &[u8]) -> Self {
        let mut board = Board {
            data: vec![vec![0; 9]; 9],
        };
        let mut cur_row = 0;
        let mut cur_col = 0;
        for (index, num) in data.iter().enumerate() {
            if index % 9 == 0 && index != 0 {
                cur_row += 1;
            }
            if cur_col == 9 {
                cur_col = 0;
            }
            board.data[cur_row][cur_col] = *num;
            cur_col += 1;
        }
        board
    }

    pub fn from_csv(filename: &str) -> Self {
        let reader = BufReader::new(File::open(filename).unwrap());
        let mut data = Vec::new();
        for line in reader.lines() {
            let unwrapped = line.unwrap();
            let nums: Vec<&str> = unwrapped.split(',').collect();
            for num in nums {
                data.push(num.parse::<u8>().unwrap());
            }
        }
        Board::new(&data)
    }

    fn valid_move(&self, row: usize, col: usize, value: u8) -> bool {
        //Check bounds
        if row >= self.data.len() || col >= self.data[0].len() {
            return false;
        }
        //Check if a number is already there
        if self.data[row][col] != 0 {
            return false;
        }
        //Check validity
        !self.in_row(row, value) && !self.in_col(col, value) && !self.in_grid(row, col, value)
    }

    fn in_row(&self, row: usize, value: u8) -> bool {
        for num in &self.data[row] {
            if *num == value {
                return true;
            }
        }
        false
    }

    fn in_col(&self, col: usize, value: u8) -> bool {
        for i in 0..self.data.len() {
            if self.data[i][col] == value {
                return true;
            }
        }
        false
    }

    fn in_grid(&self, row: usize, col: usize, value: u8) -> bool {
        let start_row = row - row % 3;
        let start_col = col - col % 3;
        for i in start_row..start_row + 3 {
            for j in start_col..start_col + 3 {
                if self.data[i][j] == value {
                    return true;
                }
            }
        }
        false
    }

    fn set_num(&mut self, row: usize, col: usize, value: u8) {
        self.data[row][col] = value;
    }

    pub fn solve(&mut self) -> Result<(), &str> {
        if self.is_solved() {
            return Ok(());
        }
        if let Some((row, col)) = self.next_empty() {
            for i in 1..=9 {
                if self.valid_move(row, col, i) {
                    self.set_num(row, col, i);
                    if let Ok(()) = self.solve() {
                        return Ok(());
                    }
                    self.set_num(row, col, 0);
                }
            }
        }
        Err("No solution found!")
    }

    fn is_solved(&self) -> bool {
        for row in &self.data {
            for num in row {
                if *num == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn next_empty(&self) -> Option<(usize, usize)> {
        for row in 0..self.data.len() {
            for col in 0..self.data[0].len() {
                if self.data[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }

    pub fn get_nums(&self) -> Vec<u8>{
        self.data.clone().into_iter().flatten().collect()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for num in row {
                write!(f, " {} ", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
