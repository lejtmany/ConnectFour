const BOARD_HEIGHT:usize = 6;
const BOARD_WIDTH:usize = 7;

pub struct Board {
    board:[[Option<Team>;BOARD_WIDTH];BOARD_HEIGHT],
}

#[derive(Copy, Clone)]
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
            println!("")
        }
    }

    pub fn insert_at_column(&mut self, column_num:usize, team:Team){
        if column_num > BOARD_WIDTH {
            panic!("column number is outside of range. must be between 0 and {} but was {}", BOARD_WIDTH, column_num);
        }
        for row_num in 0..BOARD_HEIGHT + 1{
            if row_num >= BOARD_HEIGHT || self.board[row_num][column_num].is_some(){
                self.board[row_num - 1][column_num] = Some(team);
                break;
            }
        }
    }
}