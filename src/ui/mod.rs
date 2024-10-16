use std::collections::HashMap;
use ggez::{
    event::{EventHandler, MouseButton}, 
    graphics::{self, Color, Image}, 
    miniquad::GraphicsContext, 
    Context, 
    GameResult
};
use crate::logic::game_state::{GameState, Token};
use crate::ui::input::InputHandler;

pub mod input;
pub mod setup;
pub mod game;

enum Winner {
    White(String),
    Black(String),
    Draw(String)
}

enum Mode {
    SinglePlayer,
    MultiPlayer,
}

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

enum State {
    Mode,
    Difficulty,
    Player,
    Game
}

#[derive(Clone)]
pub struct GameResources {
    pub game_board: Image,
    pub white_token: Image,
    pub black_token: Image,
    pub token_green_outline: Image,
    pub token_red_outline: Image,
    pub empty_token_outline: Image,
    pub single_multi_player: Image,
    pub black_white: Image,
    pub easy_normal_hard: Image
}

impl GameResources {
    pub fn new(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResources {
        GameResources {
            game_board: Image::new(ctx, quad_ctx, "/resources/muehle_board.png").unwrap(),
            white_token: Image::new(ctx, quad_ctx, "/resources/white_token.png").unwrap(),
            black_token: Image::new(ctx, quad_ctx, "/resources/black_token.png").unwrap(),
            token_green_outline: Image::new(ctx, quad_ctx, "/resources/token_green_outline.png").unwrap(),
            token_red_outline: Image::new(ctx, quad_ctx, "/resources/token_red_outline.png").unwrap(),
            empty_token_outline: Image::new(ctx, quad_ctx, "/resources/empty_token_green_outline.png").unwrap(),
            single_multi_player: Image::new(ctx, quad_ctx, "/resources/single_multiplayer.png").unwrap(),
            black_white: Image::new(ctx, quad_ctx, "/resources/black_white.png").unwrap(),
            easy_normal_hard: Image::new(ctx, quad_ctx, "/resources/easy_normal_hard.png").unwrap()
        }
    }
}

pub struct MuehleUi {
    resources: GameResources,
    game_state: GameState,
    input: Option<InputHandler>,
    winner: Option<Winner>,
    repetition: HashMap<(u64, Token), u8>,
    ai: Option<Token>,
    mode: Option<Mode>,
    difficulty: Option<Difficulty>,
    state: State
}

impl MuehleUi {
    pub fn new(ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> MuehleUi {
        MuehleUi {
            resources: GameResources::new(ctx, quad_ctx),
            game_state: GameState::default(),
            input: None,
            winner: None,
            repetition: HashMap::new(),
            ai: None,
            mode: None,
            difficulty: None,
            state: State::Mode
        }
    }
}

impl EventHandler for MuehleUi {
    fn update(&mut self, _ctx: &mut Context, _quad_ctx: &mut GraphicsContext) -> GameResult {
        match self.state {
            State::Mode | State::Difficulty | State::Player => {},
            State::Game => {
                if self.winner.is_some() {
                    return Ok(());
                }

                self.update_game();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut GraphicsContext) -> GameResult {
        graphics::set_canvas(ctx, None);
        graphics::clear(ctx, quad_ctx, Color::from_rgb(184, 111, 80));
        match self.state {
            State::Mode | State::Difficulty | State::Player => {
                self.draw_setup(ctx, quad_ctx);
            }
            State::Game => {
                self.draw_game(ctx, quad_ctx);
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        quad_ctx: &mut GraphicsContext,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        match self.state {
            State::Mode | State::Difficulty | State::Player => {
                self.setup_handle_mouse_event(quad_ctx, button, x, y);
            }
            State::Game => {
                self.game_handle_mouse_event(quad_ctx, button, x, y);
            }
        }
    }

    fn resize_event(
        &mut self,
        ctx: &mut Context,
        _quad_ctx: &mut GraphicsContext,
        width: f32,
        height: f32,
    ) {
        ctx.gfx_context.set_screen_coordinates(graphics::Rect::new(0.0, 0.0, width, height));
    }
}
