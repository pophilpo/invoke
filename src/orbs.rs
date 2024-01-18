use crate::assets::{EXORT, QUAS, WEX};
use ggez::{graphics, input::keyboard::KeyCode, Context, GameResult};

enum OrbType {
    Quas,
    Wex,
    Exort,
}

struct Orb {
    orb_type: OrbType,
    orb_image: graphics::Image,
    key_mapping: KeyCode,
}

impl Orb {
    pub fn new(ctx: &mut Context, orb_type: OrbType, key_mapping: String) -> GameResult<Self> {
        let orb_image = match orb_type {
            OrbType::Quas => graphics::Image::from_bytes(ctx, QUAS)?,
            OrbType::Wex => graphics::Image::from_bytes(ctx, WEX)?,
            OrbType::Exort => graphics::Image::from_bytes(ctx, EXORT)?,
        };

        let key_mapping = Self::string_to_keycode(key_mapping);

        Ok(Self {
            orb_type,
            orb_image,
            key_mapping,
        })
    }

    fn string_to_keycode(key_mapping: String) -> KeyCode {
        match key_mapping.as_str() {
            "A" => KeyCode::A,
            "B" => KeyCode::B,
            "C" => KeyCode::C,
            "D" => KeyCode::D,
            "E" => KeyCode::E,
            "F" => KeyCode::F,
            "G" => KeyCode::G,
            "H" => KeyCode::H,
            "I" => KeyCode::I,
            "J" => KeyCode::J,
            "K" => KeyCode::K,
            "L" => KeyCode::L,
            "M" => KeyCode::M,
            "N" => KeyCode::N,
            "O" => KeyCode::O,
            "P" => KeyCode::P,
            "Q" => KeyCode::Q,
            "R" => KeyCode::R,
            "S" => KeyCode::S,
            "T" => KeyCode::T,
            "U" => KeyCode::U,
            "V" => KeyCode::V,
            "W" => KeyCode::W,
            "X" => KeyCode::X,
            "Y" => KeyCode::Y,
            "Z" => KeyCode::Z,
            _ => unreachable!(),
        }
    }
}
