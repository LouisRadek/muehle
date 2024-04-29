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
    pub fn get_token_set_at_beginning(&self) -> u8 {
        self.token_set_at_beginning
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

    pub fn get_win(&self) -> bool {
        self.win
    }

    pub fn get_positions(&self) -> [Token; 24] {
        self.positions
    }

    pub fn get_token_at_position(&self, position: usize) -> Token {
        return self.positions[position]
    }

    pub fn set_token_at_position(&mut self, position: usize, token: Token) {
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

    /**
     * Makes a move if it is valid and returns if a new mill emerged
     */
    pub fn move_to(&mut self, start_position: Option<usize>, end_position: usize) -> bool {
        let token = if self.player_turn == 1 {
            Token::White   
        } else {
            Token::Black
        };

        if start_position.is_none() {
            self.token_set_at_beginning -= 1;
            self.set_token_at_position(end_position, token);
            
            return self.search_for_mill(end_position, token)
        }

        self.set_token_at_position(start_position.unwrap(), Token::None);
        self.set_token_at_position(end_position, token);

        let is_token_in_mill_before = self.search_for_mill(start_position.unwrap(), token);
        let is_token_in_mill_after = self.search_for_mill(end_position, token);

        if !is_token_in_mill_before && is_token_in_mill_after {
            return true
        }

        return false
    }
    
    pub fn is_move_valid(&self, start_position: Option<usize>, end_position: usize) -> bool {
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

    fn search_for_mill(&self, position: usize, token_type: Token) -> bool {
        if self.positions[position] != token_type {
            return false
        }

        if position % 2 == 1 {
            if [7, 15, 23].contains(&position)
               && self.positions[position - 1] == token_type && self.positions[position - 7] == token_type {
                return true
            }
            
            if self.positions[position - 1] == token_type && self.positions[position + 1] == token_type {
                return true
            }

            return self.search_for_vertical_mill(position as isize, token_type)
        } else {
            return self.search_for_horizontal_mill(position, token_type)
        }
    }

    fn search_for_vertical_mill(&self, position: isize, token_type: Token) -> bool {
        let distance_between_rings = 8;
        let max_position = 23;

        if position + distance_between_rings <= max_position
           && position + distance_between_rings * 2 <= max_position 
           && self.positions[(position + distance_between_rings) as usize] == token_type
           && self.positions[(position + distance_between_rings * 2) as usize] == token_type {
            return true
        }

        if position + distance_between_rings <= max_position
           && position - distance_between_rings > 0 
           && self.positions[(position + distance_between_rings) as usize] == token_type
           && self.positions[(position - distance_between_rings) as usize] == token_type {
            return true
        }

        if position - distance_between_rings > 0
           && position - distance_between_rings * 2 > 0 
           && self.positions[(position - distance_between_rings) as usize] == token_type
           && self.positions[(position - distance_between_rings * 2) as usize] == token_type {
            return true
        }

        return  false
    }

    fn search_for_horizontal_mill(&self, position: usize, token_type: Token) -> bool {
        if [0, 8, 16].contains(&position) {
            if self.positions[position + 7] == token_type && self.positions[position + 6] == token_type {
                return true
            }

            if self.positions[position + 1] == token_type && self.positions[position + 2] == token_type {
                return true
            }
        } else if [6, 14, 22].contains(&position) {
            if self.positions[position - 1] == token_type && self.positions[position - 2] == token_type {
                return true
            }

            if self.positions[position + 1] == token_type && self.positions[position - 6] == token_type {
                return true
            }
        } else {
            if self.positions[position - 1] == token_type && self.positions[position - 2] == token_type {
                return true
            }

            if self.positions[position + 1] == token_type && self.positions[position + 2] == token_type {
                return true
            }
        }

        return false
    }

    pub fn is_beat_possible(&self, position: usize) -> bool {
        let token_of_oponent = if self.player_turn == 2 {
            Token::White   
        } else {
            Token::Black
        };

        if self.positions[position] != token_of_oponent {
            return false
        }

        if !self.search_for_mill(position, token_of_oponent) {
            return true
        }
        
        let mut has_oponent_only_token_in_mill = true;
        for (index, token) in self.positions.iter().enumerate() {
            if !self.search_for_mill(index, token_of_oponent) && *token == token_of_oponent {
                has_oponent_only_token_in_mill = false;
            }
        }

        return has_oponent_only_token_in_mill
    }

    pub fn beat_token(&mut self, position: usize) {
        self.set_token_at_position(position, Token::None);

        if self.token_set_at_beginning > 0 {
            return
        }

        if self.get_number_of_token(Token::White) < 3 {
            self.win = true;
            println!("Player 2 has won!")
        } else if self.get_number_of_token(Token::Black) < 3 {
            self.win = true;
            println!("Player 1 has won!")
        }
    }
}


#[cfg(test)]
mod tests {
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

        // mill at set phase
        game.move_to(None, 23);
        assert!(game.move_to(None,22));

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

        // mill in move phase
        game2.player_turn = 2;
        assert_eq!(game2.get_token_at_position(15), Token::Black);
        assert_eq!(game2.get_token_at_position(23), Token::None);
        assert!(game2.move_to(Some(15), 23));
        assert_eq!(game2.get_token_at_position(23), Token::Black);

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

        // mill endphase
        assert_eq!(game3.get_token_at_position(12), Token::White);
        assert_eq!(game3.get_token_at_position(2), Token::None);
        assert!(game3.move_to(Some(12), 2));
        assert_eq!(game3.get_token_at_position(2), Token::White);

        assert_eq!(game3.get_token_at_position(2), Token::White);
        assert_eq!(game3.get_token_at_position(5), Token::None);
        assert!(!game3.move_to(Some(2), 5));
        assert_eq!(game3.get_token_at_position(5), Token::White);
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

    #[test]
    fn test_search_for_mill() {
        let mut game = GameState::default();
        game.set_token_at_position(0, Token::White);
        game.set_token_at_position(1, Token::White);
        game.set_token_at_position(2, Token::White);
        game.set_token_at_position(7, Token::White);
        game.set_token_at_position(19, Token::Black);
        game.set_token_at_position(11, Token::Black);
        game.set_token_at_position(3, Token::Black);
        game.set_token_at_position(21, Token::Black);
        game.set_token_at_position(13, Token::White);
        game.set_token_at_position(5, Token::Black);
        game.set_token_at_position(6, Token::Black);
        game.set_token_at_position(4, Token::Black);
        game.set_token_at_position(14, Token::White);
        game.set_token_at_position(15, Token::White);
        game.set_token_at_position(8, Token::White);

        assert!(game.search_for_mill(0, Token::White));
        assert!(!game.search_for_mill(0, Token::Black));
        assert!(game.search_for_mill(1, Token::White));
        assert!(game.search_for_mill(2, Token::White));
        assert!(!game.search_for_mill(7, Token::White));
        
        assert!(game.search_for_mill(19, Token::Black));
        assert!(game.search_for_mill(11, Token::Black));
        assert!(game.search_for_mill(3, Token::Black));

        assert!(!game.search_for_mill(21, Token::Black));
        assert!(!game.search_for_mill(13, Token::Black));
        assert!(!game.search_for_mill(13, Token::White));
        assert!(game.search_for_mill(5, Token::Black));
        assert!(!game.search_for_mill(5, Token::White));

        assert!(game.search_for_mill(14, Token::White));
        assert!(game.search_for_mill(15, Token::White));
        assert!(!game.search_for_mill(15, Token::Black));
        assert!(game.search_for_mill(8, Token::White));
    }

    #[test]
    fn test_is_beat_possible() {
        let mut game = GameState::default();
        game.set_token_at_position(0, Token::White);
        game.set_token_at_position(1, Token::White);
        game.set_token_at_position(2, Token::White);
        game.set_token_at_position(3, Token::White);
        
        game.set_token_at_position(4, Token::Black);
        game.set_token_at_position(5, Token::Black);
        game.set_token_at_position(6, Token::Black);
        game.set_token_at_position(7, Token::Black);

        assert!(game.is_beat_possible(7));
        assert!(!game.is_beat_possible(0));
        assert!(!game.is_beat_possible(4));
        game.set_token_at_position(7, Token::None);
        assert!(game.is_beat_possible(4));

        game.player_turn = 2;
        assert!(game.is_beat_possible(3));
        assert!(!game.is_beat_possible(5));
        assert!(!game.is_beat_possible(5));
        assert!(!game.is_beat_possible(0));
        game.set_token_at_position(3, Token::None);
        assert!(game.is_beat_possible(0));
    }
}
