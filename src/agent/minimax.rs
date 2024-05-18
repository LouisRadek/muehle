use muehle::game_state::Token;
use crate::{generate_actions::{generate_actions, list_moves}, utils::{get_number_of_mills, apply_action, get_number_of_token, get_winner}, Phase};

pub fn minimax(positions: [Token; 24], depth: usize, mut alpha: isize, mut beta: isize, mut maximizing_player: Token, phase: Phase) -> isize {
    if depth == 0 || get_winner(positions, phase) != Token::None {
        return evaluate_action(positions, phase);
    }

    let actions = generate_actions(&positions, maximizing_player, phase);
    if maximizing_player == Token::White {
        let mut max_eval = isize::MIN;
        for action in actions {
            let new_positions = apply_action(
                positions.clone(), 
                action.start_position, 
                action.end_position, 
                action.beatable_position, 
                maximizing_player
            );

            let eval = minimax(new_positions, depth - 1, alpha, beta, maximizing_player.negate(), phase);
            max_eval = std::cmp::max(max_eval, eval);
            
            alpha = std::cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval
    } else {
        let mut min_eval = isize::MAX;
        for action in actions {
            let new_positions = apply_action(
                positions.clone(), 
                action.start_position, 
                action.end_position, 
                action.beatable_position, 
                maximizing_player
            );

            let eval = minimax(new_positions, depth - 1, alpha, beta, maximizing_player.negate(), phase);
            min_eval = std::cmp::min(min_eval, eval);
            
            beta = std::cmp::min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval
    }
}

fn evaluate_action(positions: [Token; 24], phase: Phase) -> isize {
    // maximizing_player
    // Anzahl Steine
    // Anzahl Mühlen (potentieller, blocked)
    // Anzahl möglicher Moves
    // win <=> loss
    
    // opponent
    // Anzahl Steine
    // Anzahl Muehlen (potentieller, blocked)
    // Anzahl moeglicher Moves

    let mut score: isize = 0;

    // Anzahl der Steine
    score += (get_number_of_token(positions, Token::White) as isize - get_number_of_token(positions, Token::Black) as isize) * 10;

    // Gewichtung hängt ab von der Gesamtanzahl an "farblosen" Mühlen. (Optimierung für die Zukunft)
    score += (get_number_of_mills(positions, Token::White) as isize - get_number_of_mills(positions, Token::Black) as isize) * 30;
    
    // Mögliche Züge
    score += list_moves(&positions, Token::White, phase).count() as isize - list_moves(&positions, Token::Black, phase).count() as isize;

    let winning_player = get_winner(positions, phase);
    if winning_player == Token::White {
        return isize::MAX
    } else if winning_player == Token::Black {
        return isize::MIN
    }

    return score
}
