extern crate connect_four;
use connect_four::board::Board;
use connect_four::board::Team;
use connect_four::game::Game;

fn main(){
    // let mut board = Board::new();
    // board.print_board();
    // board.insert_at_column(3, Team::TEAM1);
    // board.insert_at_column(3, Team::TEAM2);
    // board.insert_at_column(2, Team::TEAM1);
    // board.print_board();
    let mut game = Game::new();
    game.make_move(1);
    game.make_move(2);
    game.make_move(1);
    game.make_move(2);
    game.make_move(1);
    game.make_move(2);
    game.make_move(1);
    game.make_move(2);
}
