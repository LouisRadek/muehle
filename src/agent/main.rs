use muehle::game_state::Token;
use crate::enumerate::decode_positions;
use crate::generate_actions::generate_actions;
use std::time::Instant;
use crate::minimax::minimax;
use crate::utils::apply_action;

mod enumerate;
mod utils;
mod test_list_moves;
mod generate_actions;
mod action;
mod minimax;

#[derive(Clone, Copy, PartialEq)]
pub enum Phase {
    Set,
    Move
}

fn main() {
    let depth = 3;
    let alpha = isize::MIN;
    let beta = isize::MAX;

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let now = Instant::now();

        let mut input = input.trim().split(" ");
        
        
        let phase = match input.next().unwrap() {
            "P" => Phase::Set,
            "M" => Phase::Move,
            _ => panic!("Unknown phase")
        };
        
        let token_type = match input.next().unwrap() {
            "B" => Token::Black,
            "W" => Token::White,
            _ => panic!("Unknown color")
        };
        

        let positions = decode_positions(input.next().unwrap().parse().unwrap());
        let actions = generate_actions(&positions, token_type, phase);
        let mut best_action = None;
        let mut best_score = if token_type == Token::White { isize::MIN } else { isize::MAX };

        for possible_action in actions {
            let new_positions = apply_action(
                positions.clone(), 
                possible_action.start_position, 
                possible_action.end_position, 
                possible_action.beatable_position, 
                token_type
            );
            let action_score = minimax(new_positions, depth, alpha, beta, token_type, phase);

            if token_type == Token::White && action_score > best_score {
                best_action = Some(possible_action);
                best_score = action_score;
            } else if token_type == Token::Black && action_score < best_score {
                best_action = Some(possible_action);
                best_score = action_score;
            }
        }
        
        println!("{}", best_action.unwrap().to_string());


        eprintln!("AI execution time was {:.3?} --- the best score found was {}", now.elapsed(), best_score);
    }
}
