
mod sudoku_puzzle;
use sudoku_puzzle::SudokuPuzzle;

fn main() {
    let mut puzzle = SudokuPuzzle::new(30);
    puzzle.show_puzzle();
    puzzle.generate_puzzle();
    puzzle.show_puzzle();
    puzzle.generate_puzzle();
    puzzle.show_puzzle();

}

 