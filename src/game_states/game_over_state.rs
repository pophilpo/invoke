use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct GameOverState {
    score: usize,
    score_draw_param: graphics::DrawParam,
    game_over_draw_param: graphics::DrawParam,
    font_size: f32,
    background_image: graphics::Image,
}

impl GameOverState {
    pub fn new(ctx: &mut Context, score: usize, settings: &Settings) -> Self {
        let game_over_text = graphics::TextFragment::new("Game Over!").scale(settings.font_size);
        let game_over_text = graphics::Text::new(game_over_text);
        let game_over_text_boundary = game_over_text.measure(ctx).unwrap();

        let game_over_position = Vec2::new(
            (settings.window_width / 2.0) - (game_over_text_boundary.x / 2.0),
            (settings.window_height / 2.0) - game_over_text_boundary.y,
        );

        let game_over_draw_param = graphics::DrawParam::new()
            .color(Color::BLACK)
            .dest(game_over_position);

        let score_text = graphics::TextFragment::new("Score: 999").scale(settings.font_size);
        let score_text = graphics::Text::new(score_text);
        let score_text_boundary = score_text.measure(ctx).unwrap();
        let score_position = Vec2::new(
            game_over_position.x,
            game_over_position.y + score_text_boundary.y,
        );

        let score_draw_param = graphics::DrawParam::new()
            .color(Color::BLACK)
            .dest(score_position);

        let font_size = settings.font_size;

        let background_image =
            graphics::Image::from_path(ctx, &settings.background_image_path).unwrap();

        Self {
            score,
            score_draw_param,
            game_over_draw_param,
            font_size,
            background_image,
        }
    }
}

impl GameState for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.background_image, graphics::DrawParam::default());

        let game_over_text = String::from("Game Over!");

        let game_over_text = graphics::Text::new(&game_over_text)
            .set_scale(self.font_size)
            .clone();
        canvas.draw(&game_over_text, self.game_over_draw_param);

        let score_text = format!("Score {}", self.score);

        let score_text = graphics::Text::new(&score_text)
            .set_scale(self.font_size)
            .clone();

        canvas.draw(&score_text, self.score_draw_param);

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
