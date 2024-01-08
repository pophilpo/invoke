use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

pub struct Button {
    pub background: graphics::Mesh,
    pub text: graphics::Text,
    pub hint: graphics::Text,
    pub position: Vec2,
}

impl Button {
    pub fn new(
        ctx: &mut Context,
        button_text: &str,
        button_hint: &str,
        position: Vec2,
        dimensions: Rect,
        font_size: f32,
    ) -> GameResult<Self> {
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
            .set_scale(font_size)
            .clone();
        let hint = graphics::Text::new(button_hint)
            .set_scale(font_size * 0.5)
            .clone();

        Ok(Self {
            background,
            text,
            hint,
            position,
        })
    }
}
