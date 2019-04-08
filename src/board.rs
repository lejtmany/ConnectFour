
const BOARD_HEIGHT:usize = 6;
const BOARD_WIDTH:usize = 7;

pub struct Board {
    board:[[Option<Team>;BOARD_WIDTH];BOARD_HEIGHT],
}

#[derive(Copy, Clone, PartialEq,Debug)]
pub enum Team{
    TEAM1, TEAM2
}

impl Board {
    pub fn new()->Board{
        Board{board:[[None;BOARD_WIDTH];BOARD_HEIGHT]}
    }

    pub fn print_board(&self){

        for column_num in 0..self.board[0].len() {
            print!("{}",column_num )
        }
        println!();

        for row in self.board.iter(){
            for column in row.iter(){
                let team_str = match column {
                    Some(Team::TEAM1) => '1',
                    Some(Team::TEAM2) => '2',
                    None => '0',
                };
                print!("{}",team_str);
            }
            println!();
        }
        println!();
    }

    pub fn insert_at_column(&mut self, column_num:usize, team:Team)->Result<(usize,usize),String>{
        if column_num > BOARD_WIDTH {
            return Err(format!("column number is outside of range. must be between 0 and {} but was {}", BOARD_WIDTH, column_num));
        }
        if self.board[0][column_num].is_some() {
            return Err(format!("column number {} is full", column_num))
        }
        for row_num in 0..=BOARD_HEIGHT{
            if row_num >= BOARD_HEIGHT || self.board[row_num][column_num].is_some(){
                let row_num_to_fill = row_num - 1;
                self.board[row_num_to_fill][column_num] = Some(team);
                return Ok((column_num, row_num_to_fill));
            }
        }
        Err("Shouldn't be here".to_string())
    }

    pub fn check_win(&self, col_idx:usize, row_idx:usize)->bool{
        self.check_col_for_win(col_idx) ||
        self.check_row_for_win(row_idx) ||
        self.check_diagonal_for_win(col_idx, row_idx)
    }

    fn check_row_for_win(&self, row_idx:usize)->bool{
        self.check_array_for_win(&self.board[row_idx][..])
    }   

    fn check_col_for_win(&self, col_idx:usize)->bool{
        let mut row_array : [Option<Team>;BOARD_HEIGHT] = [None;BOARD_HEIGHT];
        for row_idx in 0..BOARD_HEIGHT{
            row_array[row_idx] = self.board[row_idx][col_idx];
        }
        self.check_array_for_win(&row_array[..])
    }

    fn check_diagonal_for_win(&self, col_idx:usize, row_idx:usize)->bool{
        false
    }

    fn check_array_for_win(&self, board_arr:&[Option<Team>])->bool{
        let mut consec_ctr = 1;
        let mut last_cell = board_arr[0];
        for idx in 1..board_arr.len(){
            let curr_cell = board_arr[idx];
            if curr_cell != None && last_cell == curr_cell{
                consec_ctr = consec_ctr + 1;
                if consec_ctr >= 4 {
                    return true;
                }
            }
            else{
                consec_ctr = 1;
            }
            last_cell = curr_cell;
        }
        false
    }
}