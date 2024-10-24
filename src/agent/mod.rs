use ggez::timer;
use minimax::minimax;
use rayon::iter::{ParallelBridge, ParallelIterator};
use crate::logic::{
    action::{get_action_from_board, Action}, 
    forward_boards::forward_step_boards, 
    game_state::{Phase, Token}, 
    move_token_count::{insert_number_of_possible_moves_to_board, insert_token_count_to_board}, 
    position::negate_token
};

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

#[allow(unused_assignments, unused_variables, clippy::if_same_then_else)]
pub fn calculate_next_move(mut board: u64, player: Token, ai_phase: AiPhase, max_depth: usize) -> Action {
    board = insert_token_count_to_board(board);
    board = insert_number_of_possible_moves_to_board(board);

    let now = timer::time();
    
    let player_parsed = Token::parse_to_u8(player);
    let mut depth = 0;
    let mut best_action_total = None;
    let mut best_score_total = match player {
        Token::White => isize::MIN,
        Token::Black => isize::MAX,
        Token::None => unreachable!()
    };
    let mut actions_with_scores: Vec<(u64, Option<isize>)> = Vec::with_capacity(500);

    'outer_loop: loop {
        if depth > max_depth {
            break 'outer_loop;
        }

        let mut best_action = None;
        let mut best_score = match player {
            Token::White => isize::MIN,
            Token::Black => isize::MAX,
            Token::None => unreachable!()
        };

        actions_with_scores = forward_step_boards(&board, player_parsed, ai_phase)
            .par_bridge()
            .map(|forward_board| {
                (forward_board, minimax(
                    forward_board, 
                    depth, 
                    isize::MIN, 
                    isize::MAX, 
                    negate_token(player_parsed), 
                    ai_phase.clone().increased(), 
                    now
                ))
            }).collect();
            
        for (forward_board, score) in actions_with_scores.into_iter() {
            if score.is_none() {
                break 'outer_loop;
            }

            if player == Token::White && score.unwrap() >= best_score {
                best_action = Some(get_action_from_board(board, forward_board, player_parsed));
                best_score = score.unwrap();
            } else if player == Token::Black && score.unwrap() <= best_score {
                best_action = Some(get_action_from_board(board, forward_board, player_parsed));
                best_score = score.unwrap();
            }
        }

        best_action_total = best_action;
        best_score_total = best_score;
        depth += 1;
    }

    best_action_total.unwrap()
}
