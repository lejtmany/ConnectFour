extern crate connect_four;
use connect_four::board::Board;

fn main(){
    let board = Board::new();
    board.print_board();
}
