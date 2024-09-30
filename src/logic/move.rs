use crate::logic::{action::{Action, Move}, game_state::Token, mill_detection::{is_all_part_of_mill, is_part_of_mill}, position::{get_token_at, negate_token, set_token_at}};

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

pub fn is_move_valid(start_position: usize, end_position: usize, end_token: u8, number_of_token_type: u8) -> bool {
    if end_token != 0b00 {
        return false
    }
    
    if number_of_token_type == 3 || is_neighbor(start_position, end_position) {
        return true
    }

    return false
}

fn is_neighbor(position1: usize, position2: usize) -> bool {
    NEIGHBORS[position1].contains(&position2)
}

pub fn apply_move(board: &u64, r#move: &Move, token_type: u8) -> u64 {
    let mut new_board = board.clone();
    if r#move.start_position.is_some() {
        new_board = set_token_at(*board, r#move.start_position.unwrap(), 0b00);
    }
    set_token_at(new_board, r#move.end_position, token_type)
}

pub fn apply_action(board: &u64, action: &Action, token_type: u8) -> u64 {
    let mut new_board = board.clone();
    if action.start_position.is_some() {
        new_board = set_token_at(*board, action.start_position.unwrap(), 0b00);
    }
    new_board = set_token_at(new_board, action.end_position, token_type);
    if action.beatable_position.is_some() {
        new_board = set_token_at(new_board, action.beatable_position.unwrap(), 0b00);
    }
    new_board
}

pub fn is_beat_possible(board: u64, position: usize, token_current_player: u8) -> bool {
    let token_of_opponent: u8 = negate_token(token_current_player);
    
    if get_token_at(board, position) != Token::parse_to_token(token_of_opponent) {
        return false
    }
    
    if !is_part_of_mill(board, position, token_of_opponent) {
        return true
    }
    
    return is_all_part_of_mill(board, token_of_opponent)
}

#[cfg(test)]
mod tests {
    use crate::logic::{action::{Action, Move}, game_state::Token, r#move::{apply_action, apply_move, is_beat_possible, is_move_valid, is_neighbor}, position::{decode_positions, set_token_at}};
    
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
    fn test_apply_move() {
        let board = decode_positions("WBWWWWBBBEEEEEEEEEEEEEEE".to_string());
        let expected_board = decode_positions("WBWWWWBBEBEEEEEEEEEEEEEE".to_string());

        assert_eq!(apply_move(&board, &Move::new(Some(8), 9), Token::parse_to_u8(Token::Black)), expected_board);
    }

    #[test]
    fn test_apply_action() {
        let board = decode_positions("WBWWWWBBBEEEEEEEEEEEEEEE".to_string());
        let expected_board = decode_positions("WBWWWWBBEBEEEEEEEEEEEEEE".to_string());

        assert_eq!(apply_action(&board, &Action::new(Some(8), 9, None), Token::parse_to_u8(Token::Black)), expected_board);

        let board2 = decode_positions("WBWWWEWBBEEEEEEEEEEEEEEE".to_string());
        let expected_board2 = decode_positions("WBWWWWEEBEEEEEEEEEEEEEEE".to_string());

        assert_eq!(apply_action(&board2, &Action::new(Some(6), 5, Some(7)), Token::parse_to_u8(Token::White)), expected_board2);
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
