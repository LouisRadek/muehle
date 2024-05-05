use crate::game_state::Token;

pub fn get_number_of_token(positions: [Token; 24], token_type: Token) -> u8 {
    let mut number_of_token_type: u8 = 0;
    for token in positions {
        if token == token_type {
            number_of_token_type += 1
        }
    }

    return number_of_token_type
}

#[cfg(test)]
mod tests {
    use crate::{agent::utils::get_number_of_token, game_state::{tests::generate_example_positions, GameState, Token}};

    #[test]
    fn test_get_number_of_token() {
        let game = generate_example_positions();
        assert_eq!(get_number_of_token(game.get_positions(), Token::Black), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::White), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::None), 8);

        let game2 = GameState::default();
        assert_eq!(get_number_of_token(game2.get_positions(), Token::Black), 0);
        assert_eq!(get_number_of_token(game2.get_positions(), Token::None), 24);
    }
}
