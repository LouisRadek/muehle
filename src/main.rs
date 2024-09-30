use ggez::{conf::{WindowMode, WindowSetup}, event, ContextBuilder};
use ui::ui::MuehleUi;

mod agent;
mod logic;
mod ui;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("muehle", "Louis Radek")
        .add_resource_path("./resources")
        .window_mode(WindowMode::default()
            .dimensions(600.0, 600.0)
            .min_dimensions(600.0, 600.0)
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
