mod game_state;
mod settings;
mod spells;

use game_state::MainState;
use settings::Settings;

use ggez::{event, GameResult};

use std::{env, path};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut settings = Settings::new();

    let window_mode = ggez::conf::WindowMode::default()
        .resizable(true)
        .dimensions(settings.window_width, settings.window_height);

    let cb = ggez::ContextBuilder::new("Invoke", "pophilpo")
        .window_mode(window_mode)
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    settings.calculate_score_position(&mut ctx);

    let window_mode = ggez::conf::WindowMode::default()
        .resizable(true)
        .dimensions(settings.window_width, settings.window_height);

    ctx.gfx.set_mode(window_mode)?;

    let state = MainState::new(settings)?;
    event::run(ctx, event_loop, state)
}
