use crate::buttons::MenuButton;
use crate::settings::{Settings, BACKGROUND_IMAGE};
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct SettingsState {
    settings: Settings,
    backgroud_image: graphics::Image,
    buttons: Vec<MenuButton>,
}

impl SettingsState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE);
        let mut buttons = Vec::new();

        Ok(Self {
            settings,
            background_image,
            buttons,
        })
    }
}

impl GameState for SettingsState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw() -> GameResult {
        todo!();
    }

    fn mouse_button_up_event() -> GameResult<Transition> {
        todo!();
    }

    fn key_down_event() -> GameResult<Transition> {
        todo!();
    }
}
