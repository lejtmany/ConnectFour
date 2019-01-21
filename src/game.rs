use board::Team;
use board::Board;

pub struct Game {
    is_over:bool,
    curr_team:Team,
    board:Board
}

impl Game {
    pub fn make_move(&mut self, column_num:usize)->Result<(), String>{
        self.toggle_team();
        self.board.insert_at_column(column_num, self.curr_team)?;
        Ok(())
    }

    pub fn toggle_team(&mut self){
        match self.curr_team {
            Team::TEAM1 => self.curr_team = Team::TEAM2,
            Team::TEAM2 => self.curr_team = Team::TEAM1
        }
    }

    pub fn get_curr_team(&self)->Team{
        self.curr_team
    }

    pub fn get_is_over(&self)->bool{
        self.is_over
    }

    fn check_win(&self){
        
    }
}