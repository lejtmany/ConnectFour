extern crate connect_four;
use connect_four::board::Board;
use connect_four::board::Team;
use connect_four::game::Game;
use std::io;
use std::fmt;

fn main(){
    let mut game = Game::new();
    while(!game.get_is_over()){
        let mut input = String::new();
        game.print_board();
        println!("Player {:?}: Enter column for move", game.get_curr_team());
        io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");  
        game.make_move(input.trim().parse::<usize>().unwrap());
    }
    println!("Game Over! Final Board:");
    game.print_board();
}
