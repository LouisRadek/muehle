use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    None,
    Black,
    White
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Token::None => write!(f, "□"),
           Token::Black => write!(f, "B"),
           Token::White => write!(f, "W"),
       }
    }
}

impl Token {
    pub fn to_char(token_type: Token) -> char {
        if token_type == Token::None {
            return 'E'
        } else if token_type == Token::White {
            return 'W'
        } else {
            return 'B'
        }
    }

    pub fn parse_to_char(encoded_token: char) -> Token {
        if encoded_token == 'E' {
            return Token::None
        } else if encoded_token == 'W' {
            return Token::White
        } else {
            return Token::Black
        }
    }
}

/**
 * The fields are odered in rings with the 0, 8, 16th Element in the top left of each ring
 * Visualisation:
 * 
 * 0            1               2
 * 
 *     8        9       10
 * 
 *         16  17  18
 * 
 * 7   15  23      19   11      3
 *          
 *         22  21  20   
 * 
 *     14      13       12
 * 6            5               4
 */
pub struct GameState {
    positions: [Token; 24],
    player_turn: u8,
    token_set_at_beginning: u8,
    win: bool
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            positions: [Token::None; 24],
            player_turn: 1,
            token_set_at_beginning: 18,
            win: false
        }
    }
}

impl GameState {
    pub fn get_positions(&self) -> [Token; 24] {
        self.positions
    }
    
    pub fn get_token_at_position(&self, position: usize) -> Token {
        return self.positions[position]
    }
    
    pub fn set_token_at_position(&mut self, position: usize, token: Token) {
        self.positions[position] = token;
    }

    pub fn get_player_turn(&self) -> u8 {
        self.player_turn
    }

    pub fn change_player(&mut self) {
        if self.player_turn == 1 {
            self.player_turn = 2
        } else {
            self.player_turn = 1
        }
    }

    pub fn get_token_set_at_beginning(&self) -> u8 {
        self.token_set_at_beginning
    }

    pub fn set_token_set_at_beginning(&mut self, new_value: u8) {
        self.token_set_at_beginning = new_value
    }

    pub fn decrement_token_set_at_beginning(&mut self) {
        if self.token_set_at_beginning > 0 {
            self.token_set_at_beginning -= 1;
        }
    }

    pub fn get_win(&self) -> bool {
        self.win
    }

    pub fn set_win(&mut self) {
        self.win = true
    }

    pub fn get_number_of_token(&self, token_type: Token) -> u8 {
        let mut number_of_token_type: u8 = 0;
        for token in self.positions {
            if token == token_type {
                number_of_token_type += 1
            }
        }

        return number_of_token_type
    }
}

#[cfg(test)]
pub mod tests {
    use crate::game_state::{GameState, Token};

    pub fn generate_example_positions() -> GameState {
        return GameState {
            positions: [
                Token::None, Token::Black, Token::Black, Token::None, Token::None, Token::None, Token::White, Token::White, 
                Token::White, Token::White, Token::Black, Token::White, Token::Black, Token::White, Token::None, Token::Black, 
                Token::Black, Token::None, Token::White, Token::Black, Token::White, Token::None, Token::Black, Token::None
            ], 
            player_turn: 1,
            token_set_at_beginning: 18,
            win: false
        }
    }

    #[test]
    fn test_game_state() {
        let game = GameState::default();

        assert_eq!(game.positions, [Token::None; 24]);
        assert_eq!(game.player_turn, 1);
        assert_eq!(game.token_set_at_beginning, 18)
    }

    #[test]
    fn test_get_positions() {
        let game = generate_example_positions();
        let positions = [
            Token::None, Token::Black, Token::Black, Token::None, Token::None, Token::None, Token::White, Token::White, 
            Token::White, Token::White, Token::Black, Token::White, Token::Black, Token::White, Token::None, Token::Black, 
            Token::Black, Token::None, Token::White, Token::Black, Token::White, Token::None, Token::Black, Token::None
        ];
        assert_eq!(positions, game.get_positions());
    }

    #[test]
    fn test_get_token_at_position() {
        let game = generate_example_positions();

        assert_eq!(game.get_token_at_position(0), Token::None);
        assert_eq!(game.get_token_at_position(1), Token::Black);
        assert_eq!(game.get_token_at_position(7), Token::White);
        assert_eq!(game.get_token_at_position(8), Token::White);
    }

    #[test]
    fn test_set_token_at_position() {
        let mut game = generate_example_positions();

        game.set_token_at_position(0, Token::Black);
        assert_eq!(game.get_token_at_position(0), Token::Black);

        game.set_token_at_position(7, Token::None);
        assert_eq!(game.get_token_at_position(7), Token::None);

        game.set_token_at_position(10, Token::White);
        assert_eq!(game.get_token_at_position(10), Token::White);
    }

    #[test]
    fn test_get_player_turn() {
        let game = generate_example_positions();
        assert_eq!(game.player_turn, game.get_player_turn());
    }

    #[test]
    fn test_change_player() {
        let mut game = generate_example_positions();
        game.change_player();
        assert_eq!(game.player_turn, 2);
        game.change_player();
        assert_eq!(game.player_turn, 1);
    }

    #[test]
    fn test_get_token_set_at_beginning() {
        let game = generate_example_positions();
        assert_eq!(game.token_set_at_beginning, 18);
    }

    #[test]
    fn test_set_token_set_at_beginning() {
        let mut game = generate_example_positions();
        game.set_token_set_at_beginning(5);
        assert_eq!(game.token_set_at_beginning, 5);
    }

    #[test]
    fn test_get_set_win() {
        let mut game = generate_example_positions();
        assert!(!game.get_win());
        game.set_win();
        assert!(game.get_win());
    }

    #[test]
    fn test_get_number_of_token() {
        let game = generate_example_positions();
        assert_eq!(game.get_number_of_token(Token::Black), 8);
        assert_eq!(game.get_number_of_token(Token::White), 8);
        assert_eq!(game.get_number_of_token(Token::None), 8);

        let game2 = GameState::default();
        assert_eq!(game2.get_number_of_token(Token::Black), 0);
        assert_eq!(game2.get_number_of_token(Token::None), 24);
    }
}
