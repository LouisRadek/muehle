use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    None,
    Black,
    White
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Set,
    Move
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Token::Black => write!(f, "Black"),
           Token::White => write!(f, "White"),
           Token::None => write!(f, "None")
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
    step_counter: u8
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            board: 0b0,
            player_turn: Token::White,
            step_counter: 0
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

    pub fn get_step_counter(&self) -> u8 {
        self.step_counter
    }

    pub fn increase_step_counter(&mut self) {
        self.step_counter += 1;
    }

    pub fn get_phase(&self) -> Phase {
        if self.step_counter < 18 {
            Phase::Set
        } else {
            Phase::Move
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::{game_state::{GameState, Phase, Token}, position::decode_positions};

    #[test]
    fn test_game_state() {
        let game = GameState::default();

        assert_eq!(game.board, 0b0);
        assert_eq!(game.player_turn, Token::White);
        assert_eq!(game.step_counter, 0)
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
    fn test_get_step_counter() {
        let game = GameState::default();
        assert_eq!(game.get_step_counter(), 0);
    }

    #[test]
    fn test_increase_step_counter() {
        let mut game = GameState::default();
        let step_counter_before = game.get_step_counter();
        game.increase_step_counter();
        assert_eq!(game.get_step_counter(), step_counter_before + 1);
    }

    #[test]
    fn test_get_phase() {
        let mut game = GameState::default();
        assert_eq!(game.get_phase(), Phase::Set);
        game.step_counter = 18;
        assert_eq!(game.get_phase(), Phase::Move);
    }
}
