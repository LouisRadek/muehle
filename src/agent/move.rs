use crate::game_state::Token;

use super::utils::get_number_of_token;

pub fn is_move_valid(start_position: usize, end_position: usize, end_token: Token, number_of_token_type: u8) -> bool {
    if end_token != Token::None {
        return false
    }
    
    if number_of_token_type == 3 {
        return true
    }

    if is_neighbor(start_position, end_position) {
        return true
    }

    return false
}

pub fn is_neighbor(start_position: usize, end_position: usize) -> bool {
    if [0, 7].contains(&start_position) && [0, 7].contains(&end_position) {
        return true
    }

    if [8, 15].contains(&start_position) && [8, 15].contains(&end_position) {
        return true
    }

    if [16, 23].contains(&start_position) && [16, 23].contains(&end_position) {
        return true
    }

    if [7, 8].contains(&start_position) && [7, 8].contains(&end_position) {
        return false
    }

    if [15, 16].contains(&start_position) && [15, 16].contains(&end_position) {
        return false
    }

    if start_position % 2 == 1 && start_position.abs_diff(end_position) == 8 {
        return true
    }

    if start_position.abs_diff(end_position) == 1 {
        return true
    }

    return false
}

#[cfg(test)]
mod tests {
    use crate::{agent::r#move::{is_move_valid, is_neighbor}, game_state::{tests::generate_example_positions, GameState, Token}};

    #[test]
    fn test_is_move_valid() {
        // move phase
        let mut game2 = generate_example_positions();
        game2.set_token_set_at_beginning(0);

        assert_eq!(is_move_valid(7, 6, Token::White, 9), false);
        assert_eq!(is_move_valid(7, 0, Token::None, 9), true);
        assert_eq!(is_move_valid(9, 17, Token::None, 9), true);
        assert_eq!(is_move_valid(8, 0, Token::None, 9), false);

        // end phase
        let mut game3 = GameState::default();
        game3.set_token_set_at_beginning(0);
        game3.set_token_at_position(0, Token::White);
        game3.set_token_at_position(1, Token::White);
        game3.set_token_at_position(4, Token::White);

        assert_eq!(is_move_valid(4, 7, Token::None, 3), true);
        assert_eq!(is_move_valid(1, 5, Token::None, 3), true);
        assert_eq!(is_move_valid(4, 1, Token::White, 3), false);
    }

    #[test]
    fn test_is_neighbor() {
        assert_eq!(is_neighbor(0, 7), true);
        assert_eq!(is_neighbor(0, 7), true);
        assert_eq!(is_neighbor(0, 7), true);
        assert_eq!(is_neighbor(1, 2), true);
        assert_eq!(is_neighbor(13, 12), true);
        assert_eq!(is_neighbor(19, 11), true);
        assert_eq!(is_neighbor(9, 17), true);
        assert_eq!(is_neighbor(5, 13), true);

        assert_eq!(is_neighbor(0, 8), false);
        assert_eq!(is_neighbor(2, 6), false);
        assert_eq!(is_neighbor(22, 13), false);
        assert_eq!(is_neighbor(23, 19), false);
        assert_eq!(is_neighbor(2, 4), false);
        assert_eq!(is_neighbor(1, 17), false);
        assert_eq!(is_neighbor(21, 5), false);
    }
}
