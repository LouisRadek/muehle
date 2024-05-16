use muehle::game_state::Token;

use super::utils::{get_number_of_token, is_move_valid};

pub fn encode_positions(positions: [Token; 24]) -> String {
    let mut encoded_positions = String::new();

    let mut token_exception_index: Vec<Token> = Vec::new();

    for (index, token) in positions.iter().enumerate() {
        if [0, 8, 16].contains(&index) {
            token_exception_index.push(*token);
            continue
        }

        encoded_positions.push(Token::to_char(*token))
    }

    encoded_positions.insert(7, Token::to_char(token_exception_index[0]));
    encoded_positions.insert(15, Token::to_char(token_exception_index[1]));
    encoded_positions.insert(23, Token::to_char(token_exception_index[2]));

    return encoded_positions
}

pub fn decode_positions(encoded_positions: String) -> [Token; 24] {
    let mut decoded_positions = [Token::None; 24];

    for (index, position) in encoded_positions.chars().into_iter().enumerate() {
        if [7, 15, 23].contains(&index) {
            decoded_positions[index - 7] = Token::parse_to_char(position);
            continue
        }

        decoded_positions[index + 1] = Token::parse_to_char(position)
    }

    return decoded_positions
}

pub type Move = (Option<usize>, usize);
#[derive(PartialEq)]
pub enum Phase {
    Set,
    Move
}
pub fn list_moves<F>(positions: [Token; 24], token_type: Token, phase: Phase, mut callback: F) where F: FnMut(Move) {
    if phase == Phase::Set {
        for (end_position, end_token) in positions.iter().enumerate() {
            if *end_token != Token::None {
                continue;
            }

            callback((None, end_position))
        }
        return;
    } else {
        let number_of_token_type = get_number_of_token(positions, token_type); 
        for (start_position, token) in positions.iter().enumerate() {
            if token_type != *token {
                continue;
            }
    
            for (end_position, end_token) in positions.iter().enumerate() {
                if is_move_valid(start_position, end_position, *end_token, number_of_token_type) {
                    callback((Some(start_position), end_position))
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use muehle::game_state::GameState;

    use crate::enumerate::{decode_positions, encode_positions};

    #[test]
    fn test_encode_positions() {
        let positions = GameState::default().get_positions();
        let positions2 = GameState::generate_example_positions().get_positions();

        let expected_encoded_positions = "EEEEEEEEEEEEEEEEEEEEEEEE";
        let expected_encoded_positions2 = "BBEEEWWEWBWBWEBWEWBWEBEB";

        assert_eq!(expected_encoded_positions, encode_positions(positions));
        assert_eq!(expected_encoded_positions2, encode_positions(positions2))
    }

    #[test]
    fn test_decode_positions() {
        let encoded_positions = "EEEEEEEEEEEEEEEEEEEEEEEE";
        let encoded_positions2 = "BBEEEWWEWBWBWEBWEWBWEBEB";

        let expected_decoded_positions = GameState::default().get_positions();
        let expected_decoded_positions2 = GameState::generate_example_positions().get_positions();
        
        assert_eq!(expected_decoded_positions, decode_positions(encoded_positions.to_string()));
        assert_eq!(expected_decoded_positions2, decode_positions(encoded_positions2.to_string()))
    }
}
