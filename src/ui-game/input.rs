use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode};
use muehle::action::{self, list_actions, Action};
use muehle::game_state::{self, GameState, Phase, Token};
use muehle::position::get_token_at;

pub struct InputHandler {
    game_state: GameState,
    possible_actions: Vec<Action>,
    can_take: Vec<usize>,
    state: InputHandlerState,
    selected_pos: Option<usize>,
    selected_action: Option<Action>,
    selected_take: Option<usize>,
}

#[derive(PartialEq, Copy, Clone)]
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
            game_state.get_phase()
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
            selected_action: None,
            selected_take: None
        }
    }

    pub fn can_click(&self, position: usize) -> bool {
        let token_at_position = get_token_at(self.game_state.get_board(), position);
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
                    self.selected_action.unwrap().set_beatable_position(Some(position));
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

    // pub fn create_highlight_mesh(&self, ctx: &mut Context, hover: Option<Pos>) -> graphics::Mesh {
    //     let mb = &mut graphics::MeshBuilder::new();

    //     if let Some(action) = self.selected_action {
    //         match action {
    //             Action::Place(dest) => {
    //                 mb.circle(DrawMode::fill(), Vec2::from(screen_pos(dest)), 10.0, 0.1, Color::from_rgb(229, 124, 35)).unwrap();
    //             }
    //             Action::Move(src, dest) => {
    //                 mb.circle(DrawMode::stroke(2.5), Vec2::from(screen_pos(src)), 22.5, 0.1, Color::from_rgb(229, 124, 35)).unwrap();
    //                 mb.circle(DrawMode::fill(), Vec2::from(screen_pos(dest)), 10.0, 0.1, Color::from_rgb(229, 124, 35)).unwrap();
    //             }
    //         }
    //     } else if let Some(src) = self.selected_pos {
    //         mb.circle(DrawMode::stroke(2.5), Vec2::from(screen_pos(src)), 22.5, 0.1, Color::from_rgb(229, 124, 35)).unwrap();
    //     }
        
    //     for pos in POSITIONS {
    //         if self.can_click(pos) {
    //             mb.circle(DrawMode::fill(), Vec2::from(screen_pos(pos)), 8.0, 0.1, Color::from_rgba(232, 170, 66, 165)).unwrap();
    //         }
    //     }
    //     if let Some(pos) = hover {
    //         if self.can_click(pos) {
    //             mb.circle(DrawMode::stroke(2.5), Vec2::from(screen_pos(pos)), 22.5, 0.1, Color::from_rgba(232, 170, 66, 165)).unwrap();
    //         }
    //     }

    //     graphics::Mesh::from_data(ctx, mb.build())
    // }
    
    pub fn hint(&self) -> &'static str {
        match self.state {
            InputHandlerState::PlaceDest => {
                "Place a new piece"
            }
            InputHandlerState::Source => {
                "Move one of your pieces"
            }
            InputHandlerState::Dest => {
                "Select destination"
            }
            InputHandlerState::Take => {
                "Remove an enemy piece"
            }
            InputHandlerState::Done => {
                "Turn complete"
            }
        }
    }
}
