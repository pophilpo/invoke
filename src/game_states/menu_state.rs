use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color, Drawable, Rect},
    input::keyboard::KeyInput,
    Context, GameResult,
};

pub struct MenuState {
    pub start_game_position: Vec2,
    pub start_game_dimensions: Rect,
}

impl MenuState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let x = (settings.window_width / 2.0) - 50.0;
        let y = (settings.window_height / 2.0) - 20.0;

        let text = String::from("Start Game");

        // Use ctx to get the text dimensions
        let play_button = graphics::Text::new(&text).set_scale(40.0).clone();
        let start_game_dimensions = play_button.dimensions(ctx).unwrap();

        Ok(Self {
            start_game_position: Vec2::new(x, y),
            start_game_dimensions,
        })
    }
}

impl GameState for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let text = String::from("Start Game");

        // That drove me mad untill I found this:
        // https://github.com/ggez/ggez/issues/659
        let play_button = graphics::Text::new(&text).set_scale(40.0).clone();
        canvas.draw(&play_button, self.start_game_position);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult<Transition> {
        if button == ggez::event::MouseButton::Left {
            let cursor_location = Vec2::new(x, y);
            println!("{:?}", cursor_location);

            let start_game_rect = Rect::new(
                self.start_game_position.x,
                self.start_game_position.y,
                self.start_game_dimensions.w,
                self.start_game_dimensions.h,
            );

            if start_game_rect.contains(cursor_location) {
                return Ok(Transition::Game);
            }
        }
        Ok(Transition::None)
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }
}
