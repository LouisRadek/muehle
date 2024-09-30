use position::negate_token;
use utils::{get_action_from_board, insert_number_of_possible_moves_to_board, insert_token_count_to_board};

use crate::position::decode_positions;
use crate::action::forward_step_boards;
use std::time::{Duration, Instant};
use crate::minimax::minimax;
use rayon::prelude::*;

mod action;
mod minimax;
mod position;
mod utils;

#[derive(Clone, Copy, PartialEq)]
pub enum PhaseEnum {
    Set,
    Move
}
#[derive(Clone, Copy)]
pub struct Phase {
    pub phase: PhaseEnum,
    pub step_counter: u8
}
impl Phase {
    pub fn new(phase: PhaseEnum, step_counter: u8) -> Self {
        Phase {
            phase,
            step_counter
        }
    }
    pub fn increased(&self) -> Self {
        let mut new_phase = Phase::new(self.phase, self.step_counter + 1);
        if new_phase.phase == PhaseEnum::Set && new_phase.step_counter >= 18 {
            new_phase.phase = PhaseEnum::Move;
        }
        new_phase
    }
}

fn read_input(step_counter: u8) -> (Phase, u8, u64) {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut input = input.trim().split(" ");
    
    let phase = match input.next().unwrap() {
        "P" => Phase::new(PhaseEnum::Set, step_counter),
        "M" => Phase::new(PhaseEnum::Move, step_counter),
        other => panic!("Unknown phase \"{}\"", other)
    };
    
    let token_type = match input.next().unwrap() {
        "B" => 0b10,
        "W" => 0b11,
        _ => panic!("Unknown color")
    };
    let mut board = decode_positions(input.next().unwrap().parse().unwrap());
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);

    (phase, token_type, board)
}

#[allow(unused_assignments)]
fn main() {
    let mut step_counter = 0;
    loop {
        let (phase, token_type, board) = read_input(step_counter);
        let now = Instant::now();
        
        let mut depth = 0;
        
        let mut best_action_total = None;
        let mut best_score_total = if token_type == 0b11 { isize::MIN } else { isize::MAX };
        let mut last_depth_time_elapsed = Duration::from_secs(0);
        let mut actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

        'outer_loop: loop {
            depth += 1;
            let mut best_action = None;
            let mut best_score = if token_type == 0b11 { isize::MIN } else { isize::MAX };
            actions_with_scores = forward_step_boards(&board, token_type, phase).par_bridge().map(|forward_board| {
                (forward_board, minimax(forward_board, depth, isize::MIN, isize::MAX, negate_token(token_type), phase.clone().increased(), now))
            }).collect();

            for action_with_score in actions_with_scores.into_iter() {
                if action_with_score.1.is_none() {
                    break 'outer_loop;
                }

                if token_type == 0b11 && action_with_score.1.unwrap() >= best_score {
                    best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
                    best_score = action_with_score.1.unwrap();
                } else if token_type == 0b10 && action_with_score.1.unwrap() <= best_score {
                    best_action = Some(get_action_from_board(board, action_with_score.0, token_type));
                    best_score = action_with_score.1.unwrap();
                }
            }

            best_action_total = best_action;
            best_score_total = best_score;
            last_depth_time_elapsed = now.elapsed();
        }

        println!("{}", best_action_total.unwrap().to_string());

        let execution_information = format!("-> Execution time {:.3?} \n-> best score {} \n-> depth: {}\n-> step: {}\n", last_depth_time_elapsed, best_score_total, depth, step_counter);
        eprintln!("{}", execution_information);
        step_counter += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase() {
        let phase = Phase::new(PhaseEnum::Set, 0);
        println!("p:{}, s:{}", if phase.phase == PhaseEnum::Set { " Set" } else { " Move" }, phase.step_counter);
        println!("p:{}, s:{}", if phase.increased().phase == PhaseEnum::Set { " Set" } else { " Move" }, phase.increased().step_counter);
    }
}