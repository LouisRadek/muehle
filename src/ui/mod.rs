use std::{borrow::BorrowMut, collections::HashMap};
use ggez::{event::{EventHandler, MouseButton}, graphics::{self, Color, Image}, Context, GameError, GameResult};
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
    pub fn new(ctx: &mut Context) -> GameResources {
        GameResources {
            game_board: Image::from_path(ctx, "/muehle_board.png").unwrap(),
            white_token: Image::from_path(ctx, "/white_token.png").unwrap(),
            black_token: Image::from_path(ctx, "/black_token.png").unwrap(),
            token_green_outline: Image::from_path(ctx, "/token_green_outline.png").unwrap(),
            token_red_outline: Image::from_path(ctx, "/token_red_outline.png").unwrap(),
            empty_token_outline: Image::from_path(ctx, "/empty_token_green_outline.png").unwrap(),
            single_multi_player: Image::from_path(ctx, "/single_multiplayer.png").unwrap(),
            black_white: Image::from_path(ctx, "/black_white.png").unwrap(),
            easy_normal_hard: Image::from_path(ctx, "/easy_normal_hard.png").unwrap()
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
    pub fn new(ctx: &mut Context) -> MuehleUi {
        MuehleUi {
            resources: GameResources::new(ctx),
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            State::Mode | State::Difficulty | State::Player => {},
            State::Game => {
                if self.winner.is_some() {
                    return Ok(());
                }

                self.update_game(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(184, 111, 80));

        match self.state {
            State::Mode | State::Difficulty | State::Player => {
                self.draw_setup(ctx, canvas.borrow_mut());
            }
            State::Game => {
                self.draw_game(ctx, canvas.borrow_mut());
            }
        }

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
        match self.state {
            State::Mode | State::Difficulty | State::Player => {
                self.setup_handle_mouse_event(ctx, button, x, y);
            }
            State::Game => {
                self.game_handle_mouse_event(ctx, button, x, y);
            }
        }

        Ok(())
    }
}
