const POSSIBLE_MILLS_WHITE: [u64; 16] = [
    // horizontal mills
    0b111100000000001100000000000000000000000000000000, // 7 0 1
    0b000000000000000011110000000000110000000000000000, // 15 8 9
    0b000000000000000000000000000000001111000000000011, // 23 16 17
    0b000000000000110000000000000011000000000000001100, // 6 14 22
    0b000011000000000000001100000000000000110000000000, // 18 10 2
    0b000000000000000000000000000000000000001111110000, // 21 20 19
    0b000000000000000000000011111100000000000000000000, // 13 12 11
    0b000000111111000000000000000000000000000000000000, // 5 4 3
    // vertical mills
    0b000000000011111100000000000000000000000000000000, // 7 6 5
    0b000000000000000000000000001111110000000000000000, // 15 14 13
    0b000000000000000000000000000000000000000000111111, // 23 22 21
    0b110000000000000011000000000000001100000000000000, // 0 8 16
    0b000000001100000000000000110000000000000011000000, // 20 12 4
    0b000000000000000000000000000000000011111100000000, // 17 18 19
    0b000000000000000000111111000000000000000000000000, // 9 10 11
    0b001111110000000000000000000000000000000000000000, // 1 2 3
];

const POSSIBLE_MILLS_BLACK: [u64; 16] = [
    // horizontal mill
    0b101000000000001000000000000000000000000000000000, // 7 0 1
    0b000000000000000010100000000000100000000000000000, // 15 8 9
    0b000000000000000000000000000000001010000000000010, // 23 16 17
    0b000000000000100000000000000010000000000000001000, // 6 14 22
    0b000010000000000000001000000000000000100000000000, // 18 10 2
    0b000000000000000000000000000000000000001010100000, // 21 20 19
    0b000000000000000000000010101000000000000000000000, // 13 12 11
    0b000000101010000000000000000000000000000000000000, // 5 4 3
    // vertical mills
    0b000000000010101000000000000000000000000000000000, // 7 6 5
    0b000000000000000000000000001010100000000000000000, // 15 14 13
    0b000000000000000000000000000000000000000000101010, // 23 22 21
    0b100000000000000010000000000000001000000000000000, // 0 8 16
    0b000000001000000000000000100000000000000010000000, // 20 12 4
    0b000000000000000000000000000000000010101000000000, // 17 18 19
    0b000000000000000000101010000000000000000000000000, // 9 10 11
    0b001010100000000000000000000000000000000000000000, // 1 2 3
];

const MILL_INDICES_FOR_POSITION: [(usize, usize); 24] = [
    (0, 11),
    (0, 15),
    (4, 15),
    (7, 15),
    (7, 12),
    (7, 8),
    (3, 8),
    (0, 8),
    (1, 11),
    (1, 14),
    (4, 14),
    (6, 14),
    (6, 12),
    (6, 9),
    (3, 9),
    (1, 9),
    (2, 11),
    (2, 13),
    (4, 13),
    (5, 13),
    (5, 12),
    (5, 10),
    (3, 10),
    (2, 10),
];

pub const NEIGHBORS: [[usize; 4]; 24] = [
    [1, 7, 8, 24],
    [0, 2, 24, 24],
    [1, 3, 10, 24],
    [2, 4, 24, 24],
    [3, 5, 12, 24],
    [4, 6, 24, 24],
    [5, 7, 14, 24],
    [0, 6, 24, 24],
    [0, 9, 15, 16],
    [8, 10, 24, 24],
    [2, 9, 11, 18],
    [10, 12, 24, 24],
    [4, 11, 13, 20],
    [12, 14, 24, 24],
    [6, 13, 15, 22],
    [8, 14, 24, 24],
    [8, 17, 23, 24],
    [16, 18, 24, 24],
    [10, 17, 19, 24],
    [18, 20, 24, 24],
    [12, 19, 21, 24],
    [20, 22, 24, 24],
    [14, 21, 23, 24],
    [16, 22, 24, 24],
];

use crate::agent::{action::Action, position::{get_token_at, negate_token, BLACK_POSSIBLE_MOVES_FIRST_POSITION, WHITE_POSSIBLE_MOVES_FIRST_POSITION}, Phase, PhaseEnum};

#[allow(dead_code)]
pub fn get_winner(board: u64, phase: Phase) -> u8 {
    let (black_tokens, white_tokens) = (get_number_of_tokens(board, 0b10), get_number_of_tokens(board, 0b11));
    if phase.phase == PhaseEnum::Move && white_tokens < 3 {
        return 0b10
    } else if phase.phase == PhaseEnum::Move && black_tokens < 3 {
        return 0b11
    } else { 
        return 0b00 
    }
}

pub fn get_number_of_tokens(board: u64, token: u8) -> u8 {
    if token == 0b11 {
        (board & 0b010101010101010101010101010101010101010101010101).count_ones() as u8
    } else {
        ((!board & 0b010101010101010101010101010101010101010101010101).count_ones() + (board & 0b101010101010101010101010101010101010101010101010).count_ones() - 24) as u8
    }
}

pub fn insert_token_count_to_board(board: u64) -> u64 {
    let white_token_count: u64 = match get_number_of_tokens(board, 0b11)
    {
        0 => 0, // 0 -> 000
        1 => 0, // 1 -> 000
        val => val as u64 - 2 // 000 -> 2; 001 -> 3; ...; 110 - 8; 111 - 9
    };
    let black_token_count: u64 = match get_number_of_tokens(board, 0b10)
    {
        0 => 0,
        1 => 0,
        val => val as u64 - 2
    };

    (board & 0b1111111111000000111111111111111111111111111111111111111111111111) | (white_token_count << 51) | (black_token_count << 48)
}

pub fn insert_number_of_possible_moves_to_board(board: u64) -> u64 {
    let white_possible_moves = get_possible_move_count(board, 0b11) as u64;
    let black_possible_moves = get_possible_move_count(board, 0b10) as u64;

    (board & 0b0000000000111111111111111111111111111111111111111111111111111111) | (white_possible_moves << 59) | (black_possible_moves << 54)
}

pub fn extract_white_token_count_from_board(board: u64) -> u64 {
    ((board & 0b0000000000111000000000000000000000000000000000000000000000000000) >> 51) + 2
}

pub fn extract_black_token_count_from_board(board: u64) -> u64 {
    ((board & 0b0000000000000111000000000000000000000000000000000000000000000000) >> 48) + 2
}

pub fn extract_white_move_count_from_board(board: u64) -> u64 {
    (board & 0b1111100000000000000000000000000000000000000000000000000000000000) >> 59
}

pub fn extract_black_move_count_from_board(board: u64) -> u64 {
    (board & 0b0000011111000000000000000000000000000000000000000000000000000000) >> 54
}

pub fn is_move_valid(start_position: usize, end_position: usize, end_token: u8, number_of_token_type: u8) -> bool {
    if end_token != 0b00 {
        return false
    }
    
    if number_of_token_type == 3 || is_neighbor(start_position, end_position) {
        return true
    }

    return false
}

pub fn get_possible_move_count(board: u64, token_type: u8) -> usize {
    let mut board_mut = board;
    let mut count = 0;
    let mut index: isize = 23;
    while index >= 0 {
        if (board_mut & 0b11) as u8 == token_type {
            for neighbor in NEIGHBORS[index as usize].iter() {
                if *neighbor != 24 && get_token_at(board, *neighbor) == 0b00 {
                    count += 1;
                }
            }
        }
        index -= 1;
        board_mut >>= 2;
    }
    return count
}

// we add white token to board at position "position":
//  neighbor is empty => add possible move to white
//  neighbor is white => remove possible move from white
//  neighbor is black => remove possible move from black
// 
// we add white token to board at position "position":
//  neighbor is empty => add possible move to black
//  neighbor is white => remove possible move from white
//  neighbor is black => remove possible move from black
//
// we remove token at postion "position"
//  neighbor is empty => remove specific to token type of position
//  neighbor is white => add possible move from white
//  neighbor is black => add possible move from black
pub fn update_possible_move_count(mut board: u64, token_type: u8, position: usize, remove: bool) -> u64 {
    if remove {
        NEIGHBORS[position].iter()
            .for_each(|neighbor| {
                if *neighbor == 24 {
                    ()
                }
                match get_token_at(board, *neighbor) {
                    0b00 => if token_type == 0b11 {
                            board -= WHITE_POSSIBLE_MOVES_FIRST_POSITION
                        } else {
                            board -= BLACK_POSSIBLE_MOVES_FIRST_POSITION
                        },
                    0b11 => board += WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                    0b10 => board += BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                    _ => ()
                }
        });
    } else {
        NEIGHBORS[position].iter()
            .for_each(|neighbor| {
                if *neighbor == 24 {
                    ()
                }
                match get_token_at(board, *neighbor) {
                    0b00 => if token_type == 0b11 && !remove {
                            board += WHITE_POSSIBLE_MOVES_FIRST_POSITION
                        } else {
                            board += BLACK_POSSIBLE_MOVES_FIRST_POSITION
                        },
                    0b11 => board -= WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                    0b10 => board -= BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                    _ => ()
                }
        });
    }
    return board
}

fn is_neighbor(position1: usize, position2: usize) -> bool {
    NEIGHBORS[position1].contains(&position2)
}

pub fn is_part_of_mill(board: u64, position: usize, token_type: u8) -> bool {
    let (index1, index2) = MILL_INDICES_FOR_POSITION[position];
    let possible_mill_position = POSSIBLE_MILLS_WHITE[index1];
    let possible_mill_position2 = POSSIBLE_MILLS_WHITE[index2];
    if token_type == 0b11 {
        (board & possible_mill_position) == POSSIBLE_MILLS_WHITE[index1]
        || (board & possible_mill_position2) == POSSIBLE_MILLS_WHITE[index2]
    } else if token_type == 0b10 {
        (board & possible_mill_position) == POSSIBLE_MILLS_BLACK[index1]
            || (board & possible_mill_position2) == POSSIBLE_MILLS_BLACK[index2]
    } else {
        false
    }
}

pub fn is_mill_closing(pos_before: u64, pos_after: u64, token_type: u8) -> bool {
    (0..16).any(|i| {
        let possible_mill = if token_type == 0b11 {
            POSSIBLE_MILLS_WHITE[i]
        } else {
            POSSIBLE_MILLS_BLACK[i]
        };

        let possible_mill_position = POSSIBLE_MILLS_WHITE[i];
        let is_mill_before = pos_before & possible_mill_position == possible_mill;
        let is_mill_after = pos_after & possible_mill_position == possible_mill;
        !is_mill_before && is_mill_after
    })
}

pub fn is_all_part_of_mill(board: u64, token_of_opponent: u8) -> bool {
    let mut board_mod: u64 = board;
    !(0..24).any(|index| {
        let token: u8 = (board_mod & 0b11) as u8;
        board_mod >>= 2;
        token == token_of_opponent && !is_part_of_mill(board, 23 - index, token_of_opponent)
    })
}

pub fn is_beat_possible(board: u64, position: usize, token_current_player: u8) -> bool {
    let token_of_opponent: u8 = negate_token(token_current_player);
    
    if get_token_at(board, position) != token_of_opponent {
        return false
    }
    
    if !is_part_of_mill(board, position, token_of_opponent) {
        return true
    }
    
    return is_all_part_of_mill(board, token_of_opponent)
}

/*
    Cases for the board:
    1. outer if:
        E E - don't care
        B B - don't care
        W W - don't care
    2. inner if
        E W - end_position
        W E - start_position
        B E - beatable_position
    3. not accepted
        E B - not possible
        B W - not possible
        W B - not possible
*/
pub fn get_action_from_board(mut board_before: u64, mut board_after: u64, token_type: u8) -> Action {
    let mut start_position = None;
    let mut end_position = 0;
    let mut beatable_position = None;

    (0..24).rev().for_each( |index| {
        if board_before & 0b11 != board_after & 0b11 {
            if board_before & 0b11 == 0b00 {
                end_position = index as usize;
            } else if (board_before & 0b11) as u8 == token_type {
                start_position = Some(index as usize);
            } else {
                beatable_position = Some(index as usize);
            }
        }
        board_before >>= 2;
        board_after >>= 2;
    });

    return Action::new(start_position, end_position, beatable_position)
}

#[cfg(test)]
mod tests {
    use crate::{agent::{position::set_token_at, utils::{extract_black_move_count_from_board, extract_black_token_count_from_board, extract_white_move_count_from_board, extract_white_token_count_from_board, get_number_of_tokens, get_possible_move_count, get_winner, insert_number_of_possible_moves_to_board, insert_token_count_to_board, is_beat_possible, is_mill_closing, is_move_valid, is_neighbor, is_part_of_mill}, Phase, PhaseEnum}, logic::position::{decode_positions, reverse_token_of_board}};

    #[test]
    fn test_get_winner() {
        let board1 = 0b111100000000000000000000000000000000000000101010;
        let board2 =  0b111111101010101111000010001111110000001000100000;
        
        assert_eq!(0b10, get_winner(board1, Phase::new(PhaseEnum::Move, 1)));
        assert_eq!(0b00, get_winner(board1, Phase::new(PhaseEnum::Set, 3)));
        assert_eq!(0b00, get_winner(board2, Phase::new(PhaseEnum::Move, 1)));
        assert_eq!(0b10, get_winner(board1, Phase::new(PhaseEnum::Move, 1)));
        assert_eq!(0b00, get_winner(board1, Phase::new(PhaseEnum::Set, 3)));
        assert_eq!(0b00, get_winner(board2, Phase::new(PhaseEnum::Move, 1)));
    }

    #[test]
    fn test_get_number_of_tokens() {
        
        let board1 = 0b0;
        let board2 = 0b101000000011110011101110110010110011101100100010;
        let board3 = 0b000000000011110011101110110010110011101100100010;
        let board4 = 0b111100000011110011101110110010110011101100100010;

        assert_eq!((0, 0), (get_number_of_tokens(board1, 0b10), get_number_of_tokens(board1, 0b11)));
        assert_eq!((8, 8), (get_number_of_tokens(board2, 0b10), get_number_of_tokens(board2, 0b11)));
        assert_eq!((6, 8), (get_number_of_tokens(board3, 0b10), get_number_of_tokens(board3, 0b11)));
        assert_eq!((6, 10), (get_number_of_tokens(board4, 0b10), get_number_of_tokens(board4, 0b11)));
    }
    
    #[test]
    fn test_insert_token_count_to_board() {
        let board1: u64 = 0b0;
        let board2: u64 = 0b101000000011110011101110110010110011101100100010;
        let board3: u64 = 0b000000000011110011101110110010110011101100100010;
        let board4: u64 = 0b000000000000000000000000000010000000000000000000;
        
        let exp_board1: u64 = 0b0;
        let exp_board2: u64 = 0b0000000000110110101000000011110011101110110010110011101100100010; // 8v8
        let exp_board3: u64 = 0b0000000000110100000000000011110011101110110010110011101100100010; // 8v6
        let exp_board4: u64 = 0b0000000000000000000000000000000000000000000010000000000000000000; // 0v1
        
        assert_eq!(exp_board1, insert_token_count_to_board(board1));
        assert_eq!(exp_board2, insert_token_count_to_board(board2));
        assert_eq!(exp_board3, insert_token_count_to_board(board3));
        assert_eq!(exp_board4, insert_token_count_to_board(board4));
    }

    #[test]
    fn test_insert_number_of_possible_moves_to_board() {
        let board1: u64 = 0b0;
        let board2: u64 = 0b101000000011110011101110110010110011101100100010;
        let board3: u64 = 0b000000000011110011101110110010110011101100100010;
        let board4: u64 = 0b111100000011110011101110110010110011101100100010;
        
        let filter_possible_moves_black = 0b0000011111000000000000000000000000000000000000000000000000000000;
        let filter_possible_moves_white = 0b1111100000000000000000000000000000000000000000000000000000000000;

        let inserted_board1 = insert_number_of_possible_moves_to_board(board1);
        let inserted_possible_move_white1 = (inserted_board1 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black1 = (inserted_board1 & filter_possible_moves_black) >> 54;
        assert_eq!(get_possible_move_count(board1, 0b11), inserted_possible_move_white1 as usize);
        assert_eq!(get_possible_move_count(board1, 0b10), inserted_possible_move_black1 as usize);

        let inserted_board2 = insert_number_of_possible_moves_to_board(board2);
        let inserted_possible_move_white2 = (inserted_board2 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black2 = (inserted_board2 & filter_possible_moves_black) >> 54;
        assert_eq!(get_possible_move_count(board2, 0b11), inserted_possible_move_white2 as usize);
        assert_eq!(get_possible_move_count(board2, 0b10), inserted_possible_move_black2 as usize);

        let inserted_board3 = insert_number_of_possible_moves_to_board(board3);
        let inserted_possible_move_white3 = (inserted_board3 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black3 = (inserted_board3 & filter_possible_moves_black) >> 54;
        assert_eq!(get_possible_move_count(board3, 0b11), inserted_possible_move_white3 as usize);
        assert_eq!(get_possible_move_count(board3, 0b10), inserted_possible_move_black3 as usize);

        let inserted_board4 = insert_number_of_possible_moves_to_board(board4);        
        let inserted_possible_move_white4 = (inserted_board4 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black4 = (inserted_board4 & filter_possible_moves_black) >> 54;
        assert_eq!(get_possible_move_count(board4, 0b11), inserted_possible_move_white4 as usize);
        assert_eq!(get_possible_move_count(board4, 0b10), inserted_possible_move_black4 as usize);
    }

    #[test]
    fn test_extract_methods() {
        let board1: u64 = 0b101000000011110011101110110010110011101100100010;
        let board2: u64 = 0b000000000011110011101110110010110011101100100010;

        let filter_token_black = 0b0000000000000111000000000000000000000000000000000000000000000000;
        let filter_token_white = 0b0000000000111000000000000000000000000000000000000000000000000000;
        let filter_possible_moves_black = 0b0000011111000000000000000000000000000000000000000000000000000000;
        let filter_possible_moves_white = 0b1111100000000000000000000000000000000000000000000000000000000000;

        let move_and_token_count_board1 = insert_number_of_possible_moves_to_board(insert_number_of_possible_moves_to_board(board1));
        let inserted_possible_move_white1 = (move_and_token_count_board1 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black1 = (move_and_token_count_board1 & filter_possible_moves_black) >> 54;
        let inserted_token_count_white1 = (move_and_token_count_board1 & filter_token_white) >> 51;
        let inserted_token_count_black1 = (move_and_token_count_board1 & filter_token_black) >> 48;
        assert_eq!(inserted_possible_move_white1, extract_white_move_count_from_board(move_and_token_count_board1));
        assert_eq!(inserted_possible_move_black1, extract_black_move_count_from_board(move_and_token_count_board1));
        assert_eq!(inserted_token_count_white1, extract_white_token_count_from_board(move_and_token_count_board1) - 2);
        assert_eq!(inserted_token_count_black1, extract_black_token_count_from_board(move_and_token_count_board1) - 2);

        let move_and_token_count_board2 = insert_number_of_possible_moves_to_board(insert_number_of_possible_moves_to_board(board2));
        let inserted_possible_move_white2 = (move_and_token_count_board2 & filter_possible_moves_white) >> 59;
        let inserted_possible_move_black2 = (move_and_token_count_board2 & filter_possible_moves_black) >> 54;
        let inserted_token_count_white2 = (move_and_token_count_board1 & filter_token_white) >> 51;
        let inserted_token_count_black2 = (move_and_token_count_board1 & filter_token_black) >> 48;
        assert_eq!(inserted_possible_move_white2, extract_white_move_count_from_board(move_and_token_count_board2));
        assert_eq!(inserted_possible_move_black2, extract_black_move_count_from_board(move_and_token_count_board2));
        assert_eq!(inserted_token_count_white2, extract_white_token_count_from_board(move_and_token_count_board2) - 2);
        assert_eq!(inserted_token_count_black2, extract_black_token_count_from_board(move_and_token_count_board2) - 2);
    }

    #[test]
    fn test_is_move_valid() {
        // move phase
        assert_eq!(is_move_valid(7, 6, 0b11, 9), false);
        assert_eq!(is_move_valid(7, 0, 0b00, 9), true);
        assert_eq!(is_move_valid(8, 16, 0b00, 9), true);
        assert_eq!(is_move_valid(9, 1, 0b00, 9), false);

        // end phase
        assert_eq!(is_move_valid(4, 23, 0b00, 3), true);
        assert_eq!(is_move_valid(1, 5, 0b00, 3), true);
        assert_eq!(is_move_valid(4, 1, 0b11, 3), false);
    }

    #[test]
    fn test_is_neighbor() {
        let now = std::time::Instant::now();
        for _ in 0..10000 {
            assert_eq!(is_neighbor(0, 1), true);
            assert_eq!(is_neighbor(0, 7), true);
            assert_eq!(is_neighbor(0, 8), true);
            assert_eq!(is_neighbor(10, 2), true);
            assert_eq!(is_neighbor(6, 5), true);
            assert_eq!(is_neighbor(16, 17), true);
            assert_eq!(is_neighbor(22, 14), true);
            assert_eq!(is_neighbor(0, 2), false);
            assert_eq!(is_neighbor(0, 16), false);
            assert_eq!(is_neighbor(1, 9), false);
            assert_eq!(is_neighbor(22, 10), false);
            assert_eq!(is_neighbor(7, 8), false);
            assert_eq!(is_neighbor(23, 17), false);
            assert_eq!(is_neighbor(16, 0), false);
        }
        println!("Time elapsed: {:?}", now.elapsed());
    }

    #[test]
    fn test_is_part_of_mill() {
        let board = 0b111111101010101111000010001111110000001000100000;
        let now = std::time::Instant::now();
        
        for _ in 0..100000 {
            assert!(is_part_of_mill(board, 0, 0b11));
            assert!(is_part_of_mill(board, 7, 0b11));
            assert!(is_part_of_mill(board, 1, 0b11));
            assert!(is_part_of_mill(board, 13, 0b11));
            assert!(is_part_of_mill(board, 14, 0b11));
            assert!(is_part_of_mill(board, 15, 0b11));

            assert!(!is_part_of_mill(board, 2, 0b11));
            assert!(!is_part_of_mill(board, 8, 0b11));
            assert!(!is_part_of_mill(board, 4, 0b11));
            assert!(!is_part_of_mill(board, 5, 0b11));
            assert!(!is_part_of_mill(board, 6, 0b11));
            assert!(!is_part_of_mill(board, 9, 0b11));
        
            assert!(is_part_of_mill(board, 3, 0b10));
            assert!(is_part_of_mill(board, 4, 0b10));
            assert!(is_part_of_mill(board, 5, 0b10));

            assert!(!is_part_of_mill(board, 1, 0b10));
            assert!(!is_part_of_mill(board, 2, 0b10));
        }
        println!("Time elapsed: {:?}", now.elapsed());

        assert!(!is_part_of_mill(board, 8, 0b10));
        assert!(!is_part_of_mill(board, 9, 0b10));
        assert!(!is_part_of_mill(board, 10, 0b10));
        assert!(!is_part_of_mill(board, 19, 0b10));
        assert!(!is_part_of_mill(board, 21, 0b10));
    }

    #[test]
    fn test_is_closing_mill() {
        // normal mill closing
        let board_before = decode_positions("EWWEWEEEEEEEEWEEEBBBEEWE".to_string());
        let board_after = decode_positions("EWWWEEEEEEEEEWEEEBBBEEWE".to_string());
        // zwick mill closing
        let board_before2 = decode_positions("EWWWEEEEEWEWWEEEBBBEEWE".to_string());
        let board_after2 = decode_positions("EWEWEEEEEWWWWEEEBBBEEWE".to_string());
        // normal move
        let board_before3 = decode_positions("EWWWEEEEEWEWWEEEBBBEEWE".to_string());
        let board_after3 = decode_positions("EWWEWEEEEWEWWEEEBBBEEWE".to_string());

        assert!(is_mill_closing(board_before, board_after, 0b11));
        assert!(is_mill_closing(board_before2, board_after2, 0b11));
        assert!(!is_mill_closing(board_before3, board_after3, 0b11));
        assert!(is_mill_closing(reverse_token_of_board(board_before), reverse_token_of_board(board_after), 0b10));
        assert!(is_mill_closing(reverse_token_of_board(board_before2), reverse_token_of_board(board_after2), 0b10));
        assert!(!is_mill_closing(reverse_token_of_board(board_before3), reverse_token_of_board(board_after3), 0b10));
    }

    #[test]
    fn test_is_beat_possible() {
        let mut board = 0b111111111010101000000000000000000000000000000000;

        let now = std::time::Instant::now();
        for _ in 0..1000000 {
            assert!(is_beat_possible(board, 0, 0b10));
            assert!(!is_beat_possible(board, 1, 0b10));
            assert!(!is_beat_possible(board, 2, 0b10));
        }
        println!("Time elapsed: {:?}", now.elapsed());

        assert!(!is_beat_possible(board, 3, 0b10));
        assert!(!is_beat_possible(board, 4, 0b10));
        assert!(!is_beat_possible(board, 5, 0b10));
        assert!(!is_beat_possible(board, 6, 0b10));
        assert!(!is_beat_possible(board, 7, 0b10));

        assert!(!is_beat_possible(board, 0, 0b11));
        assert!(!is_beat_possible(board, 1, 0b11));
        assert!(!is_beat_possible(board, 2, 0b11));
        assert!(!is_beat_possible(board, 3, 0b11));
        assert!(is_beat_possible(board, 4, 0b11));
        assert!(!is_beat_possible(board, 5, 0b11));
        assert!(!is_beat_possible(board, 6, 0b11));
        assert!(!is_beat_possible(board, 7, 0b11));

        board = set_token_at(board, 7, 0b11);

        assert!(is_beat_possible(board, 0, 0b10));
        assert!(is_beat_possible(board, 1, 0b10));
        assert!(is_beat_possible(board, 2, 0b10));
        assert!(is_beat_possible(board, 3, 0b10));
        assert!(!is_beat_possible(board, 4, 0b10));
        assert!(!is_beat_possible(board, 5, 0b10));
        assert!(!is_beat_possible(board, 6, 0b10));
        assert!(is_beat_possible(board, 7, 0b10));

        assert!(!is_beat_possible(board, 0, 0b11));
        assert!(!is_beat_possible(board, 1, 0b11));
        assert!(!is_beat_possible(board, 2, 0b11));
        assert!(!is_beat_possible(board, 3, 0b11));
        assert!(is_beat_possible(board, 4, 0b11));
        assert!(is_beat_possible(board, 5, 0b11));
        assert!(is_beat_possible(board, 6, 0b11));
        assert!(!is_beat_possible(board, 7, 0b11));
    }
}
