use ggez::{glam::*, graphics::DrawParam};

use crate::orbs::{Orb, OrbType};
use crate::settings::Settings;
use crate::spells::Spell;

pub struct InputBuffer {
    pub buffer: Vec<char>,
    pub last_spell: Option<Spell>,
    pub keypress_count: u8,

    pub draw_params: Vec<DrawParam>,
}

impl InputBuffer {
    pub fn new(settings: &Settings) -> Self {
        let buffer: Vec<char> = Vec::with_capacity(3);
        let last_spell = None;
        let keypress_count = 0;

        let draw_params = Self::calculate_orb_draw_params(settings);

        Self {
            buffer,
            last_spell,
            keypress_count,
            draw_params,
        }
    }

    pub fn update_buffer(&mut self, orb: &Orb) -> Option<Vec<char>> {
        // Stupid, I know
        if self.buffer.len() == 3 && orb.orb_type != OrbType::Invoke {
            self.buffer.remove(0);
        }

        match orb.orb_type {
            OrbType::Quas => {
                self.buffer.push('Q');
                None
            }
            OrbType::Wex => {
                self.buffer.push('W');
                None
            }
            OrbType::Exort => {
                self.buffer.push('E');
                None
            }
            OrbType::Invoke => {
                let mut sorted_buffer = self.buffer.clone();
                sorted_buffer.sort_unstable();
                return Some(sorted_buffer);
            }
        }
    }

    fn calculate_orb_draw_params(settings: &Settings) -> Vec<DrawParam> {
        // TODO: Dynamic icon sizes
        let buffer_y = settings.window_height - (77.0 * 1.5);

        // Space between orbs
        let padding = 10.0;

        let buffer_width = 77.0 * 3.0 + padding * 2.0;

        // First orb of the buffer position
        let first_x = (settings.window_width / 2.0) - (buffer_width / 2.0);

        let second_x = first_x + 77.0 + padding;
        let third_x = second_x + 77.0 + padding;

        let first_position = Vec2::new(first_x, buffer_y);
        let second_position = Vec2::new(second_x, buffer_y);
        let third_position = Vec2::new(third_x, buffer_y);

        let first_draw_param = DrawParam::new().dest(first_position);
        let second_draw_param = DrawParam::new().dest(second_position);
        let third_draw_param = DrawParam::new().dest(third_position);

        vec![first_draw_param, second_draw_param, third_draw_param]
    }
}
