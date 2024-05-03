use crate::game_state::{GameState, Token};

impl GameState {
    pub fn search_for_mill(&self, position: usize, token_type: Token) -> bool {
        let positions = self.get_positions();
        
        if positions[position] != token_type {
            return false
        }

        if position % 2 == 1 {
            if [7, 15, 23].contains(&position)
               && positions[position - 1] == token_type && positions[position - 7] == token_type {
                return true
            }
            
            if positions[position - 1] == token_type && positions[position + 1] == token_type {
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
        let positions = self.get_positions();

        if position + distance_between_rings <= max_position
           && position + distance_between_rings * 2 <= max_position 
           && positions[(position + distance_between_rings) as usize] == token_type
           && positions[(position + distance_between_rings * 2) as usize] == token_type {
            return true
        }

        if position + distance_between_rings <= max_position
           && position - distance_between_rings > 0 
           && positions[(position + distance_between_rings) as usize] == token_type
           && positions[(position - distance_between_rings) as usize] == token_type {
            return true
        }

        if position - distance_between_rings > 0
           && position - distance_between_rings * 2 > 0 
           && positions[(position - distance_between_rings) as usize] == token_type
           && positions[(position - distance_between_rings * 2) as usize] == token_type {
            return true
        }

        return  false
    }

    fn search_for_horizontal_mill(&self, position: usize, token_type: Token) -> bool {
        let positions = self.get_positions();

        if [0, 8, 16].contains(&position) {
            if positions[position + 7] == token_type && positions[position + 6] == token_type {
                return true
            }

            if positions[position + 1] == token_type && positions[position + 2] == token_type {
                return true
            }
        } else if [6, 14, 22].contains(&position) {
            if positions[position - 1] == token_type && positions[position - 2] == token_type {
                return true
            }

            if positions[position + 1] == token_type && positions[position - 6] == token_type {
                return true
            }
        } else {
            if positions[position - 1] == token_type && positions[position - 2] == token_type {
                return true
            }

            if positions[position + 1] == token_type && positions[position + 2] == token_type {
                return true
            }
        }

        return false
    }

    pub fn is_beat_possible(&self, position: usize) -> bool {
        let token_of_oponent = if self.get_player_turn() == 2 {
            Token::White   
        } else {
            Token::Black
        };

        if self.get_positions()[position] != token_of_oponent {
            return false
        }

        if !self.search_for_mill(position, token_of_oponent) {
            return true
        }
        
        let mut has_oponent_only_token_in_mill = true;
        for (index, token) in self.get_positions().iter().enumerate() {
            if !self.search_for_mill(index, token_of_oponent) && *token == token_of_oponent {
                has_oponent_only_token_in_mill = false;
            }
        }

        return has_oponent_only_token_in_mill
    }

    pub fn beat_token(&mut self, position: usize) {
        self.set_token_at_position(position, Token::None);

        if self.get_token_set_at_beginning() > 0 {
            return
        }

        if self.get_number_of_token(Token::White) < 3 {
            self.set_win();
            println!("Player 2 has won!")
        } else if self.get_number_of_token(Token::Black) < 3 {
            self.set_win();
            println!("Player 1 has won!")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::{GameState, Token};

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

        game.change_player();
        assert!(game.is_beat_possible(3));
        assert!(!game.is_beat_possible(5));
        assert!(!game.is_beat_possible(5));
        assert!(!game.is_beat_possible(0));
        game.set_token_at_position(3, Token::None);
        assert!(game.is_beat_possible(0));
    }
}
