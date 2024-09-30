use std::{borrow::BorrowMut, collections::HashMap, ops::Deref};
use ggez::{event::{EventHandler, MouseButton}, graphics::{self, Color, DrawParam, Image, Text}, Context, GameError, GameResult};
use crate::logic::{action::{list_actions, Action}, game_state::{GameState, Phase, Token}, r#move::apply_action, position::{create_token_iter, get_number_of_tokens}};
use crate::ui::input::InputHandler;

pub const SCREEN_POS: [(f32, f32); 24] = [
    // outer ring
    (560.0, 90.0),
    (1030.0, 90.0),
    (1030.0, 560.0),
    (1030.0, 1030.0),
    (560.0, 1030.0),
    (90.0, 1030.0),
    (90.0, 560.0),
    (90.0, 90.0),
    // middle ring
    (560.0, 250.0),
    (870.0, 250.0),
    (870.0, 560.0),
    (870.0, 870.0),
    (560.0, 870.0),
    (250.0, 870.0),
    (250.0, 560.0),
    (250.0, 250.0),
    // inner ring
    (560.0, 410.0),
    (710.0, 410.0),
    (710.0, 560.0),
    (710.0, 710.0),
    (560.0, 710.0),
    (410.0, 710.0),
    (410.0, 560.0),
    (410.0, 410.0)
];

fn selected_position(x: f32, y: f32) -> Option<usize> {
    for p in 0..24 {
        let (x1, y1) = SCREEN_POS[p];
        if (x - x1) * (x - x1) + (y - y1) * (y - y1) < 50.0 * 50.0 {
            return Some(p);
        }
    }
    None
}

pub fn get_token_draw_params(ctx: &mut Context, position: usize, resources: GameResources) -> DrawParam {
    let (scale, x_offset, y_offset) = get_scaling(ctx, resources.clone());
    
    let (mut x, mut y) = SCREEN_POS[position];
    x = x * scale + x_offset;
    y = y * scale + y_offset;

    DrawParam::default()
        .scale([scale, scale])
        .dest([x, y])
}

fn get_scaling(ctx: &mut Context, resources: GameResources) -> (f32, f32, f32) {
    let (window_width, window_height) = ctx.gfx.drawable_size();
    let (image_width, image_height) = (resources.game_board.width() as f32, resources.game_board.height() as f32);

    let scale = (window_width / image_width).min(window_height / image_height);
    let x_offset = (window_width - (image_width * scale)) / 2.0;
    let y_offset = (window_height - (image_width * scale)) / 2.0;

    (scale, x_offset, y_offset)
}

enum Winner {
    White(String),
    Black(String),
    Draw(String)
}

#[derive(Clone)]
pub struct GameResources {
    pub game_board: Image,
    pub white_token: Image,
    pub black_token: Image,
    pub token_green_outline: Image,
    pub token_red_outline: Image,
    pub empty_token_outline: Image
}

impl GameResources {
    pub fn new(ctx: &mut Context) -> GameResources {
        GameResources {
            game_board: Image::from_path(ctx, "/muehle_board.png").unwrap(),
            white_token: Image::from_path(ctx, "/white_token.png").unwrap(),
            black_token: Image::from_path(ctx, "/black_token.png").unwrap(),
            token_green_outline: Image::from_path(ctx, "/token_green_outline.png").unwrap(),
            token_red_outline: Image::from_path(ctx, "/token_red_outline.png").unwrap(),
            empty_token_outline: Image::from_path(ctx, "/empty_token_green_outline.png").unwrap()
        }
    }
}

pub struct MuehleUi {
    resources: GameResources,
    game_state: GameState,
    input: Option<InputHandler>,
    winner: Option<Winner>,
    repetition: HashMap<(u64, Token), u8>
}

impl MuehleUi {
    pub fn new(ctx: &mut Context) -> MuehleUi {
        MuehleUi {
            resources: GameResources::new(ctx),
            game_state: GameState::default(),
            input: None,
            winner: None,
            repetition: HashMap::new()
        }
    }

    fn apply_action(&mut self, action: Action) {
        let game_state = self.game_state.borrow_mut();
        let successor = apply_action(&game_state.get_board(), &action, Token::parse_to_u8(game_state.get_player_turn()));
        game_state.set_board(successor);
        game_state.change_player();
        game_state.decrement_token_set_at_beginning();


        if game_state.get_phase() == Phase::Move {
            let parsed_player_token = Token::parse_to_u8(game_state.get_player_turn());
            if get_number_of_tokens(game_state.get_board(), parsed_player_token) == 2 
                || list_actions(&game_state.get_board(), parsed_player_token, game_state.get_phase()).count() == 0 {
                self.winner = Some(match game_state.get_player_turn() {
                    Token::White => Winner::Black("".to_string()),
                    Token::Black => Winner::White("".to_string()),
                    _ => unreachable!()
                });
            } else {
                let cnt = self.repetition.entry((successor, game_state.get_player_turn())).or_insert(0);
                *cnt += 1;
                if *cnt >= 3 {
                    self.winner = Some(Winner::Draw("Position repeated thrice".to_string()));
                }
            }
        }
    }
}

impl EventHandler for MuehleUi {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.winner.is_some() {
            return Ok(());
        }

        // integration of ai should be here

        if let Some(input) = self.input.as_ref() {
            if let Some(action) = input.get_action() {
                self.apply_action(action);
                self.input = None;
            }
        } else {
            self.input = Some(InputHandler::new(self.game_state));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(184, 111, 80));

        let (board_scale, x_offset, y_offset) = get_scaling(ctx, self.resources.clone());
        canvas.draw(
            &self.resources.game_board, 
            DrawParam::default().scale([board_scale, board_scale]).dest([x_offset, y_offset])
        );

        if let Some(input) = self.input.as_ref() {
            input.create_highlight_mesh(ctx, &mut canvas, self.resources.clone());
        }

        create_token_iter(self.game_state.get_board())
            .enumerate()
            .for_each(|(position, token)| {
                let token_draw_params = get_token_draw_params(ctx, position, self.resources.clone());
                match Token::parse_to_token(token) {
                    Token::White => canvas.draw(&self.resources.white_token, token_draw_params),
                    Token::Black => canvas.draw(&self.resources.black_token, token_draw_params),
                    _ => {}
                }
            });

        let (heading, subheading) = if let Some(winner) = self.winner.as_ref() {
            match winner {
                Winner::White(s) => { ("White won".to_string(), s.deref()) }
                Winner::Black(s) => { ("Black won".to_string(), s.deref()) }
                Winner::Draw(s) => { ("Draw".to_string(), s.deref()) }
            }
        } else {
            let subheading = if let Some(input) = self.input.as_ref() {
                input.hint()
            } else {
                "Waiting for engine..."
            };
            (format!("{}'s turn", self.game_state.get_player_turn()), subheading)
        };
        canvas.draw(
            Text::new(&heading).set_scale(60.0 * board_scale), 
            graphics::DrawParam::from([20.0 * board_scale, 10.0* board_scale])
        );
        canvas.draw(
            Text::new(subheading).set_scale(40.0 * board_scale), 
            graphics::DrawParam::from([20.0* board_scale, 70.0* board_scale])
        );

        canvas.finish(ctx).unwrap();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        let (scale, x_offset, y_offset) = get_scaling(ctx, self.resources.clone());

        let adjusted_x = ((x - x_offset) / scale) - 80.0;
        let adjusted_y = ((y - y_offset) / scale) - 80.0;

        if button == MouseButton::Left {
            if let Some(position) = selected_position(adjusted_x, adjusted_y) {
                if let Some(input) = self.input.as_mut() {
                    input.handle_click(position);
                }
            }
        } else if button == MouseButton::Right {
            if let Some(input) = self.input.as_mut() {
                *input = InputHandler::new(self.game_state)
            }
        }
        Ok(())
    }
}
