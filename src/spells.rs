use crate::settings::Settings;
use ggez::{glam::*, graphics, Context};
use rand::{seq::SliceRandom, Rng};

pub const ALACRITY_IMAGE: &[u8] = include_bytes!("../resources/alacrity.png");
pub const CHAOS_METEOR_IMAGE: &[u8] = include_bytes!("../resources/chaos_meteor.png");
pub const COLD_SNAP_IMAGE: &[u8] = include_bytes!("../resources/cold_snap.png");
pub const DEAFENING_BLAST_IMAGE: &[u8] = include_bytes!("../resources/deafening_blast.png");
pub const EMP_IMAGE: &[u8] = include_bytes!("../resources/emp.png");
pub const FORGE_SPIRIT_IMAGE: &[u8] = include_bytes!("../resources/forge_spirit.png");
pub const GHOST_WALK_IMAGE: &[u8] = include_bytes!("../resources/ghost_walk.png");
pub const ICE_WALL_IMAGE: &[u8] = include_bytes!("../resources/ice_wall.png");
pub const SUN_STRIKE_IMAGE: &[u8] = include_bytes!("../resources/sun_strike.png");
pub const TORNADO_IMAGE: &[u8] = include_bytes!("../resources/tornado.png");

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(settings: &Settings) -> Self {
        let mut rng = rand::thread_rng();
        let y = 0.0;
        let x = rng.gen_range(72..(settings.window_width - 72.0) as u32) as f32;

        Self { x, y }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum SpellType {
    Alacrity,
    ChaosMeteor,
    ColdSnap,
    DeafeningBlast,
    Emp,
    ForgeSpririt,
    GhostWalk,
    IceWall,
    SunStrike,
    Tornado,
}

pub struct Spell {
    pub spell_type: SpellType,
    pub cast: Vec<char>,
    pub object: graphics::Image,
    pub position: Position,
    pub speed: f32,
}

impl Spell {
    pub fn new(ctx: &mut Context, speed: f32, settings: &Settings) -> Self {
        let position = Position::new(settings);

        let spells = [
            SpellType::Alacrity,
            SpellType::ChaosMeteor,
            SpellType::ColdSnap,
            SpellType::DeafeningBlast,
            SpellType::Emp,
            SpellType::ForgeSpririt,
            SpellType::GhostWalk,
            SpellType::IceWall,
            SpellType::SunStrike,
            SpellType::Tornado,
        ];

        let mut rng = rand::thread_rng();
        let spell_type = *spells.choose(&mut rng).unwrap();

        let (cast, object) = match &spell_type {
            SpellType::Alacrity => (
                vec!['E', 'W', 'W'],
                graphics::Image::from_bytes(ctx, ALACRITY_IMAGE).unwrap(),
            ),
            SpellType::ChaosMeteor => (
                vec!['E', 'E', 'W'],
                graphics::Image::from_bytes(ctx, CHAOS_METEOR_IMAGE).unwrap(),
            ),

            SpellType::ColdSnap => (
                vec!['Q', 'Q', 'Q'],
                graphics::Image::from_bytes(ctx, COLD_SNAP_IMAGE).unwrap(),
            ),
            SpellType::DeafeningBlast => (
                vec!['E', 'Q', 'W'],
                graphics::Image::from_bytes(ctx, DEAFENING_BLAST_IMAGE).unwrap(),
            ),
            SpellType::Emp => (
                vec!['W', 'W', 'W'],
                graphics::Image::from_bytes(ctx, EMP_IMAGE).unwrap(),
            ),
            SpellType::ForgeSpririt => (
                vec!['E', 'E', 'Q'],
                graphics::Image::from_bytes(ctx, FORGE_SPIRIT_IMAGE).unwrap(),
            ),
            SpellType::GhostWalk => (
                vec!['Q', 'Q', 'W'],
                graphics::Image::from_bytes(ctx, GHOST_WALK_IMAGE).unwrap(),
            ),

            SpellType::IceWall => (
                vec!['E', 'Q', 'Q'],
                graphics::Image::from_bytes(ctx, ICE_WALL_IMAGE).unwrap(),
            ),

            SpellType::SunStrike => (
                vec!['E', 'E', 'E'],
                graphics::Image::from_bytes(ctx, SUN_STRIKE_IMAGE).unwrap(),
            ),

            SpellType::Tornado => (
                vec!['Q', 'W', 'W'],
                graphics::Image::from_bytes(ctx, TORNADO_IMAGE).unwrap(),
            ),
        };

        Self {
            spell_type,
            cast,
            object,
            position,
            speed,
        }
    }
}
