use crate::logic::position::reverse_token_of_board;

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

pub fn is_part_of_mill(board: u64, position: usize, token_type: u8) -> bool {
    let (index1, index2) = MILL_INDICES_FOR_POSITION[position];
    let possible_mill_position = POSSIBLE_MILLS_WHITE[index1];
    let possible_mill_position2 = POSSIBLE_MILLS_WHITE[index2];
    if token_type == 0b11 {
        (board & possible_mill_position) == POSSIBLE_MILLS_WHITE[index1]
        || (board & possible_mill_position2) == POSSIBLE_MILLS_WHITE[index2]
    } else if token_type == 0b10 {
        (board & possible_mill_position) == reverse_token_of_board(POSSIBLE_MILLS_WHITE[index1]) 
            || (board & possible_mill_position2) == reverse_token_of_board(POSSIBLE_MILLS_WHITE[index2]) 
    } else {
        false
    }
}

pub fn is_mill_closing(pos_before: u64, pos_after: u64, token_type: u8) -> bool {
    (0..16).any(|i| {
        let possible_mill = if token_type == 0b11 {
            POSSIBLE_MILLS_WHITE[i]
        } else {
            reverse_token_of_board(POSSIBLE_MILLS_WHITE[i])
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

#[cfg(test)]
mod tests {
    use crate::logic::{mill_detection::{is_all_part_of_mill, is_mill_closing, is_part_of_mill}, position::{decode_positions, reverse_token_of_board}};

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
    fn test_is_all_part_of_mill() {
        let board = decode_positions("WWEEEBBBWEEEEBBBWEEEEEEE".to_string());
        assert!(is_all_part_of_mill(board, 0b10));
        assert!(!is_all_part_of_mill(board, 0b11));
    }
}
