use std::time::{Duration, Instant};

use itertools::Itertools;

use crate::action::forward_step_boards;
use crate::position::negate_token;
use crate::utils::{get_every_mill_type, get_number_of_tokens, get_winner};
use crate::Phase;

pub fn minimax(board: u64, depth: usize, mut alpha: isize, mut beta: isize, maximizing_player: u8, mut phase: Phase, time: Instant) -> Option<isize> {
    if time.elapsed() > Duration::from_millis(980) {
        return None;
    } else if get_winner(board, phase) == 0b11 {
        return Some(isize::MAX - phase.step_counter as isize)
    } else if get_winner(board, phase) == 0b10 {
        return Some(isize::MIN + phase.step_counter as isize)
    } else if depth == 0 {
        return Some(evaluate_action(board, phase));
    }

    let forward_step_boards = forward_step_boards(&board, maximizing_player, phase)
        .sorted_by(|board1, board2| {
            if maximizing_player == 0b11 {
                evaluate_action(*board2, phase).cmp(&evaluate_action(*board1, phase))
            } else {
                evaluate_action(*board1, phase).cmp(&evaluate_action(*board2, phase))
            }
    });
    
    if maximizing_player == 0b11 {
        let mut max_eval = isize::MIN;

        for forward_board in forward_step_boards {
            let eval = minimax(forward_board, depth - 1, alpha, beta, negate_token(maximizing_player), phase.increased(), time);
            if eval.is_none() {
                return None;
            }
            max_eval = std::cmp::max(max_eval, eval.unwrap());
            
            alpha = std::cmp::max(alpha, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        return Some(max_eval)
    } else {
        let mut min_eval = isize::MAX;
        for forward_board in forward_step_boards {
            let eval = minimax(forward_board, depth - 1, alpha, beta, negate_token(maximizing_player), phase.increased(), time);
            if eval.is_none() {
                return None;
            }
            min_eval = std::cmp::min(min_eval, eval.unwrap());
            
            beta = std::cmp::min(beta, eval.unwrap());
            if beta <= alpha {
                break;
            }
        }
        return Some(min_eval)
    }
}

fn evaluate_action(positions: u64, phase: Phase) -> isize {
    let mut score: isize = 0;

    // Anzahl der Steine
    score += (get_number_of_tokens(positions, 0b11) as isize - get_number_of_tokens(positions, 0b10) as isize) * 1000;

    // Gewichtung hängt ab von der Gesamtanzahl an "farblosen" Mühlen.
    let (white_mills, white_2_of_3_mills, black_mills, black_2_of_3_mills, gray_mills) = get_every_mill_type(positions);
    
    score += (white_mills - black_mills) * (gray_mills + 1) * 200;
    score += (white_2_of_3_mills - black_2_of_3_mills) * (gray_mills + 1) * 20;

    let winning_player = get_winner(positions, phase);
    if winning_player == 0b11 {
        return isize::MAX - phase.step_counter as isize
    } else if winning_player == 0b10 {
        return isize::MIN + phase.step_counter as isize
    }

    return score
}
