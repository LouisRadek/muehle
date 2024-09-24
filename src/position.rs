use crate::game_state::Token;

pub fn get_token_at(board: u64, position: usize) -> Token {
    Token::parse_to_token(((board >> (46 - position * 2)) & 0b11) as u8)
}

pub fn set_token_at(board: u64, position: usize, token: u8) -> u64 {
    let new_board = board & !(0b11 << (46 - position * 2));
    new_board | ((token as u64) << (46 - position * 2))
}

pub fn negate_token(token: u8) -> u8 {
    match token {
        0b10 => 0b11,
        0b11 => 0b10,
        _ => 0b00
    }
}

pub fn reverse_token_of_board(mut board: u64) -> u64 {
    let mut reversed_board: u64 = 0;
    for i in 0..24 {
        let token = negate_token((board & 0b11) as u8) as u64;
        reversed_board |= token << i*2;
        board >>= 2;
    }
    reversed_board
}

pub fn create_token_iter<'a>(board: u64) -> impl Iterator<Item = u8> + 'a {
    (0..24).rev().map(move |i| {
        let result: u8 = (board >> (i*2) & 0b11) as u8;
        result
    })
}

pub fn get_number_of_tokens(board: u64, token: u8) -> u8 {
    if token == 0b11 {
        (board & 0b010101010101010101010101010101010101010101010101).count_ones() as u8
    } else {
        ((!board & 0b010101010101010101010101010101010101010101010101).count_ones() + (board & 0b101010101010101010101010101010101010101010101010).count_ones() - 24) as u8
    }
}

#[allow(dead_code)]
pub fn print_board(board: u64) {
    let board_vec = create_token_iter(board).map(|token| if token == 0b00 { "E" } else if token == 0b10 { "B" } else { "W" }).collect::<Vec<&str>>();
    println!("{}------------{}------------{}   ", board_vec[7], board_vec[0], board_vec[1]);
    println!("|            |            |      ");
    println!("|   {}--------{}--------{}   |   ", board_vec[15], board_vec[8], board_vec[9]);
    println!("|   |        |        |   |      ");
    println!("|   |   {}----{}----{}   |   |   ", board_vec[23], board_vec[16], board_vec[17]);
    println!("|   |   |         |   |   |      ");
    println!("{}---{}---{}         {}---{}---{}", board_vec[6], board_vec[14], board_vec[22], board_vec[18], board_vec[10], board_vec[2]);
    println!("|   |   |         |   |   |      ");
    println!("|   |   {}----{}----{}   |   |   ", board_vec[21], board_vec[20], board_vec[19]);
    println!("|   |        |        |   |      ");
    println!("|   {}--------{}--------{}   |   ", board_vec[13], board_vec[12], board_vec[11]);
    println!("|            |            |      ");
    println!("{}------------{}------------{}   ", board_vec[5], board_vec[4], board_vec[3]);
}

pub fn decode_positions(encoded_positions: String) -> u64 {
    let mut decoded_positions: u64 = 0b0;

    for char in encoded_positions.trim().chars().into_iter() {
        match char {
            'B' => decoded_positions |= 0b10,
            'W' => decoded_positions |= 0b11,
            'E' => (),
            _ => panic!("Invalid character in encoded positions")
        }
        decoded_positions <<= 2;
    }
    decoded_positions >>= 2;

    return decoded_positions
}

pub fn encode_positions(board: u64) -> String {
    let mut encoded_positions = String::new();
    for index in 0..24 {
        match (board & (0b11 << (46 - index*2))) >> (46 - index*2) {
            0b10 => encoded_positions.push('B'),
            0b11 => encoded_positions.push('W'),
            _ => encoded_positions.push('E')
        };
    }
    return encoded_positions
}

#[cfg(test)]
pub mod tests {
    use crate::{game_state::Token, position::{get_number_of_tokens, reverse_token_of_board}};

    #[test]
    fn test_get_token_at() {
        use crate::position::get_token_at;
        let board1: u64 = 0b101000000011110011101110110010110011101100100010; // BBEEEWWE WBWBWEBW EWBWEBEB

        assert_eq!(Token::Black, get_token_at(board1, 0));
        assert_eq!(Token::Black, get_token_at(board1, 1));
        assert_eq!(Token::None, get_token_at(board1, 2));
        assert_eq!(Token::None, get_token_at(board1, 3));
        assert_eq!(Token::None, get_token_at(board1, 4));
        assert_eq!(Token::White, get_token_at(board1, 5));
        assert_eq!(Token::White, get_token_at(board1, 6));
        assert_eq!(Token::None, get_token_at(board1, 7));
        assert_eq!(Token::White, get_token_at(board1, 8));
        assert_eq!(Token::Black, get_token_at(board1, 9));
        assert_eq!(Token::White, get_token_at(board1, 10));
        assert_eq!(Token::Black, get_token_at(board1, 11));
        assert_eq!(Token::White, get_token_at(board1, 12));
        assert_eq!(Token::None, get_token_at(board1, 13));
        assert_eq!(Token::Black, get_token_at(board1, 14));
        assert_eq!(Token::White, get_token_at(board1, 15));
        assert_eq!(Token::None, get_token_at(board1, 16));
        assert_eq!(Token::White, get_token_at(board1, 17));
        assert_eq!(Token::Black, get_token_at(board1, 18));
        assert_eq!(Token::White, get_token_at(board1, 19));
        assert_eq!(Token::None, get_token_at(board1, 20));
        assert_eq!(Token::Black, get_token_at(board1, 21));
        assert_eq!(Token::None, get_token_at(board1, 22));
        assert_eq!(Token::Black, get_token_at(board1, 23));
    }

    #[test]
    fn test_set_token_at() {
        use super::set_token_at;

        let mut board = 0b0;
        board = set_token_at(board, 0, 0b11);
        board = set_token_at(board, 23, 0b11);
        assert_eq!(0b110000000000000000000000000000000000000000000011, board);
    }

    #[test]
    fn test_negate_token() {
        use crate::position::negate_token;
        assert_eq!(0b11, negate_token(0b10));
        assert_eq!(0b10, negate_token(0b11));
        assert_eq!(0b00, negate_token(0b00));
        assert_eq!(0b00, negate_token(0b01));
        assert_eq!(0b00, negate_token(0b10011010));
    }

    #[test]
    fn test_reverse_token_of_board() {
        let board = 0b101000000011110011101110110010110011101100100010;
        let reversed_board = 0b111100000010100010111011100011100010111000110011;

        assert_eq!(reversed_board, reverse_token_of_board(board));
    }

    #[test]
    fn test_create_token_iter() {
        use crate::position::create_token_iter;
        let board1: u64 = 0b0;
        for position in create_token_iter(board1) {
            assert_eq!(0b00, position);
        }

        let board2: u64 = 0b101000000011110011101110110010110011101100100010;
        let expected_positions: Vec<u8> = vec![
            0b10,
            0b00,
            0b10,
            0b00,
            0b11,
            0b10,
            0b11,
            0b00,
            0b11,
            0b10,
            0b00,
            0b11,
            0b10,
            0b11,
            0b10,
            0b11,
            0b00,
            0b11,
            0b11,
            0b00,
            0b00,
            0b00,
            0b10,
            0b10
        ];
        for (index, position) in create_token_iter(board2).enumerate() {
            assert_eq!(expected_positions[23 - index], position);
        }
    }

    #[test]
    fn test_get_number_of_tokens() {
        let board1 = 0b0;
        let board2 = 0b101000000011110011101110110010110011101100100010;
        let board3 = 0b000000000011110011101110110010110011101100100010;
        let board4 = 0b111100000011110011101110110010110011101100100010;

        assert_eq!((0, 0), (get_number_of_tokens(board1, 0b10), get_number_of_tokens(board1, 0b11)));
        assert_eq!((8, 8), (get_number_of_tokens(board2, 0b10), get_number_of_tokens(board2, 0b11)));
        assert_eq!((6, 8), (get_number_of_tokens(board3, 0b10), get_number_of_tokens(board3, 0b11)));
        assert_eq!((6, 10), (get_number_of_tokens(board4, 0b10), get_number_of_tokens(board4, 0b11)));
    }

    #[test]
    fn test_decode_positions() {
        use crate::position::decode_positions;
        let encoded_positions = "EEEEEEEEEEEEEEEEEEEEEEEE";
        let encoded_positions2 = "BBEEEWWEWBWBWEBWEWBWEBEB";
        let encoded_positions3 = "WEEWEWBWBBEEBWEWEEEBEEEE";

        let expected_decoded_positions = 0b0;
        let expected_decoded_positions2 = 0b101000000011110011101110110010110011101100100010;
        let expected_positions3 = 0b110000110011101110100000101100110000001000000000;
        
        assert_eq!(expected_decoded_positions, decode_positions(encoded_positions.to_string()));
        assert_eq!(expected_decoded_positions2, decode_positions(encoded_positions2.to_string()));
        assert_eq!(expected_positions3, decode_positions(encoded_positions3.to_string()));
    }

    #[test]
    fn test_encode_positions() {
        use crate::position::encode_positions;
        let positions = 0b0;
        let positions2 = 0b101000000011110011101110110010110011101100100010;

        let expected_encoded_positions = "EEEEEEEEEEEEEEEEEEEEEEEE";
        let expected_encoded_positions2 = "BBEEEWWEWBWBWEBWEWBWEBEB";

        assert_eq!(expected_encoded_positions, encode_positions(positions));
        assert_eq!(expected_encoded_positions2, encode_positions(positions2))
    }
}
