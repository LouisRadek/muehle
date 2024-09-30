use std::iter;

use crate::{position::{create_token_iter, encode_single_position, negate_token, set_token_at, BLACK_TOKEN_FIRST_POSITION, WHITE_TOKEN_FIRST_POSITION}, utils::{extract_black_token_count_from_board, extract_white_token_count_from_board, is_beat_possible, is_mill_closing, is_move_valid, update_possible_move_count}, Phase, PhaseEnum};

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

impl ToString for Action {
    fn to_string(&self) -> String {
        let mut encoded_string = String::new();
        if self.start_position.is_none() {
            encoded_string.push('P');
            encoded_string.push(' ');
            
            encoded_string.push_str(encode_single_position(self.end_position).as_str());
        } else {
            encoded_string.push('M');
            encoded_string.push(' ');
            
            encoded_string.push_str(encode_single_position(self.start_position.unwrap()).as_str());
            encoded_string.push(' ');
            encoded_string.push_str(encode_single_position(self.end_position).as_str());
        };
        
        if self.beatable_position.is_some() {
            encoded_string.push(' ');
            encoded_string.push('T');
            encoded_string.push(' ');
            encoded_string.push_str(encode_single_position(self.beatable_position.unwrap()).as_str());
        }
        
        return encoded_string;
    }
}

pub fn forward_step_boards<'a>(board: &'a u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + 'a {
    list_moves(board, token_type, phase)
        .flat_map(move |applyed_move_board| {
            if is_mill_closing(*board, applyed_move_board, token_type) {
                itertools::Either::Left(
                    create_token_iter(*board).enumerate()
                        .filter(move |(index, _)| is_beat_possible(*board, *index, token_type))
                        .map(move |(beatable_position, _)| {
                            let mut new_board = set_token_at(applyed_move_board, beatable_position, 0b00);
                            new_board -= if token_type == 0b11 {
                                BLACK_TOKEN_FIRST_POSITION
                            } else {
                                WHITE_TOKEN_FIRST_POSITION
                            };
                            update_possible_move_count(new_board, negate_token(token_type), beatable_position, true)
                        }))
            } else {
                itertools::Either::Right(iter::once(applyed_move_board))
            }
    })
}

pub fn list_moves<'a>(board: &'a u64, token_type: u8, phase: Phase) -> impl Iterator<Item=u64> + 'a {
    let token_extended: u64 = if token_type == 0b11 {
        0b111111111111111111111111111111111111111111111111
    } else {
        0b101010101010101010101010101010101010101010101010
    };

    if phase.phase == PhaseEnum::Set {
        let mut shifted: u64 = 0b11;

        return itertools::Either::Left(
            (0..24).filter_map(move |index| {
                let result = if *board & shifted == 0 {
                    let mut new_board = *board | (shifted & token_extended);
                    if phase.step_counter >= 4 {
                        if token_type == 0b11 {
                            new_board += WHITE_TOKEN_FIRST_POSITION;
                        } else {
                            new_board += BLACK_TOKEN_FIRST_POSITION;
                        }
                    }
                    new_board = update_possible_move_count(new_board, token_type, 23 - index, false);
                    Some(new_board)
                } else {
                    None
                };
                shifted <<= 2;
                result
            }))
    } else {
        let number_of_token = if token_type == 0b11 {
            extract_white_token_count_from_board(*board)
        } else {
            extract_black_token_count_from_board(*board)
        };

        return itertools::Either::Right(
            create_token_iter(*board).enumerate()
                .filter(move |(_, token)| *token == token_type)
                    .flat_map(move |(start_position, _)| {
                        let mut new_board = set_token_at(*board, start_position, 0b00);
                        new_board = update_possible_move_count(new_board, token_type, start_position, true);

                        create_token_iter(*board)
                            .enumerate()
                            .filter_map(move |(end_position, end_token)| {
                                if is_move_valid(start_position, end_position, end_token, number_of_token as u8) {
                                    new_board = update_possible_move_count(new_board, token_type, end_position, false);
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
    use crate::{position::{print_board, set_token_at}, utils::get_action_from_board, Phase, PhaseEnum};

    use super::*;

    #[test]
    fn test_forward_step_boards2() {
        let board = 0b0;
        let phase = Phase::new(PhaseEnum::Set, 0);

        for forward_step in forward_step_boards(&board, 0b11, phase) {
            for inner_forward_step in forward_step_boards(&forward_step, 0b10, phase) {
                println!("{:#066b}", inner_forward_step);
            }
        }
    }

    #[test]
    fn test_speed_of_list_moves() {
        let board = 0b111000000000111000000011000011001100101010101010; // 4.9s, 5.8s, 5.1s, 5.0s, 5.1s, 5.0s
        let phase = Phase::new(PhaseEnum::Move, 17);
        
        let now = std::time::Instant::now();

        for _ in 0..40000000 {
            let _ = forward_step_boards(&board, 0b11, phase);
        }
        println!("The test took: {:?}", now.elapsed());
    }

    #[test]
    fn test_generate_actions() {
        use crate::position::decode_positions;
        use crate::{Phase, PhaseEnum};

        let now = std::time::Instant::now();

        let board = "WBEEEEWBEEEWEEWEWEBBBBBB".to_string();
        let decoded_board = decode_positions(board);

        for forward_board in forward_step_boards(&decoded_board, 0b10, Phase::new(PhaseEnum::Set, 1)) {
            println!("forward board: {}", forward_board);
        }
        println!("Time elapsed: {:?}", now.elapsed());
    }

    #[test]
    fn test_list_moves() {
        use crate::action::list_moves;
        use crate::{Phase, PhaseEnum};

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

        for ele in list_moves(&board, 0b11, Phase::new(PhaseEnum::Move, 1)) {
            assert!(expected_boards.contains(&ele));
        }
    }

    #[test]
    fn test_forward_step_boards() {
        use crate::position::{decode_positions, encode_positions};
        use crate::{Phase, PhaseEnum};

        let board = decode_positions("EEEEEEEEEEEEEEEEEWBWEBBW".to_string());
        print_board(board);
        println!("{}", encode_positions(board));

        for forward_board in forward_step_boards(&board, 0b11, Phase::new(PhaseEnum::Set, 18)) {
            let action = get_action_from_board(board, forward_board, 0b11);
            println!("{}, move: start_{}, end_{}, beat_{}, --- {}", encode_positions(forward_board), action.start_position.unwrap_or(30), action.end_position, action.beatable_position.unwrap_or(30), action.to_string());
        }
    }
}
