use crate::settings::Settings;

use ggez::graphics::{DrawParam, Drawable};
use ggez::{
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

pub struct MenuButton {
    pub background: graphics::Mesh,
    pub text: graphics::Text,
    pub text_draw_param: DrawParam,
    pub hint: graphics::Text,
    pub hint_draw_param: DrawParam,
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
        button_y: f32,
    ) -> GameResult<Self> {
        let button_width = settings.window_width / 4.0;
        let button_height = settings.window_height / 20.0;

        // Button is always centered by W
        let button_x = (settings.window_width / 2.0) - button_width / 2.0;

        let position = Vec2::new(button_x, button_y);

        let dimensions = Rect::new(0.0, 0.0, button_width, button_height);

        let radius = 10.0;
        let color = Color::from_rgb(180, 180, 190);
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
        let text_dimensions = text
            .dimensions(ctx)
            .expect("Text Drawable always has dimensions");

        let text_position_x = position.x + (dimensions.w / 2.0) - (text_dimensions.w / 2.0);
        let text_position_y = position.y + (dimensions.h / 2.0) - (text_dimensions.h / 2.0);
        let text_position = Vec2::new(text_position_x, text_position_y);
        let text_draw_param = DrawParam::new().dest(text_position).color(Color::BLACK);

        let hint = graphics::Text::new(button_hint)
            .set_scale(settings.font_size * 0.5)
            .clone();
        let hint_dimensions = hint
            .dimensions(ctx)
            .expect("Text Drawable always has dimensions");

        let hint_position_x = (position.x + dimensions.w) - (hint_dimensions.w * 1.2);
        let hint_position_y = (position.y + dimensions.h) - hint_dimensions.h;
        let hint_position = Vec2::new(hint_position_x, hint_position_y);
        let hint_draw_param = DrawParam::new().dest(hint_position).color(Color::BLACK);

        let draw_param = graphics::DrawParam::new()
            .dest(position)
            .color(Color::WHITE);
        Ok(Self {
            background,
            text,
            text_draw_param,
            hint,
            hint_draw_param,
            position,
            dimensions,
            draw_param,
        })
    }
}
