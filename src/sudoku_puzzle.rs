use rand::Rng;
use std::collections::HashMap;

mod sudoku_grid_module;
use self::sudoku_grid_module::{possible_values, generate_complete_grid, show_sudoku, check_row, check_column, check_square};





pub struct SudokuPuzzle
{
    pub grid:Vec<Vec<u8>>,// 9x9 grid
    pub solution:Vec<Vec<u8>>, //solution to the puzzle
    pub empty_cells: HashMap<(usize,usize),u8>, // (row,col) -> solution  
    pub difficulty:usize // number of holes in the puzzle
    
}



impl SudokuPuzzle
{


    
    pub fn new(difficulty:usize) -> Self
    {
        let mut grid=vec![vec![0;9];9];
        generate_complete_grid(&mut grid);
        let solution = grid.clone();
        
        let empty_cells = HashMap::new();
        SudokuPuzzle
        {
            grid,
            solution,
            empty_cells,
            difficulty
        }
    }


    pub fn generate_puzzle(&mut self) {
        self.empty_cells.clear();
        self.grid=self.solution.clone();
        self.dig_holes(self.difficulty);
    }

    fn dig_holes(&mut self, mut holes:usize) {
        let mut rng = rand::thread_rng();
        let total_holes=holes;
        let mut pairs:Vec<(usize,usize)>=Vec::new();
        for i in 0..9
        {
            for j in 0..9
            {
                pairs.push((i,j));
            }
        }
        while holes!=0
        {
            let a= 81-(total_holes-holes);
            let i=rng.gen_range(0..a);
            let (row,col)=pairs[i];
            if self.grid[row][col]!=0 && !self.check_multiple_solutions(row,col,self.grid[row][col]){
                self.empty_cells.insert((row,col),self.grid[row][col]);
                self.grid[row][col]=0;
                holes-=1;
                pairs.remove(i);
            }
        }
    }
    pub fn solve(grid:&mut Vec<Vec<u8>>,empty_cells: &mut Vec<(usize,usize)>) -> bool{
            if empty_cells.len()==0
            {
                return true;
            }
            let (row,col)=empty_cells[0];
            for pv in possible_values(grid,row,col)
            {
                grid[row][col]=pv;
                empty_cells.remove(0);
                if SudokuPuzzle::solve(grid,empty_cells)
                {
                    return true;
                }
                empty_cells.insert(0,(row,col));
                grid[row][col]=0;
            }
            return false;
            
            
    }

    fn check_multiple_solutions(&self,row:usize,col:usize,rm_value:u8) -> bool
    {
        let mut grid=self.grid.clone();
        grid[row][col]=0;
        
        let mut em_cells : Vec<(usize,usize)>;
        em_cells=self.empty_cells.keys().cloned().collect();
        for pv in possible_values(&grid,row,col)
        {
            if pv!=rm_value && check_row(&grid, pv, row) && check_column(&grid, pv, col) && check_square(&grid, pv, row, col)
            {
                
                grid[row][col]=pv;
                if SudokuPuzzle::solve(&mut grid,&mut em_cells)
                {
                    return true;
                }
            }
        }
        
        return false;
    }


    pub fn show_puzzle(&self) {
        show_sudoku(&self.grid);
    }
}
