use ggez::{conf::{WindowMode, WindowSetup}, event, ContextBuilder};
use ui::MuehleUi;

mod ui;
mod input;

fn main() {
    // Make a Context
    let (mut ctx, event_loop) = ContextBuilder::new("muehle", "Louis Radek")
        .add_resource_path("./resources")
        .window_mode(WindowMode::default()
            .dimensions(800.0, 500.0)
            .min_dimensions(800.0, 500.0)
            .resizable(true)
            .resize_on_scale_factor_change(true))
        .window_setup(WindowSetup::default()
            .title("Muehle")
            .vsync(true))
        .build()
        .expect("Could not create ggez context!");
    let ui = MuehleUi::new(&mut ctx);

    event::run(ctx, event_loop, ui);
}
