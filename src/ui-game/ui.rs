use ggez::{event::EventHandler, graphics::{self, Color, DrawParam, Image, Text}, winit::window, Context, GameResult};

pub struct MuehleUi {
    game_board: Image
}

impl MuehleUi {
    pub fn new(ctx: &mut Context) -> MuehleUi {
        MuehleUi {
            game_board: Image::from_path(ctx, "/muehle_board.png").unwrap()
        }
    }
}

impl EventHandler for MuehleUi {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(184, 111, 80));

        let window_size = ctx.gfx.window().inner_size();
        let (window_width, window_height) = (window_size.width as f32, window_size.height as f32);
        let (image_width, image_height) = (self.game_board.width() as f32, self.game_board.height() as f32);

        let scale_factor = (window_width / image_width).min(window_height / image_height);
        let scaled_width = (image_width * scale_factor) as f32;
        let scaled_height = (image_height * scale_factor) as f32;
        let x_pos = (window_width - scaled_width) / 2.0;
        let y_pos = (window_height - scaled_height) / 2.0;

        canvas.draw(&self.game_board, DrawParam::default().scale([scale_factor, scale_factor]).dest([x_pos, y_pos]));
        canvas.finish(ctx).unwrap();
        Ok(())
    }
}
