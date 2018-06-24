pub struct Board {
    board:[[Square;6];7],
    boardHeight:u32,
    boardWidth:u32
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
        Board{board:[[Square{team:None};6];7], boardHeight:8, boardWidth:9}
    }

    pub fn print_board(&self){
        for row in self.board.iter(){
            for square in row.iter(){
                let teamStr = match square.team {
                    Some(Team::TEAM1) => '1',
                    Some(Team::TEAM2) => '2',
                    None => '0',
                };
                print!("{}",teamStr);
            }
            println!("")
        }
    }
}