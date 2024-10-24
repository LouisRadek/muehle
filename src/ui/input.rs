use ggez::{Context, graphics, miniquad::GraphicsContext};
use crate::logic::action::{list_actions, Action};
use crate::logic::game_state::{GameState, Phase, Token};
use crate::logic::position::get_token_at;
use super::game::get_token_draw_params;
use super::GameResources;

pub struct InputHandler {
    game_state: GameState,
    possible_actions: Vec<Action>,
    can_take: Vec<usize>,
    state: InputHandlerState,
    selected_pos: Option<usize>,
    selected_action: Option<Action>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum InputHandlerState {
    PlaceDest,
    Source,
    Dest,
    Take,
    Done,
}

impl InputHandler {
    pub fn new(game_state: GameState) -> Self {
        let possible_actions = list_actions(
            &game_state.get_board(), 
            Token::parse_to_u8(game_state.get_player_turn()), 
            game_state.get_phase(),
            None
        ).collect::<Vec<Action>>();
        let can_take = possible_actions.iter().flat_map(|action| {
           if action.beatable_position.is_some() {
                Some(action.beatable_position.unwrap())
           } else { None } 
        }).collect::<Vec<usize>>();
        Self {
            game_state,
            possible_actions,
            can_take,
            state: (match game_state.get_phase() {
                Phase::Set => { InputHandlerState::PlaceDest }
                Phase::Move => { InputHandlerState::Source }
            }),
            selected_pos: None,
            selected_action: None
        }
    }

    pub fn can_click(&self, position: usize) -> bool {
        let token_at_position = Token::parse_to_token(
            get_token_at(self.game_state.get_board(), position)
        );
        match self.state {
            InputHandlerState::PlaceDest => {
                token_at_position == Token::None
            }
            InputHandlerState::Source => {
                self.possible_actions.iter().any(|action| action.start_position.unwrap() == position)
            }
            InputHandlerState::Dest => {
                if self.selected_pos.is_some() {
                    self.possible_actions.iter().any(|&action| 
                        action.start_position.unwrap() == self.selected_pos.unwrap() 
                        && action.end_position == position
                    )
                } else { false }
            }
            InputHandlerState::Take => {
                self.can_take.contains(&position)
            }
            InputHandlerState::Done => {
                false
            }
        }
    }

    pub fn handle_click(&mut self, position: usize) {
        if self.can_click(position) {
            match self.state {
                InputHandlerState::PlaceDest => {
                    let action = Action::new(None, position, None);
                    self.selected_action = Some(action);
                }
                InputHandlerState::Source => {
                    self.selected_pos = Some(position);
                    self.state = InputHandlerState::Dest;
                }
                InputHandlerState::Dest => {
                    let action = Action::new(Some(self.selected_pos.unwrap()), position, None);
                    self.selected_action = Some(action);
                }
                InputHandlerState::Take => {
                    if let Some(action) = self.selected_action.as_mut() {
                        action.beatable_position = Some(position);
                    }
                    self.state = InputHandlerState::Done;
                }
                InputHandlerState::Done => {}
            }
            if self.selected_action.is_some() && self.state != InputHandlerState::Done  {
                let action = self.selected_action.unwrap();
                let is_mill_closing = self.possible_actions.iter().find(|possible_action| 
                    action.start_position == possible_action.start_position &&
                    action.end_position == possible_action.end_position
                );
                if is_mill_closing.unwrap().beatable_position.is_some() {
                    self.state = InputHandlerState::Take;
                } else {
                    self.state = InputHandlerState::Done;
                }
                
            }
        }
    }

    pub fn get_action(&self) -> Option<Action> {
        if self.state == InputHandlerState::Done {
            self.selected_action
        } else {
            None
        }
    }

    pub fn create_highlight_mesh(&self, ctx: &mut Context, quad_ctx: &mut GraphicsContext, resources: GameResources) {
        if self.state != InputHandlerState::Take {
            if let Some(src) = self.selected_pos {
                let outline_draw_params = get_token_draw_params(quad_ctx, src, resources.clone());
                let _ = graphics::draw(ctx, quad_ctx, &resources.token_green_outline, outline_draw_params);
            }
        }
        
        for position in 0..24 {
            if self.can_click(position) {
                let outline_draw_params = get_token_draw_params(quad_ctx, position, resources.clone());
                match self.state {
                    InputHandlerState::PlaceDest => {
                        let _ = graphics::draw(ctx, quad_ctx, &resources.empty_token_outline, outline_draw_params);
                    },
                    InputHandlerState::Source => {
                        let _ = graphics::draw(ctx, quad_ctx, &resources.token_green_outline, outline_draw_params);
                    },
                    InputHandlerState::Dest => {
                        let _ = graphics::draw(ctx, quad_ctx, &resources.empty_token_outline, outline_draw_params);
                    },
                    InputHandlerState::Take => {
                        let _ = graphics::draw(ctx, quad_ctx, &resources.token_red_outline, outline_draw_params);
                    },
                    InputHandlerState::Done => {}
                }
            }
        }
    }
    
    pub fn hint(&self) -> String {
        let tokens_left_to_place = 9 - (self.game_state.get_step_counter() / 2);
        match self.state {
            InputHandlerState::PlaceDest => {
                format!("Place a new piece. Remaining: {}", tokens_left_to_place)
            }
            InputHandlerState::Source => {
                "Move one of your pieces".to_string()
            }
            InputHandlerState::Dest => {
                "Select destination".to_string()
            }
            InputHandlerState::Take => {
                "Remove an enemy piece".to_string()
            }
            InputHandlerState::Done => {
                "Turn complete".to_string()
            }
        }
    }
}
