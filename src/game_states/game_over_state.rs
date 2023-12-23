use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct GameOverState {
    pub score: usize,
    pub score_position: Vec2,
    pub game_over_position: Vec2,
}

impl GameOverState {
    pub fn new(_ctx: &mut Context, score: usize, settings: &Settings) -> Self {
        let game_over_x = (settings.window_width / 2.0) - 50.0;
        let game_over_y = (settings.window_height / 2.0) - 20.0;
        let game_over_position = Vec2::new(game_over_x, game_over_y);

        let score_x = game_over_x;
        let score_y = game_over_y + 35.0;
        let score_position = Vec2::new(score_x, score_y);

        Self {
            score,
            score_position,
            game_over_position,
        }
    }
}

impl GameState for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let game_over_text = String::from("Game Over!");

        let game_over_text = graphics::Text::new(&game_over_text).set_scale(40.0).clone();
        canvas.draw(&game_over_text, self.game_over_position);

        let score_text = format!("Score {}", self.score);

        let score_text = graphics::Text::new(&score_text).set_scale(40.0).clone();

        canvas.draw(&score_text, self.score_position);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult<Transition> {
        match keycode.keycode.unwrap() {
            KeyCode::Return => return Ok(Transition::Game),
            KeyCode::Escape => return Ok(Transition::Menu),
            _ => return Ok(Transition::None),
        }
    }
}
