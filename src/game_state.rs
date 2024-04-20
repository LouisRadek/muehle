#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    None,
    Black,
    White
}

struct GameState {
    positions: [Token; 24],
    player_turn: u8,
    token_set_beginning: u8
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            positions: [Token::None; 24],
            player_turn: 1,
            token_set_beginning: 18
        }
    }
}

impl GameState {
    fn get_token_at_position(&self, position: usize) -> Token {
        return self.positions[position]
    }

    fn set_token_at_position(&mut self, position: usize, token: Token) {
        self.positions[position] = token;
    }
}


#[cfg(test)]
mod tests {
    use crate::game_state::{GameState, Token};

    fn generate_example_positions() -> GameState {
        return GameState {
            positions: [
                Token::White, Token::Black, Token::None, 
                Token::White, Token::Black, Token::None,
                Token::White, Token::Black, Token::None, 
                Token::White, Token::Black, Token::None, Token::White, Token::Black, Token::None,
                Token::White, Token::Black, Token::None,
                Token::White, Token::Black, Token::None,
                Token::White, Token::Black, Token::None
            ], 
            player_turn: 1,
            token_set_beginning: 18
        }
    }

    #[test]
    fn test_game_state() {
        let game = GameState::default();

        assert_eq!(game.positions, [Token::None; 24]);
        assert_eq!(game.player_turn, 1);
        assert_eq!(game.token_set_beginning, 18)
    }

    #[test]
    fn test_get_token_at_position() {
        let game = generate_example_positions();

        assert_eq!(game.get_token_at_position(0), Token::White);
        assert_eq!(game.get_token_at_position(1), Token::Black);
        assert_eq!(game.get_token_at_position(7), Token::Black);
        assert_eq!(game.get_token_at_position(8), Token::None);
    }

    #[test]
    fn test_set_token_at_position() {
        let mut game = generate_example_positions();

        game.set_token_at_position(0, Token::Black);
        assert_eq!(game.get_token_at_position(0), Token::Black);

        game.set_token_at_position(7, Token::None);
        assert_eq!(game.get_token_at_position(7), Token::None);

        game.set_token_at_position(8, Token::White);
        assert_eq!(game.get_token_at_position(8), Token::White);
    }
}
