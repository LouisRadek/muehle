use std::time::{Duration, Instant};

use muehle::game_state::Token;
use crate::{generate_actions::generate_actions, utils::{apply_action, get_winner}, Phase};

pub fn minimax(positions: [Token; 24], depth: usize, mut alpha: isize, mut beta: isize, mut maximizing_player: Token, phase: Phase, time: Instant) -> Option<isize> {
    if time.elapsed() > Duration::from_millis(980) {
        return None;
    } else if depth == 0 || get_winner(positions, phase) != Token::None {
        return Some(evaluate_action(positions, phase));
    }

    let actions = generate_actions(&positions, maximizing_player, phase);
    
    if maximizing_player == Token::White {
        let mut max_eval = isize::MIN;
        for action in actions {
            let new_positions = apply_action(
                positions, 
                action.start_position, 
                action.end_position, 
                action.beatable_position, 
                maximizing_player
            );

            let eval = minimax(new_positions, depth - 1, alpha, beta, maximizing_player.negate(), phase, time);
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
        for action in actions {
            let new_positions = apply_action(
                positions, 
                action.start_position, 
                action.end_position, 
                action.beatable_position, 
                maximizing_player
            );

            let eval = minimax(new_positions, depth - 1, alpha, beta, maximizing_player.negate(), phase, time);
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

fn evaluate_action(positions: [Token; 24], phase: Phase) -> isize {
    let mut score: isize = 0;

    // Anzahl der Steine
    score += get_number_of_token_difference(positions) * 1000;

    // Gewichtung hängt ab von der Gesamtanzahl an "farblosen" Mühlen.
    let (white_mills, white_2_of_3_mills, black_mills, black_2_of_3_mills, gray_mills) = get_every_mill_type(positions);
    
    score += (white_mills - black_mills) * (gray_mills + 1) * 100;
    score += (white_2_of_3_mills - black_2_of_3_mills) * (gray_mills + 1) * 20;

    let winning_player = get_winner(positions, phase);
    if winning_player == Token::White {
        return isize::MAX
    } else if winning_player == Token::Black {
        return isize::MIN
    }

    return score
}

#[test]
fn test_get_number_of_token() {
    let mut positions = [Token::None; 24];
    positions[23] = Token::Black;
    positions[11] = Token::Black;
    positions[19] = Token::Black;
    positions[4] = Token::White;
    positions[5] = Token::White;
    positions[7] = Token::White;
    positions[9] = Token::White;
    positions[15] = Token::White;
    positions[20] = Token::White;
    positions[21] = Token::White;
    positions[22] = Token::White;

    let actual = get_every_mill_type(positions);
    println!("{}-{}-{}-{}-{}", actual.0, actual.1, actual.2, actual.3, actual.4);
    println!("score: {}", evaluate_action(positions, Phase::Set));

    assert_eq!(1, actual.0);
    assert_eq!(2, actual.1);
    assert_eq!(0, actual.2);
    assert_eq!(1, actual.3);
    assert_eq!(1, actual.4);
    assert_eq!(5240, evaluate_action(positions, Phase::Set));
}

fn get_number_of_token_difference(positions: [Token; 24]) -> isize {
    let mut difference: isize = 0;

    for token in positions {
        if token == Token::White {
            difference += 1;
        } else if token == Token::Black {
            difference -= 1;
        }
    }

    return difference
}

fn get_every_mill_type(positions: [Token; 24]) -> (isize, isize, isize, isize, isize) {
    let mut white_mills = 0;
    let mut white_2_of_3_mills = 0;
    let mut black_mills = 0;
    let mut black_2_of_3_mills = 0;
    let mut gray_mills = 0;

    let mut index: isize = 24;
    while index > -8 {
        let mut white_tokens = 0;
        let mut black_tokens = 0;

        let combinations = if index > 0 { // 012, 234, 456, 670, 8910, ...
            [index-2, index-1, if index%8 == 0 { index - 8 } else { index } ]
        } else { // crossing ring, i.e. 1-9-17, 3-11-19, 5-13-21, 7-15-23
            [(-index) + 1, (-index) + 9, (-index) + 17]
        };

        for &mod_index in combinations.iter() {

            match positions[mod_index as usize] {
                Token::White => white_tokens += 1,
                Token::Black => black_tokens += 1,
                Token::None => { }
            }
        }
        if white_tokens == 3 {
            white_mills += 1;
        } else if white_tokens == 2 && black_tokens == 0 {
            white_2_of_3_mills += 1;
        } else if white_tokens == 2 {
            gray_mills += 1;
        } else if black_tokens == 3 {
            black_mills += 1;
        } else if black_tokens == 2 && white_tokens == 0 {
            black_2_of_3_mills += 1;
        } else if black_tokens == 2 {
            gray_mills += 1;
        }

        index -= 2;
    }

    return (white_mills, white_2_of_3_mills, black_mills, black_2_of_3_mills, gray_mills)
}
