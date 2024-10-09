use std::{borrow::BorrowMut, time::Duration};
use ggez::{event::MouseButton, graphics::{self, Canvas, DrawParam, Image, Text}, timer::sleep, Context};
use crate::{agent::{calculate_next_move, AiPhase}, logic::{action::{list_actions, Action}, game_state::{Phase, Token}, r#move::apply_action, position::{create_token_iter, get_number_of_tokens}}};
use super::{input::InputHandler, Difficulty, GameResources, MuehleUi, Winner};

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
    let (scale, x_offset, y_offset) = get_scaling(ctx, resources.game_board);
    
    let (mut x, mut y) = SCREEN_POS[position];
    x = x * scale + x_offset;
    y = y * scale + y_offset;

    DrawParam::default()
        .scale([scale, scale])
        .dest([x, y])
}

pub fn get_scaling(ctx: &mut Context, image: Image) -> (f32, f32, f32) {
    let (window_width, window_height) = ctx.gfx.drawable_size();

    let scale = (window_width / image.width() as f32).min(window_height / image.height() as f32);
    let x_offset = (window_width - (image.width() as f32 * scale)) / 2.0;
    let y_offset = (window_height - (image.height() as f32 * scale)) / 2.0;

    (scale, x_offset, y_offset)
}

impl MuehleUi {
    fn apply_action(&mut self, action: Action) {
        self.last_action = Some(action.clone());
        let game_state = self.game_state.borrow_mut();
        let successor = apply_action(&game_state.get_board(), &action, Token::parse_to_u8(game_state.get_player_turn()));
        game_state.set_board(successor);
        game_state.change_player();
        game_state.increase_step_counter();


        if game_state.get_phase() == Phase::Move {
            let parsed_player_token = Token::parse_to_u8(game_state.get_player_turn());
            if get_number_of_tokens(game_state.get_board(), parsed_player_token) == 2 
                || list_actions(&game_state.get_board(), parsed_player_token, game_state.get_phase(), None).count() == 0 {
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

    pub fn update_game(&mut self, _ctx: &mut Context) {
        let player_turn = self.game_state.get_player_turn();

        if self.ai.is_some() && player_turn == self.ai.unwrap() {
            let board = self.game_state.get_board();
            let ai_phase = AiPhase::new(self.game_state.get_phase(), self.game_state.get_step_counter());
            let max_time  = match self.difficulty.as_ref().unwrap() {
                Difficulty::Easy => 1,
                Difficulty::Normal => 3,
                Difficulty::Hard => 15
            };
            let action = calculate_next_move(board, player_turn, ai_phase, max_time);
            let possible_actions = list_actions(
                &board, 
                Token::parse_to_u8(player_turn), 
                self.game_state.get_phase(), 
                None
            ).collect::<Vec<Action>>();

            sleep(Duration::from_millis(750));
    
            if possible_actions.contains(&action) {
                self.apply_action(action);
            } else {
                self.winner = Some(match player_turn {
                    Token::White => Winner::Black("White attempted illegal move".to_string()),
                    Token::Black => Winner::White("Black attempted illegal move".to_string()),
                    _ => unreachable!()
                });
            }
        } else if let Some(input) = self.input.as_ref() {
            if let Some(action) = input.get_action() {
                self.apply_action(action);
                self.input = None;
            }
        } else {
            self.input = Some(InputHandler::new(self.game_state));
        }
    }

    pub fn draw_game(&mut self, ctx: &mut Context, mut canvas: &mut Canvas) {
        let (board_scale, x_offset, y_offset) = get_scaling(ctx, self.resources.game_board.clone());
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
                Winner::White(s) => { ("White won".to_string(), s.to_string()) }
                Winner::Black(s) => { ("Black won".to_string(), s.to_string()) }
                Winner::Draw(s) => { ("Draw".to_string(), s.to_string()) }
            }
        } else {
            let subheading = if let Some(input) = self.input.as_ref() {
                input.hint(self.ai, self.last_action.clone())
            } else {
                "Waiting for engine...".to_string()
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
    }

    pub fn game_handle_mouse_event(
        &mut self, 
        ctx: &mut Context, 
        button: MouseButton,
        x: f32, 
        y: f32
    ) {
        let (scale, x_offset, y_offset) = get_scaling(ctx, self.resources.game_board.clone());

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
    }
}
