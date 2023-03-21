use rand::seq::SliceRandom;

pub struct SudokuGrid
{
    pub grid: Vec<Vec<u8>> // 9x9 grid
}




impl SudokuGrid
{
    fn check_row(&self, row:usize) -> bool {
        let mut rowt = vec![0;9];
        for i in 0..9 {
            let index= self.grid[row][i] as usize;
            if index!=0 {
                rowt[index-1] += 1;
                if rowt[index-1] > 1 {
                    return false;
                }
            } 
        }
        return true;

    }
    fn check_column(&self, col:usize) -> bool  {
        let mut colt = vec![0;9];
        for i in 0..9 {
            let index= self.grid[i][col] as usize;
            if index!=0 {
                colt[index-1] += 1;
                if colt[index-1] > 1 {
                    return false;
                }
            } 
        }
        return true;

    }
    fn check_square(&self, row:usize, col:usize) -> bool {
        let mut squaret=vec![0;9]; 
        let row_bound= (row/3)*3;
        let col_bound= (col/3)*3;
        for i in row_bound..row_bound+3 {
            for j in col_bound..col_bound+3 {
                let index= self.grid[i][j] as usize;
                if index!=0 {
                    squaret[index-1] += 1;
                    if squaret[index-1] > 1 {
                        return false;
                    }
                } 
            }
        }
        return  true;
    }
    pub fn show_sudoku(&self) {
        for i in 0..9 {
            if i%3 == 0 {
                println!("  - - -   - - -   - - - ");
            }
                for j in 0..9 {
                    if j%3 == 0  {
                        print!("| ");
                    }
                    print!("{} ", self.grid[i][j]);
            }       
            println!();
        }
        println!("")
    }


    pub fn generate_complete_grid(&mut self) {
        self.gcg(0,0);
    }


    fn gcg(&mut self, row:usize, col:usize) -> bool
    {
            if row==9 {
                return true;
            }
        let next_row= if col==8 {row+1} else {row};
        let next_col= if col==8 {0} else {col+1};

        let mut numbers: Vec<u8> = (1..=9).collect();
        let mut rng = rand::thread_rng();
        numbers.shuffle(&mut rng);

            for i in 0..9 {
                self.grid[row][col]=numbers[i];
                if self.check_row(row) && self.check_column(col) && self.check_square(row,col) {
                    if self.gcg(next_row, next_col)
                    {
                        return true;
                    } 
                }
            }
            self.grid[row][col]=0;
            return false;
    
            
    }


}
