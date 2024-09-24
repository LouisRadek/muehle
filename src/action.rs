use std::iter;
use crate::{game_state::Phase, mill_detection::is_mill_closing, r#move::{is_beat_possible, is_move_valid}, position::{create_token_iter, get_number_of_tokens, set_token_at}};

pub struct Action {
    pub start_position: Option<usize>,
    pub end_position: usize,
    pub beatable_position: Option<usize>,
}

impl Action {
    pub fn new(start_position: Option<usize>, end_position: usize, beatable_position: Option<usize>) -> Self {
        Action { start_position, end_position, beatable_position }
    }
}

pub fn forward_step_boards<'a>(board: &'a u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + 'a {
    list_moves(&board, token_type, phase)
        .flat_map(move |applyed_move_board| {
            if is_mill_closing(*board, applyed_move_board, token_type) {
                return itertools::Either::Left(
                    create_token_iter(*board).enumerate()
                        .filter(move |(index, _)| is_beat_possible(*board, *index, token_type))
                        .map(move |(beatable_position, _)| {
                            set_token_at(applyed_move_board, beatable_position, 0b00)
                        }))
            } else {
                itertools::Either::Right(iter::once(applyed_move_board))
            }
    })
}

pub fn list_moves<'a>(board: &'a u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + 'a {
    if phase == Phase::Set {
        let mut shifted: u64 = 0b11;
        let token_extended: u64 = if token_type == 0b11 {
            0b111111111111111111111111111111111111111111111111
        } else {
            0b101010101010101010101010101010101010101010101010
        };

        return itertools::Either::Left(
            (0..24).filter_map(move |_| {
                let result = if *board & shifted == 0 {
                    Some(*board | (shifted & token_extended))
                } else {
                    None
                };
                shifted <<= 2;
                result
            }))
    } else {
        let number_of_token = get_number_of_tokens(*board, token_type);

        return itertools::Either::Right(
            create_token_iter(*board).enumerate()
                .filter(move |(_, token)| *token == token_type)
                    .flat_map(move |(start_position, _)| {
                        create_token_iter(*board)
                            .enumerate()
                            .filter_map(move |(end_position, end_token)| {
                                if is_move_valid(start_position, end_position, end_token, number_of_token) {
                                    let new_board = set_token_at(*board, start_position, 0b00);
                                    Some(set_token_at(new_board, end_position, token_type))
                                } else {
                                    None
                                }
                            })
                    })
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{action::forward_step_boards, game_state::{Phase, Token}, position::{decode_positions, encode_positions, set_token_at}};

    use super::list_moves;

    #[test]
    fn test1_forward_step_boards() {
        let board1 = decode_positions("WEEEEBBBWWEEEEEEWEEEEEEE".to_string());
        let expected_boards = vec![
            decode_positions("EWEEEBBBWWEEEEEEWEEEEEEE".to_string()),
            decode_positions("WEEEEBBBEWEEEEEWWEEEEEEE".to_string()),
            decode_positions("WEEEEBBBWEWEEEEEWEEEEEEE".to_string()),
            decode_positions("WEEEEBBBWWEEEEEEEWEEEEEE".to_string()),
            decode_positions("WEEEEBBBWWEEEEEEEEEEEEEW".to_string())
        ];

        for forward_board in forward_step_boards(&board1, Token::parse_to_u8(Token::White), Phase::Move) {
            assert!(expected_boards.contains(&forward_board));
        }
    }

    #[test]
    fn test2_forward_step_boards() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());
        let expected_boards = vec![
            decode_positions("EWEEBBBBWWEEEEWEWEEEEEEE".to_string()),
            decode_positions("WEEEBBBBEWEEEEWWWEEEEEEE".to_string()),
            decode_positions("WEEEBBBBWEWEEEWEWEEEEEEE".to_string()),
            decode_positions("WEEEBBBBWWEEEEWEEWEEEEEE".to_string()),
            decode_positions("WEEEBBBBWWEEEEWEEEEEEEEW".to_string()),
            decode_positions("WEEEBBBBWWEEEWEEWEEEEEEE".to_string()),
            decode_positions("WEEEBBBBWWEEEEEEWEEEEEWE".to_string()),
            decode_positions("WEEEEBBBWWEEEEEWWEEEEEEE".to_string()) // with mill
        ];

        for forward_board in forward_step_boards(&board, Token::parse_to_u8(Token::White), Phase::Move) {
            assert!(expected_boards.contains(&forward_board));
        }
    }

    #[test]
    fn test3_forward_steps_bourds() {
        let board = decode_positions("BWEEEWBBWBBWWBBWEEEEEEWE".to_string());
        let expected_boards = vec![
            decode_positions("BWEEEWBBWBBWWBBWWEEEEEWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEWEEEEWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEEWEEEWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEEEWEEWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEEEEWEWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEEEEEWWE".to_string()),
            decode_positions("BWEEEWBBWBBWWBBWEEEEEEWW".to_string()),
            decode_positions("BWWEEWBBWBBWWBBWEEEEEEWE".to_string()),
            decode_positions("BWEWEWBBWBBWWBBWEEEEEEWE".to_string()),
            decode_positions("BWEEWWBBWBBWWBBWEEEEEEWE".to_string()),
        ];

        for forward_board in forward_step_boards(&board, Token::parse_to_u8(Token::White), Phase::Set) {
            assert!(expected_boards.contains(&forward_board));
        }
    }

    #[test]
    fn test_list_moves() {
        let board: u64 = 0b111000000000111000000011000011001100101010101010; // WBEEEEWBEEEWEEWEWEBBBBBB
        let expected_moves = vec![
            (Some(0), 8),
            (Some(6), 5),
            (Some(11), 10),
            (Some(11), 12),
            (Some(14), 13),
            (Some(14), 15),
            (Some(16), 8),
            (Some(16), 17)
        ];

        let mut expected_boards = Vec::new();
        for expected_move in expected_moves {
            let mut applyed_move_board = board.clone();
            applyed_move_board = set_token_at(applyed_move_board, expected_move.0.unwrap(), 0b00);
            applyed_move_board = set_token_at(applyed_move_board, expected_move.1, 0b11);
            expected_boards.push(applyed_move_board);
        }

        for ele in list_moves(&board, 0b11, Phase::Move) {
            assert!(expected_boards.contains(&ele));
        }
    }
}
