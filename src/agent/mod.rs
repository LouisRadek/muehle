use std::time::{Duration, Instant};
use minimax::minimax;
use rayon::iter::{ParallelBridge, ParallelIterator};
use utils::{insert_number_of_possible_moves_to_board, insert_token_count_to_board};
use crate::logic::{action::{get_action_from_board, Action}, forward_boards::forward_step_boards, game_state::{Phase, Token}, position::negate_token};

pub mod minimax;

#[derive(Clone, Copy)]
pub struct AiPhase {
    pub phase: Phase,
    pub step_counter: u8
}

impl AiPhase {
    pub fn new(phase: Phase, step_counter: u8) -> Self {
        AiPhase {
            phase,
            step_counter
        }
    }
    pub fn increased(&self) -> Self {
        let mut new_phase = AiPhase::new(self.phase, self.step_counter + 1);
        if new_phase.phase == Phase::Set && new_phase.step_counter >= 18 {
            new_phase.phase = Phase::Move;
        }
        new_phase
    }
}

fn calculate_next_move(mut board: u64, player: Token, phase: Phase) -> u64 {
    let mut step_counter = 0;
    let ai_phase = AiPhase::new(phase, step_counter);
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);

    loop {
        let now = Instant::now();
        
        let mut depth = 0;
        
        let mut best_action_total = None;
        let mut best_score_total = match player {
            Token::White => isize::MIN,
            Token::Black => isize::MAX,
            Token::None => unreachable!()
        };
        let mut last_depth_time_elapsed = Duration::from_secs(0);
        let mut actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

        'outer_loop: loop {
            depth += 1;
            let mut best_action = None;
            let mut best_score = match player {
                Token::White => isize::MIN,
                Token::Black => isize::MAX,
                Token::None => unreachable!()
            };
            actions_with_scores = forward_step_boards(&board, Token::parse_to_u8(player), ai_phase)
                .par_bridge()
                .map(|forward_board| {
                    (forward_board, minimax(
                        forward_board, 
                        depth, 
                        isize::MIN, 
                        isize::MAX, 
                        negate_token(Token::parse_to_u8(player)), 
                        ai_phase.clone().increased(), 
                        now
                    ))
                }).collect();

            for (forward_board, score) in actions_with_scores.into_iter() {
                if score.is_none() {
                    break 'outer_loop;
                }

                best_action = Some(get_action_from_board(board, forward_board, Token::parse_to_u8(player)));
                if player == Token::White && score.unwrap() >= best_score {
                    best_score = score.unwrap();
                } else if player == Token::Black && score.unwrap() <= best_score {
                    best_score = score.unwrap();
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
