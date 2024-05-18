use muehle::{game_state::Token, mill_detection::search_for_mill};
use muehle::mill_detection::is_beat_possible;
use crate::action::Action;
use crate::utils::{get_number_of_token, is_move_valid};
use crate::Phase;

pub fn generate_actions<'a>(positions: &'a [Token; 24], token_type: Token, phase: Phase) -> Box<dyn Iterator<Item=Action> + 'a> {
    let moves = list_moves(&positions, token_type, phase);
    Box::new(moves.flat_map(move |(start_position, end_position)| {
        let mut positions_move_fake = positions.clone();
        if !start_position.is_none() {
            positions_move_fake[start_position.unwrap()] = Token::None;
        }
        positions_move_fake[end_position] = token_type;
    
        let is_token_in_mill_after = search_for_mill(positions_move_fake, end_position, token_type);
        
        if is_token_in_mill_after {
            let token_opponent = if token_type == Token::Black {
                Token::White
            } else {
                Token::Black
            };
    
            let iter = positions.iter().enumerate()
                .filter(move |(index, token)| **token == token_opponent && is_beat_possible(*positions, *index, token_type))
                .map(move |(beatable_position, _)| {
                    Action::new(start_position, end_position, Some(beatable_position))
                });

            return Box::new(iter) as Box<dyn Iterator<Item = Action>>;
        } else {
            Box::new(std::iter::once(Action::new(start_position, end_position, None))) as Box<dyn Iterator<Item = Action>>  
        }
    }))
}

pub type Move = (Option<usize>, usize);
pub fn list_moves<'a>(positions: &'a [Token; 24], token_type: Token, phase: Phase) -> Box<dyn Iterator<Item=Move> + 'a> {
    let empty_positions = positions.iter().enumerate()
        .filter(|(_, token)| **token == Token::None);
        
    if phase == Phase::Set {
        return Box::new(empty_positions
            .map(|(end_position, _)| (None, end_position)))
    } else {
        let number_of_token = get_number_of_token(*positions, token_type);

        return Box::new(positions.iter().enumerate()
            .filter(move |(_, token)| **token == token_type)
            .flat_map(move |(start_position, _)| {
                empty_positions.clone().filter(move |(end_position, end_token)| is_move_valid(start_position, *end_position, **end_token, number_of_token))
                    .map(move |(end_position, _)| (Some(start_position), end_position))
            }));
    }
}
