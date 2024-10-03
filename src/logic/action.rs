use std::iter;
use crate::logic::{game_state::Phase, mill_detection::is_mill_closing, r#move::{apply_move, is_beat_possible, is_move_valid}, position::{create_token_iter, get_number_of_tokens}};

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub struct Move {
    pub start_position: Option<usize>,
    pub end_position: usize
}

impl Move {
    pub fn new(start_position: Option<usize>, end_position: usize) -> Self {
        Move { start_position, end_position }
    }
}

pub fn list_actions<'a>(board: &'a u64, token_type: u8, phase: Phase, number_of_token: Option<u8>) -> impl Iterator<Item=Action> + 'a {
    list_moves(&board, token_type, phase, number_of_token)
        .flat_map(move |possible_move| {
            if is_mill_closing(*board, apply_move(board, &possible_move, token_type), token_type) {
                return itertools::Either::Left(
                    create_token_iter(*board).enumerate()
                        .filter_map(move |(beatable_position, _)| {
                            if is_beat_possible(*board, beatable_position, token_type) {
                                Some(Action::new(possible_move.start_position, possible_move.end_position, Some(beatable_position)))
                            } else {
                                None
                            }
                        }))
            } else {
                itertools::Either::Right(iter::once(Action::new(possible_move.start_position, possible_move.end_position, None)))
            }
    })
}

pub fn list_moves<'a>(board: &'a u64, token_type: u8, phase: Phase, number_of_token: Option<u8>) -> impl Iterator<Item=Move> + 'a {
    if phase == Phase::Set {
        itertools::Either::Left(list_moves_set_phase(*board))
    } else {
        itertools::Either::Right(list_moves_move_phase(board, token_type, number_of_token))
    }
}

fn list_moves_move_phase<'a>(board: &'a u64, token_type: u8, number_of_token: Option<u8>) -> impl Iterator<Item=Move> + 'a {
    create_token_iter(*board).enumerate()
        .filter(move |(_, token)| *token == token_type)
            .flat_map(move |(start_position, _)| {
                create_token_iter(*board)
                    .enumerate()
                    .filter_map(move |(end_position, end_token)| {
                        if is_move_valid(
                            start_position, 
                            end_position, 
                            end_token, 
                            number_of_token.unwrap_or(get_number_of_tokens(*board, token_type))
                        ) {
                            Some(Move::new(Some(start_position), end_position))
                        } else {
                            None
                        }
                    })
            })
}

fn list_moves_set_phase(board: u64) -> impl Iterator<Item=Move> {
    return create_token_iter(board)
        .enumerate()
        .filter_map(move |(position, token)| {
            if token == 0b00 {
                Some(Move::new(None, position))
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::logic::{game_state::{Phase, Token}, position::decode_positions};
    use super::{list_actions, list_moves, list_moves_move_phase, list_moves_set_phase, Action, Move};

    #[test]
    fn test_list_actions() {
        let board = decode_positions("BWWEWBBBBEEWWWEEBEEEEEEE".to_string());
        let expected_actions = vec![
            Action::new(Some(2), 10, None),
            Action::new(Some(2), 3, None),
            Action::new(Some(11), 10, None),
            Action::new(Some(12), 20, None),
            Action::new(Some(13), 14, None),
            Action::new(Some(4), 3, Some(5)),
            Action::new(Some(4), 3, Some(6)),
            Action::new(Some(4), 3, Some(7)),
            Action::new(Some(4), 3, Some(0)),
            Action::new(Some(4), 3, Some(8)),
            Action::new(Some(4), 3, Some(16))
        ];

        for action in list_actions(&board, Token::parse_to_u8(Token::White), Phase::Move, None) {
            assert!(expected_actions.contains(&action));
        }
    }

    #[test]
    fn test_list_actions2() {
        let board = decode_positions("WBBEBWWWWEEBBBEEWWEEEEEE".to_string());
        let expected_actions = vec![
            Action::new(Some(2), 10, None),
            Action::new(Some(2), 3, None),
            Action::new(Some(11), 10, None),
            Action::new(Some(12), 20, None),
            Action::new(Some(13), 14, None),
            Action::new(Some(4), 3, Some(17))
        ];

        for action in list_actions(&board, Token::parse_to_u8(Token::Black), Phase::Move, None) {
            assert!(expected_actions.contains(&action));
        }
    }

    #[test]
    fn test_list_actions3() {
        let board = decode_positions("BWWEWBBBBEEWWWEBBEEEEEEE".to_string());
        let expected_actions = vec![
            Action::new(None, 9, None),
            Action::new(None, 10, None),
            Action::new(None, 14, None),
            Action::new(None, 15, None),
            Action::new(None, 17, None),
            Action::new(None, 18, None),
            Action::new(None, 19, None),
            Action::new(None, 20, Some(15)),
            Action::new(None, 21, None),
            Action::new(None, 22, None),
            Action::new(None, 23, None),
            Action::new(None, 3, Some(15)),
        ];

        for action in list_actions(&board, Token::parse_to_u8(Token::White), Phase::Set, None) {
            assert!(expected_actions.contains(&action));
        }
    }

    #[test]
    fn test_list_moves() {
        let board = decode_positions("WBBEBWWWWEEBBBEEWEEEEEEE".to_string());
        let moves_set_phase = list_moves(&board, Token::parse_to_u8(Token::Black), Phase::Set, None).collect::<Vec<Move>>();
        let expected_move_set_phase = list_moves_set_phase(board).collect::<Vec<Move>>();
        assert_eq!(moves_set_phase, expected_move_set_phase);

        let board2 = decode_positions("BWWEWBBBBEEWWWEEBEEEEEEE".to_string());
        let moves_move_phase = list_moves(&board2, Token::parse_to_u8(Token::White), Phase::Move, None).collect::<Vec<Move>>();
        let expected_move_move_phase = list_moves_move_phase(&board2, Token::parse_to_u8(Token::White), None).collect::<Vec<Move>>();
        assert_eq!(moves_move_phase, expected_move_move_phase);
    }

    #[test]
    fn test_list_moves_move_phase() {
        let board = decode_positions("BWWEWBBBBEEWWWEEBEEEEEEE".to_string());
        let expected_moves = vec![
            Move::new(Some(2), 10),
            Move::new(Some(2), 3),
            Move::new(Some(11), 10),
            Move::new(Some(4), 3),
            Move::new(Some(12), 20),
            Move::new(Some(13), 14),
        ];

        for r#move in list_moves_move_phase(&board, Token::parse_to_u8(Token::White), None) {
            assert!(expected_moves.contains(&r#move));
        }
    }

    #[test]
    fn test_list_moves_set_phase() {
        let board = decode_positions("WBBEBWWWWEEBBBEEWEEEEEEE".to_string());
        let expected_moves = vec![
            Move::new(None, 3),
            Move::new(None, 9),
            Move::new(None, 10),
            Move::new(None, 14),
            Move::new(None, 15),
            Move::new(None, 17),
            Move::new(None, 18),
            Move::new(None, 19),
            Move::new(None, 20),
            Move::new(None, 21),
            Move::new(None, 22),
            Move::new(None, 23),
        ];

        for r#move in list_moves_set_phase(board) {
            assert!(expected_moves.contains(&r#move));
        }
    }
}
