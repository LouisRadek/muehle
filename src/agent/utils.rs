use muehle::{game_state::Token, mill_detection::search_for_mill, r#move::is_neighbor};

use crate::{generate_actions::list_moves, Phase};

pub fn get_winner(positions: [Token; 24], phase: Phase) -> Token {
    if list_moves(&positions, Token::White, Phase::Move).count() == 0 {
        return Token::Black
    }

    if list_moves(&positions, Token::Black, Phase::Move).count() == 0 {
        return Token::White
    }

    if phase == Phase::Move && get_number_of_token(positions, Token::White) < 3 {
        return Token::Black
    }

    if phase == Phase::Move && get_number_of_token(positions, Token::Black) < 3 {
        return Token::White
    }

    return Token::None
}

pub fn get_number_of_token(positions: [Token; 24], token_type: Token) -> u8 {
    let mut number_of_token_type: u8 = 0;
    for token in positions {
        if token == token_type {
            number_of_token_type += 1
        }
    }

    return number_of_token_type
}

pub fn is_move_valid(start_position: usize, end_position: usize, end_token: Token, number_of_token_type: u8) -> bool {
    if end_token != Token::None {
        return false
    }
    
    if number_of_token_type == 3 {
        return true
    }

    if is_neighbor(start_position, end_position) {
        return true
    }

    return false
}

pub fn get_number_of_mills(positions: [Token; 24], token_type: Token) -> u8 {
    let mut number_of_mills = 0;
    for (position, _) in positions.iter().enumerate() {
        if search_for_mill(positions, position, token_type) {
            number_of_mills += 1;
        }
    }
    return number_of_mills
}

pub fn apply_action(
    mut positions: [Token; 24], 
    start_position: Option<usize>, 
    end_position: usize, 
    beatable_position: Option<usize>,
    token_type: Token
) -> [Token; 24] {
    if start_position.is_none() {
        positions[end_position] = token_type;
    } else {
        positions[start_position.unwrap()] = Token::None;
        positions[end_position] = token_type;
    }

    if beatable_position.is_some() {
        positions[beatable_position.unwrap()] = Token::None
    }

    return positions
}

#[cfg(test)]
mod tests {
    use muehle::game_state::{GameState, Token};
    use crate::{utils::{apply_action, get_number_of_token, get_winner, is_move_valid}, Phase};

    #[test]
    fn test_get_number_of_token() {
        let game = GameState::generate_example_positions();
        assert_eq!(get_number_of_token(game.get_positions(), Token::Black), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::White), 8);
        assert_eq!(get_number_of_token(game.get_positions(), Token::None), 8);

        let game2 = GameState::default();
        assert_eq!(get_number_of_token(game2.get_positions(), Token::Black), 0);
        assert_eq!(get_number_of_token(game2.get_positions(), Token::None), 24);
    }

    #[test]
    fn test_is_move_valid() {
        // move phase
        let mut game2 = GameState::generate_example_positions();
        game2.set_token_set_at_beginning(0);

        assert_eq!(is_move_valid(7, 6, Token::White, 9), false);
        assert_eq!(is_move_valid(7, 0, Token::None, 9), true);
        assert_eq!(is_move_valid(9, 17, Token::None, 9), true);
        assert_eq!(is_move_valid(8, 0, Token::None, 9), false);

        // end phase
        let mut game3 = GameState::default();
        game3.set_token_set_at_beginning(0);
        game3.set_token_at_position(0, Token::White);
        game3.set_token_at_position(1, Token::White);
        game3.set_token_at_position(4, Token::White);

        assert_eq!(is_move_valid(4, 7, Token::None, 3), true);
        assert_eq!(is_move_valid(1, 5, Token::None, 3), true);
        assert_eq!(is_move_valid(4, 1, Token::White, 3), false);
    }

    #[test]
    fn test_get_winner() {
        let game = GameState::generate_example_positions();
        assert_eq!(Token::None, get_winner(game.get_positions(), Phase::Set));

        let mut game2 = GameState::default();
        game2.set_token_at_position(0, Token::White);
        game2.set_token_at_position(1, Token::White);
        game2.set_token_at_position(2, Token::White);
        game2.set_token_at_position(4, Token::Black);
        game2.set_token_at_position(5, Token::Black);
        game2.set_token_at_position(6, Token::Black);

        assert_eq!(Token::None, get_winner(game2.get_positions(), Phase::Move));

        game2.set_token_at_position(0, Token::None);
        assert_eq!(Token::Black, get_winner(game2.get_positions(), Phase::Move));
        
        game2.set_token_at_position(0, Token::White);
        game2.set_token_at_position(6, Token::None);
        assert_eq!(Token::White, get_winner(game2.get_positions(), Phase::Move));

        let game3 = [
            Token::None, Token::White, Token::White, Token::White, Token::White, Token::White, Token::White, Token::White,
            Token::White, Token::White, Token::White, Token::Black, Token::Black, Token::Black, Token::Black, Token::White,
            Token::White, Token::White, Token::White, Token::White, Token::White, Token::White, Token::White, Token::White 
        ];
        assert_eq!(Token::White, get_winner(game3, Phase::Move))
    }

    #[test]
    fn test_apply_action() {
        let mut game = GameState::default();

        let mut expected_positions = game.get_positions().clone();
        expected_positions[0] = Token::Black;
        assert_eq!(expected_positions, apply_action(game.get_positions(), None, 0, None, Token::Black));
        game.set_token_at_position(0, Token::Black);
        expected_positions[16] = Token::White;
        assert_eq!(expected_positions, apply_action(game.get_positions(), None, 16, None, Token::White));
        game.set_token_at_position(16, Token::White);
        expected_positions[17] = Token::White;
        expected_positions[0] = Token::None;
        assert_eq!(expected_positions, apply_action(game.get_positions(), None, 17, Some(0), Token::White));

        let mut game2 = GameState::generate_example_positions();

        let mut expected_positions2 = game2.get_positions().clone();
        expected_positions2[1] = Token::None;
        expected_positions2[0] = Token::Black;
        assert_eq!(expected_positions2, apply_action(game2.get_positions(), Some(1), 0, None, Token::Black));
        game2.set_token_at_position(1, Token::None);
        game2.set_token_at_position(0, Token::Black);
        expected_positions2[6] = Token::None;
        expected_positions2[5] = Token::White;
        assert_eq!(expected_positions2, apply_action(game2.get_positions(), Some(6), 5, None, Token::White));
        game2.set_token_at_position(6, Token::None);
        game2.set_token_at_position(5, Token::White);
        expected_positions2[20] = Token::None;
        expected_positions2[21] = Token::White;
        expected_positions2[0] = Token::None;
        assert_eq!(expected_positions2, apply_action(game2.get_positions(), Some(20), 21, Some(0), Token::White));
    }
}
