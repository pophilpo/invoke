use crate::orbs::{Orb, OrbType};
use crate::spells::Spell;

pub struct InputBuffer {
    pub buffer: Vec<char>,
    pub last_spell: Option<Spell>,
    pub keypress_count: u8,
}

impl InputBuffer {
    pub fn new() -> Self {
        let buffer: Vec<char> = Vec::with_capacity(3);
        let last_spell = None;
        let keypress_count = 0;
        Self {
            buffer,
            last_spell,
            keypress_count,
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
}
