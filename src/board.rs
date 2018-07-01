const BOARD_HEIGHT:usize = 6;
const BOARD_WIDTH:usize = 7;

pub struct Board {
    board:[[Square;BOARD_WIDTH];BOARD_HEIGHT],
}

#[derive(Copy, Clone)]
struct Square {
    team: Option<Team>
}

#[derive(Copy, Clone)]
enum Team{
    TEAM1, TEAM2
}

impl Board {
    pub fn new()->Board{
        Board{board:[[Square{team:None};BOARD_WIDTH];BOARD_HEIGHT]}
    }

    pub fn print_board(&self){

        for column_num in 1..self.board[0].len() + 1 {
            print!("{}",column_num )
        }
        println!();
        println!();

        for row in self.board.iter(){
            for column in row.iter(){
                let team_str = match column.team {
                    Some(Team::TEAM1) => '1',
                    Some(Team::TEAM2) => '2',
                    None => '0',
                };
                print!("{}",team_str);
            }
            println!("")
        }
    }
}