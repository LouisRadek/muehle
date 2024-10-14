extern crate good_web_game as ggez;
use ggez::conf::Conf;
use ui::MuehleUi;

mod agent;
mod logic;
mod ui;

fn main() {
    let config = Conf::default()
        .cache(Some(include_bytes!("../resources.tar")))
        .window_title("Muehle".to_string())
        .window_width(600)
        .window_height(600)
        .window_resizable(true);

    let _ = ggez::start(config, |mut ctx, quad_ctx| {
        Box::new(MuehleUi::new(&mut ctx, quad_ctx))
    });
}
