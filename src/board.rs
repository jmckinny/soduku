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
#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_washington_post() {
        let mut board = Board::from_csv("boards/washington_post.csv");
        board.solve().unwrap();
        assert_eq!(
            board.data,
            vec![
                vec![6, 1, 2, 8, 4, 3, 9, 7, 5],
                vec![7, 8, 5, 9, 1, 6, 2, 4, 3],
                vec![4, 3, 9, 7, 5, 2, 6, 1, 8],
                vec![2, 5, 3, 1, 9, 7, 8, 6, 4],
                vec![1, 7, 8, 4, 6, 5, 3, 2, 9],
                vec![9, 6, 4, 2, 3, 8, 7, 5, 1],
                vec![8, 4, 6, 5, 7, 9, 1, 3, 2],
                vec![3, 2, 1, 6, 8, 4, 5, 9, 7],
                vec![5, 9, 7, 3, 2, 1, 4, 8, 6],
            ]
        );
    }
    #[test]
    fn test_board() {
        let mut board = Board::from_csv("boards/board.csv");
        board.solve().unwrap();
        assert_eq!(
            board.data,
            vec![
                vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
                vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
                vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
                vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
                vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
                vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
                vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
                vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
                vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
            ]
        );
    }
    #[test]
    fn test_samurai_section() {
        let mut board = Board::from_csv("boards/samurai.csv");
        board.solve().unwrap();
        assert_eq!(
            board.data,
            vec![
                vec![4, 9, 5, 6, 7, 2, 3, 8, 1],
                vec![8, 2, 1, 3, 4, 5, 6, 7, 9],
                vec![7, 6, 3, 1, 8, 9, 4, 2, 5],
                vec![5, 3, 8, 4, 2, 1, 9, 6, 7],
                vec![2, 4, 7, 9, 6, 8, 1, 5, 3],
                vec![6, 1, 9, 5, 3, 7, 2, 4, 8],
                vec![3, 5, 2, 7, 9, 4, 8, 1, 6],
                vec![9, 7, 4, 8, 1, 6, 5, 3, 2],
                vec![1, 8, 6, 2, 5, 3, 7, 9, 4],
            ]
        );
    }

    #[test]
    fn test_bad_board() {
        let mut board = Board::from_csv("boards/bad.csv");
        assert!(board.solve().is_err());
    }
}
