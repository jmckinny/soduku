use wasm_bindgen::prelude::*;
mod board;
use board::Board;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn solve(numbers: &[u8]) -> Vec<u8> {
    let mut board = Board::new(numbers);
    if board.solve().is_ok() {
        board.get_nums()
    } else {
        numbers.to_vec()
    }
}