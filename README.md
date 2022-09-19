# Soduku Solver in Rust
This is an implementation of the [backtracking](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking) soduku solver in rust
## Usage
- Run tests: `cargo test`
- Run solver on a particular board `cargo run --release <boards/name.csv>`
## Performance
- Worst case scenario for backtracking algorithim is under "anti_brute.csv"
- Run on Intel i7-9750H (12) @ 2.592GHz on kernel 5.10.102.1-microsoft-standard-WSL2
- Hyperfine benchmark:
    - Time (mean ± σ):     11.816 s ±  0.055 s    [User: 11.816 s, System: 0.000 s]
    - Range (min … max):   11.775 s … 11.943 s    10 runs