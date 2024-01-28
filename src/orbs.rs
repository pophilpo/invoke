use crate::assets::{EXORT, INVOKE, QUAS, WEX};
use ggez::{graphics, Context, GameResult};

#[derive(Clone, PartialEq)]
pub enum OrbType {
    Quas,
    Wex,
    Exort,

    // Not an orb techincally but who cares?
    Invoke,
}

#[derive(Clone)]
pub struct Orb {
    pub orb_type: OrbType,
    pub orb_image: graphics::Image,
}

impl Orb {
    pub fn new(ctx: &mut Context, orb_type: OrbType) -> GameResult<Self> {
        let orb_image = match orb_type {
            OrbType::Quas => graphics::Image::from_bytes(ctx, QUAS)?,
            OrbType::Wex => graphics::Image::from_bytes(ctx, WEX)?,
            OrbType::Exort => graphics::Image::from_bytes(ctx, EXORT)?,
            OrbType::Invoke => graphics::Image::from_bytes(ctx, INVOKE)?,
        };

        Ok(Self {
            orb_type,
            orb_image,
        })
    }
}
