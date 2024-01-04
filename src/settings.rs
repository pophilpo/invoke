use ggez::{glam::Vec2, graphics};

// TODO: Implement key modifiers
#[derive(Debug, Clone)]
pub struct Settings {
    pub window_width: f32,
    pub window_height: f32,
    pub font_size: f32,
    pub background_image_path: String,
    pub background_draw_param: graphics::DrawParam,
}

impl Settings {
    // TODO: read from file to keep the changes?

    pub fn new() -> Settings {
        let window_width = 1024.0;
        let window_height = 1024.0;
        let font_size = window_height * 0.03;
        let background_image_path = String::from("/background_tower_1.png");

        let scale_w = window_width / 1024.0;
        let scale_h = window_height / 1024.0;

        let scale = Vec2::new(scale_w, scale_h);
        let background_draw_param = graphics::DrawParam::new().scale(scale);

        Settings {
            window_width,
            window_height,
            font_size,
            background_image_path,
            background_draw_param,
        }
    }
}
