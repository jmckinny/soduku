mod board;
use board::Board;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board = Board::from_csv(&args[1]);
    println!("{}", board);
    println!("{:?}", board.solve());
    println!("{}", board);
}
