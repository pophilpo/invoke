use ggez::glam::Vec2;
use ggez::{glam::*, graphics, Context};

// TODO: Implement key modifiers
#[derive(Debug)]
pub struct Settings {
    pub window_width: f32,
    pub window_height: f32,
    pub score_position: Option<Vec2>,
    pub score_font_size: f32,
}

impl Settings {
    // TODO: read from file to keep the changes?

    pub fn new() -> Settings {
        let window_width = 1920.0;
        let window_height = 1080.0;
        let score_font_size = 20.0;

        Settings {
            window_width,
            window_height,
            score_position: None,
            score_font_size,
        }
    }

    pub fn calculate_score_position(&mut self, ctx: &mut Context) {
        // Generate sample text to get it's accurate width once
        let score_text = graphics::TextFragment::new("Score 9999").scale(self.score_font_size);
        let score_text = graphics::Text::new(score_text);
        let score_text_boundary = score_text.measure(ctx).unwrap();

        let score_position = Vec2::new(
            self.window_width - score_text_boundary.x - 2.0,
            self.window_height - score_text_boundary.y - 2.0,
        );
        self.score_position = Some(score_position);
    }

    pub fn change_resolution(&mut self, ctx: &mut Context, window_width: f32, window_height: f32) {
        self.window_height = window_height;
        self.window_width = window_width;

        self.calculate_score_position(ctx);
    }
}
