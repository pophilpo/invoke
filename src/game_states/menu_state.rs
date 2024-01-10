use crate::buttons::MenuButton;
use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color, Drawable, Rect},
    input::keyboard::KeyInput,
    Context, GameResult,
};

pub struct MenuState {
    background_image: graphics::Image,
    settings: Settings,
    play_button: MenuButton,
}

impl MenuState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let play_button = MenuButton::new(ctx, "Start Game", "ENTER", settings)?;

        let background_image =
            graphics::Image::from_path(ctx, &settings.background_image_path).unwrap();

        Ok(Self {
            background_image,
            settings: settings.clone(),
            play_button,
        })
    }
}

impl GameState for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.draw(&self.background_image, self.settings.background_draw_param);

        // That drove me mad untill I found this:
        // https://github.com/ggez/ggez/issues/659
        canvas.draw(&self.play_button.background, self.play_button.draw_param);

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
            let start_game_rect = Rect::new(
                self.play_button.position.x,
                self.play_button.position.y,
                self.play_button.dimensions.w,
                self.play_button.dimensions.h,
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
