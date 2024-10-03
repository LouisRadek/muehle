use core::panic;
use crate::logic::{r#move::NEIGHBORS, position::{get_number_of_tokens, get_token_at}};

/*
    One Board u64 looks like:
        5 bits with possible moves for white
        5 bits with possible moves for black
        3 bits with number of white tokens
        3 bits with number of black tokens
        48 bits with the board itself
    
    Furthermore, the counter of the tokens is offsetted to
    fit the 3 bits. Because a player can have max. 9 Tokens
    you ignore the 0 and 1 tokens and start counting from 2.
    000 -> 2
    001 -> 3
    ...
    111 -> 9
*/
pub const WHITE_TOKEN_FIRST_POSITION: u64 =          0b0000000000001000000000000000000000000000000000000000000000000000;
pub const BLACK_TOKEN_FIRST_POSITION: u64 =          0b0000000000000001000000000000000000000000000000000000000000000000;
pub const WHITE_POSSIBLE_MOVES_FIRST_POSITION: u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
pub const BLACK_POSSIBLE_MOVES_FIRST_POSITION: u64 = 0b0000000001000000000000000000000000000000000000000000000000000000;

pub fn insert_token_count_to_board(board: u64) -> u64 {
    let white_token_count: u64 = match get_number_of_tokens(board, 0b11)
    {
        0 => 0,
        1 => 0,
        val => val as u64 - 2
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

pub fn update_possible_move_count(board: u64, token_type: u8, position: usize, remove: bool) -> u64 {
    let mut new_board = board.clone();

    let token_at_position = get_token_at(new_board, position);
    if token_type == 0b00 || (token_at_position != 0b00 && remove) 
        || (token_at_position == 0b00 && !remove) 
        || (token_at_position != token_type && !remove) {
        panic!("Invalid token type or position")
    }

    NEIGHBORS[position].iter()
        .for_each(|neighbor| {
            if *neighbor == 24 {
                return ()
            }

            match get_token_at(new_board, *neighbor) {
                0b00 if remove && token_type == 0b11 => new_board -= WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                0b00 if remove && token_type == 0b10 => new_board -= BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                0b00 if !remove && token_type == 0b11 => new_board += WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                0b00 if !remove && token_type == 0b10 => new_board += BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                0b11 if remove => new_board += WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                0b11 if !remove => new_board -= WHITE_POSSIBLE_MOVES_FIRST_POSITION,
                0b10 if remove => new_board += BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                0b10 if !remove => new_board -= BLACK_POSSIBLE_MOVES_FIRST_POSITION,
                _ => ()
            }
    });

    return new_board
}

#[cfg(test)]
mod tests {
    use crate::{agent::utils::{extract_black_move_count_from_board, extract_black_token_count_from_board, extract_white_move_count_from_board, extract_white_token_count_from_board, get_possible_move_count, insert_number_of_possible_moves_to_board, insert_token_count_to_board, update_possible_move_count}, logic::position::{decode_positions, set_token_at}};
    use super::{BLACK_POSSIBLE_MOVES_FIRST_POSITION, WHITE_POSSIBLE_MOVES_FIRST_POSITION};

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
    fn test_get_possible_move_count() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());
        let board2 = decode_positions("BWEEEWBBWBBWWBBWEEEEEEWE".to_string());
        let board3 = decode_positions("EEEEEEEEEEEEEEEEEEEEEEEE".to_string());

        assert_eq!(get_possible_move_count(board, 0b11), 8);
        assert_eq!(get_possible_move_count(board, 0b10), 2);
        assert_eq!(get_possible_move_count(board2, 0b11), 7);
        assert_eq!(get_possible_move_count(board2, 0b10), 2);
        assert_eq!(get_possible_move_count(board3, 0b11), 0);
        assert_eq!(get_possible_move_count(board3, 0b10), 0);
    }

    #[test]
    fn test_update_possible_move_count() {
        let board = 
            decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string())
            + 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION
            + 8 * WHITE_POSSIBLE_MOVES_FIRST_POSITION;
        
        let mut new_board = set_token_at(board, 18, 0b11);
        assert_eq!(update_possible_move_count(new_board, 0b11, 18, false), new_board + 3 * WHITE_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 18, 0b10);
        assert_eq!(update_possible_move_count(new_board, 0b10, 18, false), new_board + 3 * BLACK_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 12, 0b10);
        assert_eq!(update_possible_move_count(new_board, 0b10, 12, false), new_board + 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 15, 0b11);
        assert_eq!(update_possible_move_count(new_board, 0b11, 15, false), new_board - 2 * WHITE_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 15, 0b10);
        assert_eq!(update_possible_move_count(new_board, 0b10, 15, false), new_board - 2 * WHITE_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 10, 0b10);
        assert_eq!(update_possible_move_count(new_board, 0b10, 10, false), 
            new_board 
            + 3 * BLACK_POSSIBLE_MOVES_FIRST_POSITION
            - WHITE_POSSIBLE_MOVES_FIRST_POSITION
        );
        new_board = set_token_at(board, 8, 0b00);
        assert_eq!(update_possible_move_count(new_board, 0b11, 8, true), new_board + 2 * WHITE_POSSIBLE_MOVES_FIRST_POSITION);
        new_board = set_token_at(board, 14, 0b00);
        assert_eq!(update_possible_move_count(new_board, 0b11, 14, true), 
            new_board
            - 3 * WHITE_POSSIBLE_MOVES_FIRST_POSITION
            + BLACK_POSSIBLE_MOVES_FIRST_POSITION
        );
        new_board = set_token_at(board, 6, 0b00);
        assert_eq!(update_possible_move_count(new_board, 0b10, 6, true), 
            new_board
            + 2 * BLACK_POSSIBLE_MOVES_FIRST_POSITION
            + WHITE_POSSIBLE_MOVES_FIRST_POSITION
        );
    }

    #[test]
    #[should_panic]
    fn test_update_possible_move_count2() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());

        assert_eq!(update_possible_move_count(board, 0b00, 18, false), board);
    }

    #[test]
    #[should_panic]
    fn test_update_possible_move_count3() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());

        assert_eq!(update_possible_move_count(board, 0b11, 18, false), board);
    }

    #[test]
    #[should_panic]
    fn test_update_possible_move_count4() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());

        assert_eq!(update_possible_move_count(board, 0b10, 7, true), board);
    }

    #[test]
    #[should_panic]
    fn test_update_possible_move_count5() {
        let board = decode_positions("WEEEBBBBWWEEEEWEWEEEEEEE".to_string());

        assert_eq!(update_possible_move_count(board, 0b10, 8, false), board);
    }
}
