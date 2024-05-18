use muehle::game_state::Token;

pub fn encode_positions(positions: [Token; 24]) -> String {
    let mut encoded_positions = String::new();

    let mut token_exception_index: Vec<Token> = Vec::new();

    for (index, token) in positions.iter().enumerate() {
        if index == 0 || index == 8 || index == 16 {
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

pub fn encode_single_position(position: usize) -> String {
    if position == 0 || position == 8 || position == 16 {
        return (position + 8).to_string()
    } else {
        return position.to_string()
    }
}

pub fn decode_positions(encoded_positions: String) -> [Token; 24] {
    let mut decoded_positions = [Token::None; 24];

    for (index, position) in encoded_positions.chars().into_iter().enumerate() {
        if index == 7 || index == 15 || index == 23 {
            decoded_positions[index - 7] = Token::parse_to_char(position);
            continue
        }

        decoded_positions[index + 1] = Token::parse_to_char(position)
    }

    return decoded_positions
}

#[cfg(test)]
pub mod tests {
    use muehle::game_state::{GameState, Token};

    use crate::enumerate::{decode_positions, encode_positions, encode_single_position};

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

    #[test]
    fn test_decode_positions2() {
        let encoded_positions = "WEEWEWBWBBEEBWEWEEEBEEEE";
        let expected_positions = [
            Token::White, Token::White, Token::None, Token::None, Token::White, Token::None, Token::White, Token::Black,
            Token::White, Token::Black, Token::Black, Token::None, Token::None, Token::Black, Token::White, Token::None,
            Token::None, Token::None, Token::None, Token::None, Token::Black, Token::None, Token::None, Token::None,
        ];

        assert_eq!(expected_positions, decode_positions(encoded_positions.to_string()))
    }

    #[test]
    fn test_encode_single_position() {
        assert_eq!("8", encode_single_position(0));
        assert_eq!("1", encode_single_position(1));
        assert_eq!("2", encode_single_position(2));
        assert_eq!("3", encode_single_position(3));
        assert_eq!("4", encode_single_position(4));
        assert_eq!("5", encode_single_position(5));
        assert_eq!("6", encode_single_position(6));
        assert_eq!("7", encode_single_position(7));
        assert_eq!("16", encode_single_position(8));
        assert_eq!("9", encode_single_position(9));
        assert_eq!("10", encode_single_position(10));
        assert_eq!("11", encode_single_position(11));
        assert_eq!("12", encode_single_position(12));
        assert_eq!("13", encode_single_position(13));
        assert_eq!("14", encode_single_position(14));
        assert_eq!("15", encode_single_position(15));
        assert_eq!("24", encode_single_position(16));
        assert_eq!("17", encode_single_position(17));
        assert_eq!("18", encode_single_position(18));
        assert_eq!("19", encode_single_position(19));
        assert_eq!("20", encode_single_position(20));
        assert_eq!("21", encode_single_position(21));
        assert_eq!("22", encode_single_position(22));
        assert_eq!("23", encode_single_position(23));
    }
}
