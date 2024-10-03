use crate::agent::AiPhase;
use super::move_token_count::{extract_black_token_count_from_board, extract_white_token_count_from_board, update_possible_move_count, BLACK_TOKEN_FIRST_POSITION, WHITE_TOKEN_FIRST_POSITION};
use super::{action::list_actions, game_state::Phase, position::{negate_token, set_token_at}};

pub fn forward_step_boards<'a>(board: &'a u64, token_type: u8, phase: AiPhase) -> impl Iterator<Item=u64> + 'a {
    let number_of_token = if token_type == 0b11 {
        extract_white_token_count_from_board(*board) as u8
    } else {
        extract_black_token_count_from_board(*board) as u8
    };

    list_actions(board, token_type, phase.phase, Some(number_of_token)).map(move |action| {
        let mut new_board = board.clone();
        
        if phase.phase == Phase::Set && phase.step_counter >= 4 {
            new_board += if token_type == 0b11 {
                WHITE_TOKEN_FIRST_POSITION
            } else {
                BLACK_TOKEN_FIRST_POSITION
            };
        }
        
        if phase.phase == Phase::Move {
            new_board = set_token_at(new_board, action.start_position.unwrap(), 0b00);
            new_board = update_possible_move_count(new_board, token_type, action.start_position.unwrap(), true)
        }
        new_board = set_token_at(new_board, action.end_position, token_type);
        new_board = update_possible_move_count(new_board, token_type, action.end_position, false);

        if action.beatable_position.is_some() {
            new_board = set_token_at(new_board, action.beatable_position.unwrap(), 0b00);
            new_board -= if token_type == 0b11 {
                BLACK_TOKEN_FIRST_POSITION
            } else {
                WHITE_TOKEN_FIRST_POSITION
            };
            new_board = update_possible_move_count(new_board, negate_token(token_type), action.beatable_position.unwrap(), true);
        }

        return new_board;
    })
}

#[cfg(test)]
mod tests {
    use crate::{agent::AiPhase, logic::{game_state::{Phase, Token}, move_token_count::{BLACK_POSSIBLE_MOVES_FIRST_POSITION, BLACK_TOKEN_FIRST_POSITION, WHITE_POSSIBLE_MOVES_FIRST_POSITION, WHITE_TOKEN_FIRST_POSITION}, position::decode_positions}};
    use super::forward_step_boards;

    /*
        For the following test cases for forward_step_boards you have to always substract
        -2 from the real number of black or white tokens due to the implementation of the token counter
        000 -> 2
        001 -> 3
        ...  
    */
    #[test]
    fn test1_forward_step_boards() {
        let token_count = BLACK_TOKEN_FIRST_POSITION + 2 * WHITE_TOKEN_FIRST_POSITION;
        let black_2_moves = 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION;
        let white_9_moves = 9 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let white_6_moves = 6 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let board1 = 
            decode_positions("WEEEEBBBWWEEEEEEWEEEEEEE".to_string())
            + black_2_moves
            + 5 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + token_count;
    
        let expected_boards = vec![
            decode_positions("EWEEEBBBWWEEEEEEWEEEEEEE".to_string())
            + token_count
            + 3 * BLACK_POSSIBLE_MOVES_FIRST_POSITION
            + 7 * WHITE_POSSIBLE_MOVES_FIRST_POSITION,
            decode_positions("WEEEEBBBEWEEEEEWWEEEEEEE".to_string())
            + token_count
            + black_2_moves
            + white_9_moves,
            decode_positions("WEEEEBBBWEWEEEEEWEEEEEEE".to_string())
            + token_count
            + black_2_moves
            + white_9_moves,
            decode_positions("WEEEEBBBWWEEEEEEEWEEEEEE".to_string())
            + token_count
            + black_2_moves
            + white_6_moves,
            decode_positions("WEEEEBBBWWEEEEEEEEEEEEEW".to_string())
            + token_count
            + black_2_moves
            + white_6_moves
        ];
        
        for forward_board in forward_step_boards(&board1, Token::parse_to_u8(Token::White), AiPhase::new(Phase::Move, 25)) {
            assert!(expected_boards.contains(&forward_board));
        }
    }

    #[test]
    fn test2_forward_step_boards() {
        let token_count = 2 * BLACK_TOKEN_FIRST_POSITION + 3 * WHITE_TOKEN_FIRST_POSITION;
        let black_2_moves = 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION;
        let black_3_moves = 3 * BLACK_POSSIBLE_MOVES_FIRST_POSITION;
        let white_8_moves = 8 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let white_9_moves = 9 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let white_10_moves = 10 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let board = 
            decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string())
            + token_count
            + white_8_moves
            + black_2_moves;
        let expected_boards = vec![
            decode_positions("EWEEBBBBWWEEEEWEWEEEEEEE".to_string())
            + token_count
            + white_10_moves
            + black_3_moves,
            decode_positions("WEEEBBBBEWEEEEWWWEEEEEEE".to_string())
            + token_count
            + white_10_moves
            + black_2_moves,
            decode_positions("WEEEBBBBWEWEEEWEWEEEEEEE".to_string())
            + token_count
            + 12 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + black_2_moves,
            decode_positions("WEEEBBBBWWEEEEWEEWEEEEEE".to_string())
            + token_count
            + white_9_moves
            + black_2_moves,
            decode_positions("WEEEBBBBWWEEEEWEEEEEEEEW".to_string())
            + token_count
            + white_9_moves
            + black_2_moves,
            decode_positions("WEEEBBBBWWEEEWEEWEEEEEEE".to_string())
            + token_count
            + 7 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + black_3_moves,
            decode_positions("WEEEBBBBWWEEEEEEWEEEEEWE".to_string())
            + token_count
            + white_8_moves
            + black_3_moves,
            decode_positions("WEEEEBBBWWEEEEEWWEEEEEEE".to_string()) // with mill
            + BLACK_TOKEN_FIRST_POSITION
            + 3 * WHITE_TOKEN_FIRST_POSITION
            + 5 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + black_2_moves 
        ];

        for forward_board in forward_step_boards(&board, Token::parse_to_u8(Token::White), AiPhase::new(Phase::Move, 25)) {
            assert!(expected_boards.contains(&forward_board));
        }
    }

    #[test]
    fn test3_forward_steps_bourds() {
        let token_count = 5 * BLACK_TOKEN_FIRST_POSITION + 6 * WHITE_TOKEN_FIRST_POSITION;
        let white_7_moves = 7 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let white_8_moves = 8 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let white_9_moves = 9 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let black_2_moves = 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION;
        let board = 
            decode_positions("BWEEEWBBWBBWWBBWEEEEEEWE".to_string())
            + 5 * BLACK_TOKEN_FIRST_POSITION
            + 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION
            + 5 * WHITE_TOKEN_FIRST_POSITION
            + 7 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        let expected_boards = vec![
            decode_positions("BWEEEWBBWBBWWBBWWEEEEEWE".to_string())
            + token_count
            + white_8_moves
            + black_2_moves,
            decode_positions("BWEEEWBBWBBWWBBWEWEEEEWE".to_string())
            + token_count
            + white_9_moves
            + black_2_moves,
            decode_positions("BWEEEWBBWBBWWBBWEEWEEEWE".to_string())
            + token_count
            + white_9_moves
            + BLACK_POSSIBLE_MOVES_FIRST_POSITION,
            decode_positions("BWEEEWBBWBBWWBBWEEEWEEWE".to_string())
            + token_count
            + white_9_moves
            + black_2_moves,
            decode_positions("BWEEEWBBWBBWWBBWEEEEWEWE".to_string())
            + token_count
            + white_8_moves
            + black_2_moves,
            decode_positions("BWEEEWBBWBBWWBBWEEEEEWWE".to_string())
            + token_count
            + white_7_moves
            + black_2_moves,
            decode_positions("BWEEEWBBWBBWWBBWEEEEEEWW".to_string())
            + token_count
            + white_7_moves
            + black_2_moves,
            decode_positions("BWWEEWBBWBBWWBBWEEEEEEWE".to_string())
            + token_count
            + white_7_moves
            + BLACK_POSSIBLE_MOVES_FIRST_POSITION,
            decode_positions("BWEWEWBBWBBWWBBWEEEEEEWE".to_string())
            + token_count
            + white_9_moves
            + black_2_moves,
            decode_positions("BWEEWWBBWBBWWBBWEEEEEEWE".to_string())
            + token_count
            + 6 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + black_2_moves
        ];

        for forward_board in forward_step_boards(&board, Token::parse_to_u8(Token::White), AiPhase::new(Phase::Set, 14)) {
            assert!(expected_boards.contains(&forward_board));
        }
    }
}
