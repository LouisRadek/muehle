use crate::game_state::{GameState, Token};

pub fn search_for_mill(positions: [Token; 24], position: usize, token_type: Token) -> bool {
    if positions[position] != token_type {
        return false
    }

    if position % 2 == 1 {
        if [7, 15, 23].contains(&position) {
            if positions[position - 1] == token_type && positions[position - 7] == token_type {
                return true
            } else {
                return search_for_vertical_mill(positions, position as isize, token_type)
            }
        }
        
        if positions[position - 1] == token_type && positions[position + 1] == token_type {
            return true
        }

        return search_for_vertical_mill(positions, position as isize, token_type)
    } else {
        return search_for_horizontal_mill(positions, position, token_type)
    }
}

fn search_for_vertical_mill(positions: [Token; 24], position: isize, token_type: Token) -> bool {
    let distance_between_rings = 8;
    let max_position = 23;

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

fn search_for_horizontal_mill(positions: [Token; 24], position: usize, token_type: Token) -> bool {
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

pub fn is_beat_possible(positions: [Token; 24], position: usize, token_current_player: Token) -> bool {
    let token_of_oponent = if token_current_player == Token::Black {
        Token::White   
    } else {
        Token::Black
    };

    if positions[position] != token_of_oponent {
        return false
    }

    if !search_for_mill(positions, position, token_of_oponent) {
        return true
    }
    
    let mut has_oponent_only_token_in_mill = true;
    for (index, token) in positions.iter().enumerate() {
        if !search_for_mill(positions, index, token_of_oponent) && *token == token_of_oponent {
            has_oponent_only_token_in_mill = false;
        }
    }

    return has_oponent_only_token_in_mill
}

#[cfg(test)]
mod tests {
    use crate::{game_state::{GameState, Token}, mill_detection::{is_beat_possible, search_for_mill}};

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

        assert!(search_for_mill(game.get_positions(), 0, Token::White));
        assert!(!search_for_mill(game.get_positions(), 0, Token::Black));
        assert!(search_for_mill(game.get_positions(), 1, Token::White));
        assert!(search_for_mill(game.get_positions(), 2, Token::White));
        assert!(!search_for_mill(game.get_positions(), 7, Token::White));
        
        assert!(search_for_mill(game.get_positions(), 19, Token::Black));
        assert!(search_for_mill(game.get_positions(), 11, Token::Black));
        assert!(search_for_mill(game.get_positions(), 3, Token::Black));

        assert!(!search_for_mill(game.get_positions(), 21, Token::Black));
        assert!(!search_for_mill(game.get_positions(), 13, Token::Black));
        assert!(!search_for_mill(game.get_positions(), 13, Token::White));
        assert!(search_for_mill(game.get_positions(), 5, Token::Black));
        assert!(!search_for_mill(game.get_positions(), 5, Token::White));

        assert!(search_for_mill(game.get_positions(), 14, Token::White));
        assert!(search_for_mill(game.get_positions(), 15, Token::White));
        assert!(!search_for_mill(game.get_positions(), 15, Token::Black));
        assert!(search_for_mill(game.get_positions(), 8, Token::White));
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

        let token_of_player = Token::White;
        assert!(is_beat_possible(game.get_positions(), 7, token_of_player));
        assert!(!is_beat_possible(game.get_positions(), 0, token_of_player));
        assert!(!is_beat_possible(game.get_positions(), 4, token_of_player));
        game.set_token_at_position(7, Token::None);
        assert!(is_beat_possible(game.get_positions(), 4, token_of_player));

        game.change_player();
        let token_of_player = Token::Black;
        assert!(is_beat_possible(game.get_positions(), 3, token_of_player));
        assert!(!is_beat_possible(game.get_positions(), 5, token_of_player));
        assert!(!is_beat_possible(game.get_positions(), 5, token_of_player));
        assert!(!is_beat_possible(game.get_positions(), 0, token_of_player));
        game.set_token_at_position(3, Token::None);
        assert!(is_beat_possible(game.get_positions(), 0, token_of_player));
    }
}
