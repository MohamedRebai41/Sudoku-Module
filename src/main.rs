
mod sudoku_grid;






fn main() {
    
    print!("Hello, world! \n");
    let mut sudo_grid=  sudoku_grid::SudokuGrid{
        grid: vec![vec![0;9];9]
    };
    sudo_grid.generate_complete_grid();
    sudo_grid.show_sudoku();
    
}
