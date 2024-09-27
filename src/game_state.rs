use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    None,
    Black,
    White
}

#[derive(Debug, PartialEq)]
pub enum Phase {
    Set,
    Move
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
    pub fn parse_to_token(encoded_token: u8) -> Token {
        match encoded_token {
            0b11 => Token::White,
            0b10 => Token::Black,
            _ => Token::None
        }
    }

    pub fn parse_to_u8(token: Token) -> u8 {
        match token {
            Token::White => 0b11,
            Token::Black => 0b10,
            Token::None => 0b00
        }
    }

    pub fn negate(&mut self) -> Token {
        match self {
            Token::Black => Token::White,
            Token::White => Token::Black,
            Token::None => Token::None
        }
    }
}

/*
    The gameboard is stored as 48 bits in a u64, where each 2 bits represent a position on the board.
    The position are ordered in the u64 as follows:
        7            0               1

            15       8       9

                23  16  17

        6   14  22      18   10      2
                
                21  20  19   

            13      12       11
        5            4               3
*/
#[derive(Clone, Copy)]
pub struct GameState {
    board: u64,
    player_turn: Token,
    token_set_at_beginning: u8
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            board: 0b0,
            player_turn: Token::White,
            token_set_at_beginning: 18
        }
    }
}

impl GameState {
    pub fn get_board(&self) -> u64 {
        self.board
    }

    pub fn set_board(&mut self, new_board: u64) {
        self.board = new_board;
    }

    pub fn get_player_turn(&self) -> Token {
        self.player_turn
    }

    pub fn change_player(&mut self) {
        if self.player_turn == Token::White {
            self.player_turn = Token::Black
        } else {
            self.player_turn = Token::White
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

    pub fn get_phase(&self) -> Phase {
        if self.get_token_set_at_beginning() > 0 {
            return Phase::Set
        } else {
            return Phase::Move
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{game_state::{GameState, Phase, Token}, position::decode_positions};

    #[test]
    fn test_game_state() {
        let game = GameState::default();

        assert_eq!(game.board, 0b0);
        assert_eq!(game.player_turn, Token::White);
        assert_eq!(game.token_set_at_beginning, 18)
    }

    #[test]
    fn test_get_board() {
        let mut game = GameState::default();
        assert_eq!(game.get_board(), 0b0);

        let new_board = decode_positions("BEEEEEEEEEEEEEEEEEEEEEEE".to_string());
        game.set_board(new_board);
        assert_eq!(game.get_board(), new_board);
    }

    #[test]
    fn test_set_board() {
        let mut game = GameState::default();
        assert_eq!(game.get_board(), 0b0);

        let new_board = decode_positions("BEEEEEEEEEEEEEEEEEEEEEEE".to_string());
        game.set_board(new_board);
        assert_eq!(game.get_board(), new_board);
    }

    #[test]
    fn test_get_player_turn() {
        let game = GameState::default();
        assert_eq!(game.player_turn, game.get_player_turn());
    }

    #[test]
    fn test_change_player() {
        let mut game = GameState::default();
        game.change_player();
        assert_eq!(game.player_turn, Token::Black);
        game.change_player();
        assert_eq!(game.player_turn, Token::White);
    }

    #[test]
    fn test_get_token_set_at_beginning() {
        let game = GameState::default();
        assert_eq!(game.token_set_at_beginning, 18);
    }

    #[test]
    fn test_set_token_set_at_beginning() {
        let mut game = GameState::default();
        game.set_token_set_at_beginning(5);
        assert_eq!(game.token_set_at_beginning, 5);
    }

    #[test]
    fn test_get_phase() {
        let mut game = GameState::default();
        assert_eq!(game.get_phase(), Phase::Set);
        game.set_token_set_at_beginning(0);
        assert_eq!(game.get_phase(), Phase::Move);
    }
}
