use rand::seq::SliceRandom;

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



    pub fn generate_complete_grid(grid: &mut Vec<Vec<u8>>) {
        gcg(grid,0,0);
    }



    pub fn gcg(grid: &mut Vec<Vec<u8>>, row:usize, col:usize) -> bool {
            if row==9 {
                return true;
            }
        let next_row= if col==8 {row+1} else {row};
        let next_col= if col==8 {0} else {col+1};

        let mut numbers: Vec<u8> = (1..=9).collect();
        let mut rng = rand::thread_rng();
        numbers.shuffle(&mut rng);

            for i in 0..9 {
                
                if check_row(grid,numbers[i],row) && check_column(grid,numbers[i],col) && check_square(grid, numbers[i],row,col) {
                    grid[row][col]=numbers[i];
                    if gcg(grid,next_row, next_col)
                    {
                        return true;
                    } 
                }
            }
            grid[row][col]=0;
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

