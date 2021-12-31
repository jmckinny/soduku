mod board;
use board::Board;
fn main() {
    let mut board = Board::from_csv("boards/board.csv");
    println!("{}", board);
    println!("{:?}", board.solve());
    println!("{}", board);
}
