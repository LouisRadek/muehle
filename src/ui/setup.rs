use ggez::{event::MouseButton, graphics::{self, DrawParam}, miniquad::GraphicsContext, Context};

use crate::logic::game_state::Token;

use super::{game::get_scaling, Difficulty, Mode, MuehleUi, State};

fn selected_position(x: f32, y: f32, state: State) -> Option<usize> {
    let screen_pos_mode_player = vec![
        (640.0, 555.0),
        (640.0, 720.0),
    ];
    
    let screen_pos_difficulty = vec![
        (650.0, 470.0),
        (650.0, 630.0),
        (650.0, 800.0),
    ];

    let screen_positions = match state {
        State::Mode => screen_pos_mode_player,
        State::Difficulty => screen_pos_difficulty.clone(),
        State::Player => screen_pos_mode_player,
        State::Game => unreachable!(),
    };

    let button_width = if screen_positions == screen_pos_difficulty {
        500.0
    } else {
        250.0
    };
    let button_height = 150.0;

    for (index, (x1, y1)) in screen_positions.iter().enumerate() {
        let (x_min, x_max) = (x1 - button_width / 2.0, x1 + button_width / 2.0);
        let (y_min, y_max) = (y1 - button_height / 2.0, y1 + button_height / 2.0);

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return Some(index);
        }
    }
    None
}

impl MuehleUi {
    pub fn draw_setup(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) {
        let (scale, x_offset, y_offset) = get_scaling(quad_ctx, self.resources.single_multi_player.clone());

        let draw_params = DrawParam::default()
            .scale([scale, scale])
            .dest([x_offset, y_offset]);

        match self.state {
            State::Mode => {
                let _ = graphics::draw(ctx, quad_ctx, &self.resources.single_multi_player, draw_params);
            },
            State::Difficulty => {
                let _ = graphics::draw(ctx, quad_ctx,&self.resources.easy_normal_hard, draw_params);
            },
            State::Player => {
                let _ = graphics::draw(ctx, quad_ctx,&self.resources.black_white, draw_params);
            }
            State::Game => {}
        }
    }

    pub fn setup_handle_mouse_event(
        &mut self, 
        quad_ctx: &mut GraphicsContext, 
        button: MouseButton,
        x: f32, 
        y: f32
    ) {
        let (scale, x_offset, y_offset) = get_scaling(quad_ctx, self.resources.game_board.clone());

        let adjusted_x = (x - x_offset) / scale;
        let adjusted_y = (y - y_offset) / scale;
        
        if MouseButton::Left == button {
            match self.state {
                State::Mode => {
                    if let Some(p) = selected_position(adjusted_x, adjusted_y, State::Mode) {
                        match p {
                            0 => {
                                self.mode = Some(Mode::SinglePlayer);
                                self.state = State::Game;
                            },
                            1 => {
                                self.mode = Some(Mode::MultiPlayer);
                                self.state = State::Difficulty;
                            },
                            _ => {}
                        }
                    }
                },
                State::Difficulty => {
                    if let Some(p) = selected_position(adjusted_x, adjusted_y, State::Difficulty) {
                        match p {
                            0 => {
                                self.difficulty = Some(Difficulty::Easy);
                            },
                            1 => {
                                self.difficulty = Some(Difficulty::Normal);
                            },
                            2 => {
                                self.difficulty = Some(Difficulty::Hard);
                            },
                            _ => {}
                        }
                        self.state = State::Player;
                    }
                },
                State::Player => {
                    if let Some(p) = selected_position(adjusted_x, adjusted_y, State::Player) {
                        match p {
                            0 => {
                                self.ai = Some(Token::White);
                            },
                            1 => {
                                self.ai = Some(Token::Black);
                            },
                            _ => {}
                        }
                        self.state = State::Game;
                    }
                },
                State::Game => {}
            }
        }
    }
}
