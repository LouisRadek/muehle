use std::io;
use muehle::{game_state::{GameState, Token}, mill_detection::is_beat_possible};
use regex::Regex;


pub fn execute_beat_token(game: &mut GameState) {
    let mut is_beat_succeded = false;
    while !is_beat_succeded {
        println!("Please Type in the piece of your oponent to be beaten");
    
        let player_input = match read_player_input() {
            Some(value) => value,
            None => continue,
        };
    
        let position = player_input.trim().parse().unwrap();
        if !is_beat_possible(game.get_positions(), position, game.get_player_turn()) {
            println!("Invalid position to beat a piece!");
            continue
        }

        game.beat_token(position);
        is_beat_succeded = true
    }
}

pub fn get_start_end_position(player_input: String) -> (Option<usize>, usize) {
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
    
    return (start_position, end_position)
}

pub fn read_player_input() -> Option<String> {
    let mut player_input = String::new();
    
    io::stdin()
        .read_line(&mut player_input)
        .expect("Failed to read line");
    let input_format = Regex::new(r"^(((0?[0-9]|1[0-9]|2[0-3]),(0?[0-9]|1[0-9]|2[0-3]))|(0?[0-9]|1[0-9]|2[0-3]))$").unwrap();
    if !input_format.is_match(&player_input.trim()) {
        println!("Invalid input! Please follow the syntax!");
        return None;
    }

    Some(player_input)
}
