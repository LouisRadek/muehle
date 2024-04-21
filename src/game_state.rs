use core::panic;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    None,
    Black,
    White
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
struct GameState {
    positions: [Token; 24],
    player_turn: u8,
    token_set_at_beginning: u8
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            positions: [Token::None; 24],
            player_turn: 1,
            token_set_at_beginning: 18
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

    fn get_number_of_token(&self, token_type: Token) -> u8 {
        let mut number_of_token_type: u8 = 0;
        for token in self.positions {
            if token == token_type {
                number_of_token_type += 1
            }
        }

        return number_of_token_type
    }

    fn move_to(&mut self, start_position: Option<usize>, end_position: usize) {
        if !self.is_move_valid(start_position, end_position) {
            panic!("not a valid move")
        }

        let token = if self.player_turn == 1 {
            Token::White   
        } else {
            Token::Black
        };

        if !start_position.is_none() {
            self.set_token_at_position(start_position.unwrap(), Token::None);
        }
        self.set_token_at_position(end_position, token)
    }
    
    fn is_move_valid(&self, start_position: Option<usize>, end_position: usize) -> bool {
        if start_position.is_none() {
            if self.token_set_at_beginning > 0 && self.get_token_at_position(end_position) == Token::None {
                return true
            }

            return false
        }

        if self.get_token_at_position(end_position) == Token::None {
            if self.player_turn == 1 && self.get_token_at_position(start_position.unwrap()) == Token::White {
                if self.get_number_of_token(Token::White) <= 3 {
                    return true
                }
                
                if GameState::is_neighbor(start_position.unwrap(), end_position) {
                    return true
                }
            }

            if self.player_turn == 2 && self.get_token_at_position(start_position.unwrap()) == Token::Black {
                if self.get_number_of_token(Token::Black) <= 3 {
                    return true
                }
                
                if GameState::is_neighbor(start_position.unwrap(), end_position) {
                    return true
                }
            }
        }

        return false
    }

    fn is_neighbor(start_position: usize, end_position: usize) -> bool {
        if [0, 7, 8, 15, 16, 23].contains(&start_position) && start_position.abs_diff(end_position) == 7 {
            return true
        }

        if start_position % 2 == 1 && start_position.abs_diff(end_position) == 8 {
            return true
        }

        if start_position.abs_diff(end_position) == 1 {
            return true
        }

        return false
    }
}


#[cfg(test)]
mod tests {
    use crate::game_state::{GameState, Token};

    fn generate_example_positions() -> GameState {
        return GameState {
            positions: [
                Token::None, Token::Black, Token::Black, Token::None, Token::None, Token::None, Token::White, Token::White, 
                Token::White, Token::White, Token::Black, Token::White, Token::Black, Token::White, Token::None, Token::Black, 
                Token::Black, Token::None, Token::White, Token::Black, Token::White, Token::None, Token::Black, Token::None
            ], 
            player_turn: 1,
            token_set_at_beginning: 18
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
    fn test_get_number_of_token() {
        let game = generate_example_positions();
        assert_eq!(game.get_number_of_token(Token::Black), 8);
        assert_eq!(game.get_number_of_token(Token::White), 8);
        assert_eq!(game.get_number_of_token(Token::None), 8);

        let game2 = GameState::default();
        assert_eq!(game2.get_number_of_token(Token::Black), 0);
        assert_eq!(game2.get_number_of_token(Token::None), 24);
    }

    #[test]
    fn test_move_to() {
        // set phase
        let mut game = GameState::default();
        
        assert_eq!(game.get_token_at_position(0), Token::None);
        game.move_to(None, 0);
        assert_eq!(game.get_token_at_position(0), Token::White);

        game.player_turn = 2;
        assert_eq!(game.get_token_at_position(16), Token::None);
        game.move_to(None, 16);
        assert_eq!(game.get_token_at_position(16), Token::Black);


        // move phase
        let mut game2 = generate_example_positions();
        game2.token_set_at_beginning = 0;

        assert_eq!(game2.get_token_at_position(7), Token::White);
        assert_eq!(game2.get_token_at_position(0), Token::None);
        game2.move_to(Some(7), 0);
        assert_eq!(game2.get_token_at_position(7), Token::None);
        assert_eq!(game2.get_token_at_position(0), Token::White);

        game2.player_turn = 2;
        assert_eq!(game2.get_token_at_position(2), Token::Black);
        assert_eq!(game2.get_token_at_position(3), Token::None);
        game2.move_to(Some(2), 3);
        assert_eq!(game2.get_token_at_position(2), Token::None);
        assert_eq!(game2.get_token_at_position(3), Token::Black);

        game2.player_turn = 1;
        assert_eq!(game2.get_token_at_position(13), Token::White);
        assert_eq!(game2.get_token_at_position(21), Token::None);
        game2.move_to(Some(13), 21);
        assert_eq!(game2.get_token_at_position(13), Token::None);
        assert_eq!(game2.get_token_at_position(21), Token::White);

        // endphase
        let mut game3 = GameState::default();
        game3.token_set_at_beginning = 0;
        game3.set_token_at_position(0, Token::White);
        game3.set_token_at_position(4, Token::White);
        game3.set_token_at_position(12, Token::White);

        assert_eq!(game3.get_token_at_position(0), Token::White);
        assert_eq!(game3.get_token_at_position(3), Token::None);
        game3.move_to(Some(0), 3);
        assert_eq!(game3.get_token_at_position(0), Token::None);
        assert_eq!(game3.get_token_at_position(3), Token::White);
    }

    #[test]
    #[should_panic]
    fn test_move_to_panic() {
        let mut game = generate_example_positions();
        game.token_set_at_beginning = 0;

        // without start_position not in the set phase
        game.move_to(None, 0);
    }

    #[test]
    #[should_panic]
    fn test_move_to_panic2() {
        let mut game = generate_example_positions();
        game.token_set_at_beginning = 0;

        // try to move black token as player 1
        game.move_to(Some(16), 17);
    }
    
    #[test]
    #[should_panic]
    fn test_move_to_panic3() {
        let mut game = generate_example_positions();
        game.token_set_at_beginning = 0;

        // try to move a token on a not empty field
        game.move_to(Some(18), 19);
    }

    #[test]
    #[should_panic]
    fn test_move_to_panic4() {
        let mut game = generate_example_positions();
        game.token_set_at_beginning = 0;

        // try to move to a not neighbor field
        game.move_to(Some(6), 14);
    }

    #[test]
    #[should_panic]
    fn test_move_to_panic5() {
        let mut game = generate_example_positions();
        game.token_set_at_beginning = 0;

        // try to move from an empty field
        game.move_to(Some(5), 4);
    }

    #[test]
    fn test_is_move_valid() {
        // set phase
        let mut game = GameState::default();

        assert_eq!(game.get_token_at_position(0), Token::None);
        assert_eq!(game.is_move_valid(None, 0), true);
        game.set_token_at_position(0, Token::Black);
        assert_eq!(game.is_move_valid(None, 0), false);
        assert_eq!(game.is_move_valid(Some(0), 2), false);

        // move phase
        let mut game2 = generate_example_positions();
        game2.token_set_at_beginning = 0;

        assert_eq!(game2.is_move_valid(Some(7), 6), false);
        assert_eq!(game2.is_move_valid(Some(1), 0), false);
        assert_eq!(game2.is_move_valid(Some(7), 0), true);
        assert_eq!(game2.is_move_valid(Some(9), 17), true);
        assert_eq!(game2.is_move_valid(Some(8), 0), false);

        // end phase
        let mut game3 = GameState::default();
        game3.token_set_at_beginning = 0;
        game3.set_token_at_position(0, Token::White);
        game3.set_token_at_position(1, Token::White);
        game3.set_token_at_position(4, Token::White);

        assert_eq!(game3.is_move_valid(Some(4), 7), true);
        assert_eq!(game3.is_move_valid(Some(1), 5), true);
        assert_eq!(game3.is_move_valid(Some(4), 1), false);
    }

    #[test]
    fn test_is_neighbor() {
        assert_eq!(GameState::is_neighbor(0, 7), true);
        assert_eq!(GameState::is_neighbor(0, 7), true);
        assert_eq!(GameState::is_neighbor(0, 7), true);
        assert_eq!(GameState::is_neighbor(1, 2), true);
        assert_eq!(GameState::is_neighbor(13, 12), true);
        assert_eq!(GameState::is_neighbor(19, 11), true);
        assert_eq!(GameState::is_neighbor(9, 17), true);
        assert_eq!(GameState::is_neighbor(5, 13), true);

        assert_eq!(GameState::is_neighbor(0, 8), false);
        assert_eq!(GameState::is_neighbor(2, 6), false);
        assert_eq!(GameState::is_neighbor(22, 13), false);
        assert_eq!(GameState::is_neighbor(23, 19), false);
        assert_eq!(GameState::is_neighbor(2, 4), false);
        assert_eq!(GameState::is_neighbor(1, 17), false);
        assert_eq!(GameState::is_neighbor(21, 5), false);
    }
}
