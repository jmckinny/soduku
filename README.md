# Soduku Solver in Rust
This is an implementation of the [backtracking](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking) soduku solver in rust
## Usage
- Run tests: `cargo test`
- Run solver on a particular board `cargo run --release <boards/name.csv>`
## Example Output
```
 4  9  5 | 6  0  2 | 0  0  1 
 8  2  1 | 0  4  0 | 6  0  9 
 7  6  3 | 1  0  9 | 0  2  0 
------------------------------
 5  3  8 | 4  2  1 | 9  6  7 
 2  4  7 | 9  0  0 | 1  0  3 
 6  1  9 | 0  0  0 | 2  4  0 
------------------------------
 3  5  2 | 7  9  4 | 8  1  6 
 9  0  0 | 0  1  0 | 0  0  2 
 1  0  0 | 2  0  3 | 0  9  0 

Ok(())
 4  9  5 | 6  7  2 | 3  8  1 
 8  2  1 | 3  4  5 | 6  7  9 
 7  6  3 | 1  8  9 | 4  2  5 
------------------------------
 5  3  8 | 4  2  1 | 9  6  7 
 2  4  7 | 9  6  8 | 1  5  3 
 6  1  9 | 5  3  7 | 2  4  8 
------------------------------
 3  5  2 | 7  9  4 | 8  1  6 
 9  7  4 | 8  1  6 | 5  3  2 
 1  8  6 | 2  5  3 | 7  9  4
 ```
## Performance
- Worst case scenario for backtracking algorithim is under "anti_brute.csv"
- Run on Intel i7-9750H (12) @ 2.592GHz on kernel 5.10.102.1-microsoft-standard-WSL2
- Hyperfine benchmark:
    - Time (mean ± σ):     11.816 s ±  0.055 s    [User: 11.816 s, System: 0.000 s]
    - Range (min … max):   11.775 s … 11.943 s    10 runs
