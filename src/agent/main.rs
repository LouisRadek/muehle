use muehle::game_state::Token;
use crate::enumerate::decode_positions;
use crate::generate_actions::generate_actions;
use std::time::{Duration, Instant};
use crate::minimax::minimax;
use crate::utils::{apply_action, get_number_of_token};

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
    let alpha = isize::MIN;
    let beta = isize::MAX;
    
    loop {
        let mut depth = 0;
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
        let mut best_action_total = None;
        let mut best_score_total = if token_type == Token::White { isize::MIN } else { isize::MAX };
        let mut last_depth_time_elapsed = Duration::from_secs(0);
        
                
        'outer_loop: loop {
            depth += 1;
            let mut best_action = None;
            let mut best_score = if token_type == Token::White { isize::MIN } else { isize::MAX };
            let actions = generate_actions(&positions, token_type, phase);
            for possible_action in actions {
                let new_positions = apply_action(
                    positions,
                    possible_action.start_position,
                    possible_action.end_position,
                    possible_action.beatable_position,
                    token_type
                );
                let action_score = match minimax(new_positions, depth, alpha, beta, token_type, phase, now) {
                    Some(score) => score,
                    None => {
                        break 'outer_loop;
                    }
                };

                if token_type == Token::White && action_score > best_score {
                    best_action = Some(possible_action);
                    best_score = action_score;
                } else if token_type == Token::Black && action_score < best_score {
                    best_action = Some(possible_action);
                    best_score = action_score;
                }
            }
            best_action_total = best_action;
            best_score_total = best_score;
            last_depth_time_elapsed = now.elapsed();
        }
        
        println!("{}", best_action_total.unwrap().to_string());

        let execution_information = format!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n", last_depth_time_elapsed, best_score_total, depth);
        eprintln!("{}", execution_information);
    }
}
