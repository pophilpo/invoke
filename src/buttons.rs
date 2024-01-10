use crate::settings::{self, Settings};
use crate::state_machine::{GameState, Transition};

use ggez::graphics::DrawParam;
use ggez::{
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

pub struct MenuButton {
    pub background: graphics::Mesh,
    pub text: graphics::Text,
    pub hint: graphics::Text,
    pub position: Vec2,
    pub dimensions: Rect,
    pub draw_param: DrawParam,
}

impl MenuButton {
    pub fn new(
        ctx: &mut Context,
        button_text: &str,
        button_hint: &str,
        settings: &Settings,
    ) -> GameResult<Self> {
        let button_width = settings.window_width / 4.0;
        let button_height = settings.window_height / 20.0;
        let button_x = (settings.window_width / 2.0) - button_width / 2.0;
        let button_y = (settings.window_height / 2.0) - button_height / 2.0;

        let position = Vec2::new(button_x, button_y);

        let dimensions = Rect::new(0.0, 0.0, button_width, button_height);

        let radius = 10.0;
        let color = Color::from_rgb(200, 200, 200);
        let background = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            dimensions,
            radius,
            color,
        )?;

        let text = graphics::Text::new(button_text)
            .set_scale(settings.font_size)
            .clone();
        let hint = graphics::Text::new(button_hint)
            .set_scale(settings.font_size * 0.5)
            .clone();

        let draw_param = graphics::DrawParam::new()
            .dest(position)
            .color(Color::WHITE);
        Ok(Self {
            background,
            text,
            hint,
            position,
            dimensions,
            draw_param,
        })
    }
}
