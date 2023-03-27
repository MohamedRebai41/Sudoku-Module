



    pub fn check_row(grid: &Vec<Vec<u8>>,added:u8,row:usize) -> bool {
        for i in 0..9 {
            if grid[row][i]==added {
                return false;
            }
            } 
            return true;
        } 
    pub fn check_column(grid: &Vec<Vec<u8>>, added:u8, col:usize) -> bool  {
        for i in 0..9 {
            if grid[i][col]==added {
                return false;
            }
            } 
            return true;
     } 
    pub fn check_square(grid: &Vec<Vec<u8>>, added:u8,row:usize, col:usize) -> bool {
        let row_bound= (row/3)*3;
        let col_bound= (col/3)*3;
        for i in row_bound..row_bound+3 {
            for j in col_bound..col_bound+3 {
                if grid[i][j]==added {
                    return false;
                }
            }
        }
        return  true;
    }
    pub fn check_added(grid: &Vec<Vec<u8>>, added:u8,row:usize, col:usize) -> bool {
        if check_row(grid,added,row) && check_column(grid,added,col) && check_square(grid, added,row,col) {
            return true;
        }
        return false;
    }
    pub fn possible_values(grid: &Vec<Vec<u8>>,row:usize,col:usize) -> Vec<u8> {
        let mut possible_values:Vec<u8>=Vec::new();
        for i in 1..=9 {
            if check_row(grid,i,row) && check_column(grid,i,col) && check_square(grid,i,row,col) {
                possible_values.push(i);
            }
        }
        return possible_values;
    }
    pub fn show_sudoku(grid: &Vec<Vec<u8>>) {
        for i in 0..9 {
            if i%3 == 0 {
                println!("  - - -   - - -   - - - ");
            }
                for j in 0..9 {
                    if j%3 == 0  {
                        print!("| ");
                    }
                    print!("{} ", grid[i][j]);
            }       
            println!();
        }
        println!("")
    }







