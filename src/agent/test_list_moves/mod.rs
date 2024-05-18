use std::cell::Cell;
use muehle::{game_state::Token, mill_detection::{is_beat_possible, search_for_mill}};
use crate::{enumerate::decode_positions, Phase};
use super::generate_actions::list_moves;

fn get_moves_formatted(encoded_positions: String) -> (u8, u8, u8) {
    let positions = decode_positions(encoded_positions);
    let mut number_of_moves: u8 = 0;
    let number_of_emerged_mills: Cell<u8> = Cell::new(0);
    let mut number_of_token_to_beat: u8 = 0;

    let moves = list_moves(&positions, Token::White, Phase::Move);
    for possible_move in moves {
        number_of_moves += 1;
        
        let mut positions_move_fake = positions;
        if !possible_move.0.is_none() {
            positions_move_fake[possible_move.0.unwrap()] = Token::None;
        }
        positions_move_fake[possible_move.1] = Token::White;

        let is_token_in_mill_after = search_for_mill(positions_move_fake, possible_move.1, Token::White);
        
        if is_token_in_mill_after {
            number_of_emerged_mills.set(number_of_emerged_mills.get() + 1)
        }
    }

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
    use std::{fs::{File, OpenOptions}, io::{self, BufRead, BufReader, Read, Write}, path::PathBuf};

    use super::get_moves_formatted;

    fn normalize_line_endings(input_path: PathBuf, output_path: PathBuf) -> io::Result<()> {
        // Open input file for reading
        let input_file = File::open(input_path)?;
        let reader = BufReader::new(input_file);
    
        // Open output file for writing
        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;
    
        // Process each line and write to output with desired line endings
        for line in reader.lines() {
            let line = line?;
            writeln!(output_file, "{}", line)?;
        }
    
        Ok(())
    }

    #[test]
    fn test_get_moves_formatted() -> io::Result<()> {
        let current_dir = std::env::current_dir()?.join("src").join("agent").join("test_list_moves");
        let output_file_formatted = current_dir.clone().join("output_formatted_moves.txt");
        let output_file_expected = current_dir.clone().join("output.txt");

        let input = File::open(current_dir.clone().join("input_felder.txt"))?;
        let buffered = BufReader::new(input);
        let mut output = File::create(output_file_formatted.clone())?;

        for line in buffered.lines() {
            let move_format = get_moves_formatted(line.unwrap());
            writeln!(output, "{} {} {}", move_format.0, move_format.1, move_format.2)?
        }

        let _ = normalize_line_endings(output_file_expected.clone(), current_dir.clone().join("output_formatted_normalized"));
        let _ = normalize_line_endings(output_file_formatted.clone(), current_dir.clone().join("output_normalized"));
        
        let mut expected_output = File::open(current_dir.clone().join("output_formatted_normalized"))?;
        let mut generated_output = File::open(current_dir.clone().join("output_normalized"))?;

        let mut buffer_expected_output = Vec::new();
        let mut buffer_generated_output = Vec::new();

        expected_output.read_to_end(&mut buffer_expected_output)?;
        generated_output.read_to_end(&mut buffer_generated_output)?;

        assert!(buffer_expected_output == buffer_generated_output);
        Ok(())
    }
}
