use muehle::{game_state::Token, r#move::is_neighbor};

pub fn get_number_of_token(positions: [Token; 24], token_type: Token) -> u8 {
    let mut number_of_token_type: u8 = 0;
    for token in positions {
        if token == token_type {
            number_of_token_type += 1
        }
    }

    return number_of_token_type
}

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

#[cfg(test)]
mod tests {
    use muehle::game_state::{GameState, Token};
    use crate::utils::{get_number_of_token, is_move_valid};

    #[test]
    fn test_get_number_of_token() {
        let game = GameState::generate_example_positions();
        assert_eq!(get_number_of_token(game.get_positions(), Token::Black), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::White), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::None), 8);

        let game2 = GameState::default();
        assert_eq!(get_number_of_token(game2.get_positions(), Token::Black), 0);
        assert_eq!(get_number_of_token(game2.get_positions(), Token::None), 24);
    }

    #[test]
    fn test_is_move_valid() {
        // move phase
        let mut game2 = GameState::generate_example_positions();
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
}
