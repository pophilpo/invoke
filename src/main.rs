mod game_states;
mod settings;
mod spells;
mod state_machine;

use crate::game_states::menu_state::MenuState;

use settings::Settings;
use state_machine::StateMachine;

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

    let cb = ggez::ContextBuilder::new("Invoke", "Invoke")
        .window_mode(window_mode)
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let initial_state = MenuState::new(&mut ctx, &settings)?;

    let state_machine = StateMachine::new(Box::new(initial_state), settings);

    event::run(ctx, event_loop, state_machine)
}
