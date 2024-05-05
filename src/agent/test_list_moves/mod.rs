use std::cell::Cell;
use crate::{agent::mill_detection::{is_beat_possible, search_for_mill}, game_state::Token};
use super::enumerate::{decode_positions, list_moves, Move};

fn get_moves_formatted(encoded_positions: String) -> (u8, u8, u8) {
    let positions = decode_positions(encoded_positions);
    let mut number_of_moves: u8 = 0;
    let number_of_emerged_mills: Cell<u8> = Cell::new(0);
    let mut number_of_token_to_beat: u8 = 0;

    let callback = |(start_position, end_position): Move| {
        number_of_moves += 1;

        let is_token_in_mill_before = search_for_mill(positions, start_position.unwrap(), Token::White);

        let mut positions_move_fake = positions;
        positions_move_fake[start_position.unwrap()] = Token::None;
        positions_move_fake[end_position] = Token::White;

        let is_token_in_mill_after = search_for_mill(positions_move_fake, end_position, Token::White);
        
        if (!is_token_in_mill_before && is_token_in_mill_after) || (is_token_in_mill_before && is_token_in_mill_after) {
            number_of_emerged_mills.set(number_of_emerged_mills.get() + 1)
        }
    };
    
    list_moves(positions, Token::White, 0, callback);

    if number_of_emerged_mills.get() > 0 {
        for (index, token) in positions.iter().enumerate() {
            if *token != Token::Black {
                continue;
            }

            if is_beat_possible(positions, index, Token::White) {
                number_of_token_to_beat += 1
            }
        }
    }
    
    return (number_of_moves, number_of_emerged_mills.get(), number_of_token_to_beat);
}

#[cfg(test)]
pub mod tests {
    use std::{fs::File, io::{self, BufRead, BufReader, Read, Write}};

    use super::get_moves_formatted;

    #[test]
    fn test_get_moves_formatted() -> io::Result<()> {
        let input = File::open("C:\\PROJECTS\\muehle\\src\\agent\\test_list_moves\\input_felder.txt")?;
        let buffered = BufReader::new(input);
        let mut output = File::create("C:\\PROJECTS\\muehle\\src\\agent\\test_list_moves\\output_formatted_moves.txt")?;

        for line in buffered.lines() {
            let move_format = get_moves_formatted(line.unwrap());
            writeln!(output, "{} {} {}", move_format.0, move_format.1, move_format.2)?
        }

        let mut expected_output = File::open("C:\\PROJECTS\\muehle\\src\\agent\\test_list_moves\\output.txt")?;
        let mut generated_output = File::open("C:\\PROJECTS\\muehle\\src\\agent\\test_list_moves\\output_formatted_moves.txt")?;

        let mut buffer_expected_output = Vec::new();
        let mut buffer_generated_output = Vec::new();

        expected_output.read_to_end(&mut buffer_expected_output)?;
        generated_output.read_to_end(&mut buffer_generated_output)?;

        assert!(buffer_expected_output == buffer_generated_output);

        Ok(())
    }

    #[test]
    fn test_test_list_moves() {
        let result = get_moves_formatted("WWEEWBWEBWEWEBWBWEEEEEWE".to_string());
        println!("{} {} {}", result.0, result.1, result.2)
    }
}
