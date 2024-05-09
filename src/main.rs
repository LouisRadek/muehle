mod game_state;
mod rendering;
mod r#move;
mod mill_detection;
mod agent;

use std::io::{self};

use game_state::GameState;
use regex::Regex;

use crate::{game_state::Token, mill_detection::is_beat_possible};

fn main() {
    let mut game = GameState::default();

    rendering::print_introduction_text();
    while !game.get_win() {
        rendering::print_board(game.get_positions());

        let phase = if game.get_token_set_at_beginning() > 0 {
            "set phase"
        } else {
            "move phase"
        };
        let player = if game.get_player_turn() == 1 {
            "Player 1"
        } else {
            "Player 2"
        };
        println!("{} has to do his move next ({}):", player, phase);

        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");

        let input_format = Regex::new(r"^(((0?[0-9]|1[0-9]|2[0-3]),(0?[0-9]|1[0-9]|2[0-3]))|(0?[0-9]|1[0-9]|2[0-3]))$").unwrap();
        if !input_format.is_match(&player_input.trim()) {
            println!("Invalid input! Please follow the syntax!");
            continue;
        }
        
        let start_end_position: Vec<&str> = player_input.trim().split(',').collect();
        let start_position;
        let end_position;
        if start_end_position.len() == 1 {
            start_position = None;
            end_position = start_end_position[0].parse().unwrap();
        } else {
            start_position = Some(start_end_position[0].parse().unwrap());
            end_position = start_end_position[1].parse().unwrap();
        }

        if !game.is_move_valid(start_position, end_position) {
            println!("Invalid move! Try again!");
            continue;
        }

        let is_mill_emerged = game.move_to(start_position, end_position);
        if is_mill_emerged {
            let mut is_beat_succeded = false;
            while !is_beat_succeded {
                println!("Please Type in the piece of your oponent to be beaten");
                
                let mut player_input = String::new();
                io::stdin()
                    .read_line(&mut player_input)
                    .expect("Failed to read line");
                
                let input_format = Regex::new(r"^(0?[0-9]|1[0-9]|2[0-3])$").unwrap();
                if !input_format.is_match(&player_input.trim()) {
                    println!("Invalid input! Please follow the syntax!");
                    continue
                }
    
                let position = player_input.trim().parse().unwrap();
                let token_of_player = if game.get_player_turn() == 1 {
                    Token::White
                } else {
                    Token::Black
                };
                if !is_beat_possible(game.get_positions(), position, token_of_player) {
                    println!("Invalid position to beat a piece!");
                    continue
                }

                game.beat_token(position);
                is_beat_succeded = true
            }
        }

        game.change_player()
    }
}
