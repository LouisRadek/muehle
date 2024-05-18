use muehle::game_state::Token;
use crate::enumerate::decode_positions;
use crate::generate_actions::generate_actions;
use rand::Rng;

mod enumerate;
mod utils;
mod test_list_moves;
mod generate_actions;
mod action;

#[derive(PartialEq)]
pub enum Phase {
    Set,
    Move
}

fn main() {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let mut input = input.trim().split(" ");
        
        
        let phase = match input.next().unwrap() {
            "P" => Phase::Set,
            "M" => Phase::Move,
            _ => panic!("Unknown phase")
        };
        
        let token_type = match input.next().unwrap() {
            "B" => Token::Black,
            "W" => Token::White,
            _ => panic!("Unknown color")
        };
        
        let positions = decode_positions(input.next().unwrap().parse().unwrap());
        let mut actions = Vec::new();
        let list_generate_actions = generate_actions(&positions, token_type, phase);
        for possible_action in list_generate_actions {
            actions.push(possible_action);
        }
        
        let random_action_index = rand::thread_rng().gen_range(0..actions.len());
        let encoded_move = actions[random_action_index].to_string();
        println!("{}", encoded_move);
    }
}
