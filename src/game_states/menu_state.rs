use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color, Drawable, Rect},
    input::keyboard::KeyInput,
    Context, GameResult,
};

pub struct MenuState {
    draw_param: graphics::DrawParam,
    start_game_position: Vec2,
    start_game_dimensions: Rect,
    background_image: graphics::Image,
    font_size: f32,
    settings: Settings,
}

impl MenuState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let text = String::from("Start Game");

        // Use ctx to get the text dimensions
        let play_button = graphics::Text::new(&text)
            .set_scale(settings.font_size)
            .clone();
        let start_game_dimensions = play_button.dimensions(ctx).unwrap();

        let x = (settings.window_width / 2.0) - start_game_dimensions.w / 2.0;
        let y = (settings.window_height / 2.0) - start_game_dimensions.h / 2.0;
        let start_game_position = Vec2::new(x, y);

        let draw_param = graphics::DrawParam::new()
            .dest(start_game_position)
            .color(Color::BLACK);

        let background_image =
            graphics::Image::from_path(ctx, &settings.background_image_path).unwrap();

        Ok(Self {
            draw_param,
            start_game_position,
            start_game_dimensions,
            font_size: settings.font_size,
            background_image,
            settings: settings.clone(),
        })
    }
}

impl GameState for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.draw(
            &self.background_image,
            graphics::DrawParam::new()
                .scale(Vec2::new(self.settings.scale_w, self.settings.scale_h)),
        );

        let text = String::from("Start Game");

        // That drove me mad untill I found this:
        // https://github.com/ggez/ggez/issues/659
        let play_button = graphics::Text::new(&text).set_scale(self.font_size).clone();

        canvas.draw(&play_button, self.draw_param);

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
