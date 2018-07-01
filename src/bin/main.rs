extern crate connect_four;
use connect_four::board::Board;
use connect_four::board::Team;

fn main(){
    let mut board = Board::new();
    board.print_board();
    board.insert_at_column(3, Team::TEAM1);
    board.insert_at_column(3, Team::TEAM2);
    board.insert_at_column(2, Team::TEAM1);
    board.print_board();
}
