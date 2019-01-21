
const BOARD_HEIGHT:usize = 6;
const BOARD_WIDTH:usize = 7;

pub struct Board {
    board:[[Option<Team>;BOARD_WIDTH];BOARD_HEIGHT],
}

#[derive(Copy, Clone, PartialEq)]
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
        for row_num in 0..BOARD_HEIGHT + 1{
            if row_num >= BOARD_HEIGHT || self.board[row_num][column_num].is_some(){
                self.board[row_num - 1][column_num] = Some(team);
                return Ok((column_num, row_num));
            }
        }
        Err("Shouldn't be here".to_string())
    }

    pub fn check_win(&self)->bool{
        self.check_horizontal_win() ||
        self.check_vertical_win() 
        // check_diagonal_win(&self)
    }

    fn check_horizontal_win(&self)->bool{
        for row_num in 0..BOARD_HEIGHT{
            let mut consec_cntr = 1;
            let mut last_cell = self.board[row_num][0];
            for column_num in 1..BOARD_WIDTH{
                let curr_cell = self.board[row_num][column_num];
                if curr_cell != None && last_cell == curr_cell{
                    consec_cntr = consec_cntr + 1;
                }
                // dbg!(consec_cntr);
                last_cell = curr_cell;
                if consec_cntr >= 4 {
                    return true;
                }
            }
        }
        false
    }

    fn check_vertical_win(&self)->bool{
        for column_num in 0..BOARD_WIDTH{
            let mut consec_cntr = 1;
            let mut last_cell = self.board[0][column_num];
            for row_num in 1..BOARD_HEIGHT{
                let curr_cell = self.board[row_num][column_num];
                if curr_cell != None  && last_cell == curr_cell{
                    consec_cntr = consec_cntr + 1;
                }
                // dbg!(consec_cntr);
                last_cell = curr_cell;
                if consec_cntr >= 4 {
                    return true;
                }
            }
        }
        false
    }
}