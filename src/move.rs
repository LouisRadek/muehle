use crate::{game_state::{GameState, Token}, mill_detection::search_for_mill};

impl GameState {
    /**
     * Makes a move if it is valid and returns if a new mill emerged
     */
    pub fn move_to(&mut self, start_position: Option<usize>, end_position: usize) -> bool {
        let token = if self.get_player_turn() == 1 {
            Token::White   
        } else {
            Token::Black
        };

        if start_position.is_none() {
            self.decrement_token_set_at_beginning();
            self.set_token_at_position(end_position, token);
            
            return search_for_mill(self.get_positions(), end_position, token)
        }

        self.set_token_at_position(start_position.unwrap(), Token::None);
        self.set_token_at_position(end_position, token);

        let is_token_in_mill_before = search_for_mill(self.get_positions(), start_position.unwrap(), token);
        let is_token_in_mill_after = search_for_mill(self.get_positions(), end_position, token);

        if (!is_token_in_mill_before && is_token_in_mill_after) || (is_token_in_mill_before && is_token_in_mill_after) {
            return true
        }

        return false
    }
    
    pub fn is_move_valid(&self, start_position: Option<usize>, end_position: usize) -> bool {
        if start_position.is_none() {
            if self.get_token_set_at_beginning() > 0 && self.get_token_at_position(end_position) == Token::None {
                return true
            }

            return false
        }

        if self.get_token_at_position(end_position) == Token::None {
            if self.get_player_turn() == 1 && self.get_token_at_position(start_position.unwrap()) == Token::White {
                if self.get_number_of_token(Token::White) <= 3 {
                    return true
                }
                
                if is_neighbor(start_position.unwrap(), end_position) {
                    return true
                }
            }

            if self.get_player_turn() == 2 && self.get_token_at_position(start_position.unwrap()) == Token::Black {
                if self.get_number_of_token(Token::Black) <= 3 {
                    return true
                }
                
                if is_neighbor(start_position.unwrap(), end_position) {
                    return true
                }
            }
        }

        return false
    }
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
    use crate::{game_state::{tests::generate_example_positions, GameState, Token}, r#move::is_neighbor};

    #[test]
    fn test_move_to() {
        // set phase
        let mut game = GameState::default();
        
        assert_eq!(game.get_token_at_position(0), Token::None);
        game.move_to(None, 0);
        assert_eq!(game.get_token_at_position(0), Token::White);

        game.change_player();
        assert_eq!(game.get_token_at_position(16), Token::None);
        game.move_to(None, 16);
        assert_eq!(game.get_token_at_position(16), Token::Black);

        // mill at set phase
        game.move_to(None, 23);
        assert!(game.move_to(None,22));

        // move phase
        let mut game2 = generate_example_positions();
        game2.set_token_set_at_beginning(0);

        assert_eq!(game2.get_token_at_position(7), Token::White);
        assert_eq!(game2.get_token_at_position(0), Token::None);
        game2.move_to(Some(7), 0);
        assert_eq!(game2.get_token_at_position(7), Token::None);
        assert_eq!(game2.get_token_at_position(0), Token::White);

        game2.change_player();
        assert_eq!(game2.get_token_at_position(2), Token::Black);
        assert_eq!(game2.get_token_at_position(3), Token::None);
        game2.move_to(Some(2), 3);
        assert_eq!(game2.get_token_at_position(2), Token::None);
        assert_eq!(game2.get_token_at_position(3), Token::Black);

        game2.change_player();
        assert_eq!(game2.get_token_at_position(13), Token::White);
        assert_eq!(game2.get_token_at_position(21), Token::None);
        game2.move_to(Some(13), 21);
        assert_eq!(game2.get_token_at_position(13), Token::None);
        assert_eq!(game2.get_token_at_position(21), Token::White);

        // mill in move phase
        game2.change_player();
        assert_eq!(game2.get_token_at_position(15), Token::Black);
        assert_eq!(game2.get_token_at_position(23), Token::None);
        assert!(game2.move_to(Some(15), 23));
        assert_eq!(game2.get_token_at_position(23), Token::Black);

        // endphase
        let mut game3 = GameState::default();
        game3.set_token_set_at_beginning(0);
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
        game2.set_token_set_at_beginning(0);

        assert_eq!(game2.is_move_valid(Some(7), 6), false);
        assert_eq!(game2.is_move_valid(Some(1), 0), false);
        assert_eq!(game2.is_move_valid(Some(7), 0), true);
        assert_eq!(game2.is_move_valid(Some(9), 17), true);
        assert_eq!(game2.is_move_valid(Some(8), 0), false);

        // end phase
        let mut game3 = GameState::default();
        game3.set_token_set_at_beginning(0);
        game3.set_token_at_position(0, Token::White);
        game3.set_token_at_position(1, Token::White);
        game3.set_token_at_position(4, Token::White);

        assert_eq!(game3.is_move_valid(Some(4), 7), true);
        assert_eq!(game3.is_move_valid(Some(1), 5), true);
        assert_eq!(game3.is_move_valid(Some(4), 1), false);
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
