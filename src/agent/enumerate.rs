use crate::game_state::Token;

use super::{r#move::{is_move_valid, is_neighbor}, utils::get_number_of_token};

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
pub fn list_moves<F>(positions: [Token; 24], token_type: Token, token_set_at_beginning: u8, mut callback: F) where F: FnMut(Move) {
    if token_set_at_beginning > 0 {
        for (end_position, end_token) in positions.iter().enumerate() {
            if *end_token != Token::None {
                continue;
            }

            callback((None, end_position))
        }
        return;
    }

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

#[cfg(test)]
pub mod tests {
    use crate::{agent::enumerate::{decode_positions, encode_positions, list_moves, Move}, game_state::{tests::generate_example_positions, GameState, Token}};

    #[test]
    fn test_encode_positions() {
        let positions = GameState::default().get_positions();
        let positions2 = generate_example_positions().get_positions();

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
        let expected_decoded_positions2 = generate_example_positions().get_positions();
        
        assert_eq!(expected_decoded_positions, decode_positions(encoded_positions.to_string()));
        assert_eq!(expected_decoded_positions2, decode_positions(encoded_positions2.to_string()))
    }

    #[test]
    fn test_list_moves() {
        let mut game = generate_example_positions();
        game.set_token_set_at_beginning(0);

        fn callback((start_position, end_position): Move) {
            println!("start: {}, end: {}", start_position.unwrap_or(100), end_position)
        }

        list_moves(game.get_positions(), Token::White, game.get_token_set_at_beginning(), callback)

        /* * Expected for white
          *  start: 6, end: 5
          *  start: 7, end: 0
          *  start: 9, end: 17
          *  start: 11, end: 3
          *  start: 13, end: 5
          *  start: 13, end: 14
          *  start: 13, end: 21
          *  start: 18, end: 17
          *  start: 20, end: 21
         */

        /* * Expected for Black
            start: 1, end: 0
            start: 2, end: 3
            start: 15, end: 14
            start: 15, end: 23
            start: 16, end: 17
            start: 16, end: 23
            start: 22, end: 21
            start: 22, end: 23
         */
    }
}
