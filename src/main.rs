mod game_state;
mod prints;
mod r#move;
mod mill_detection;
mod agent;
mod game_utils;

use game_state::GameState;
use game_utils::{execute_beat_token, get_start_end_position, read_player_input};

fn main() {
    let mut game = GameState::default();
    prints::print_introduction_text();
    
    while !game.get_win() {
        prints::print_board(game.get_positions());
        prints::print_move_instruction(&game);
        
        let player_input = match read_player_input() {
            Some(value) => value,
            None => continue,
        };
        let (start_position, end_position) = get_start_end_position(player_input);

        if !game.is_move_valid(start_position, end_position) {
            println!("Invalid move! Try again!");
            continue;
        }

        let is_mill_emerged = game.move_to(start_position, end_position);
        if is_mill_emerged {
            execute_beat_token(&mut game);
        }

        game.change_player()
    }
}
