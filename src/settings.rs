use ggez::glam::Vec2;
use ggez::{glam::*, graphics, Context};

// TODO: Implement key modifiers
#[derive(Debug, Clone)]
pub struct Settings {
    pub window_width: f32,
    pub window_height: f32,
    pub score_position: Option<Vec2>,
    pub font_size: f32,
}

impl Settings {
    // TODO: read from file to keep the changes?

    pub fn new() -> Settings {
        let window_width = 800.0;
        let window_height = 1500.0;
        let font_size = window_height * 0.03;

        Settings {
            window_width,
            window_height,
            score_position: None,
            font_size,
        }
    }
}
