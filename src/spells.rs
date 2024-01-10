use crate::settings::Settings;
use ggez::{glam::*, graphics::Image, Context};
use rand::{seq::SliceRandom, Rng};

const ALACRITY_IMAGE: &[u8] = include_bytes!("../resources/alacrity.png");
const CHAOS_METEOR_IMAGE: &[u8] = include_bytes!("../resources/chaos_meteor.png");
const COLD_SNAP_IMAGE: &[u8] = include_bytes!("../resources/cold_snap.png");
const DEAFENING_BLAST_IMAGE: &[u8] = include_bytes!("../resources/deafening_blast.png");
const EMP_IMAGE: &[u8] = include_bytes!("../resources/emp.png");
const FORGE_SPIRIT_IMAGE: &[u8] = include_bytes!("../resources/forge_spirit.png");
const GHOST_WALK_IMAGE: &[u8] = include_bytes!("../resources/ghost_walk.png");
const ICE_WALL_IMAGE: &[u8] = include_bytes!("../resources/ice_wall.png");
const SUN_STRIKE_IMAGE: &[u8] = include_bytes!("../resources/sun_strike.png");
const TORNADO_IMAGE: &[u8] = include_bytes!("../resources/tornado.png");

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
#[derive(Clone, Debug)]
pub enum SpellType {
    Alacrity(Image),
    ChaosMeteor(Image),
    ColdSnap(Image),
    DeafeningBlast(Image),
    Emp(Image),
    ForgeSpririt(Image),
    GhostWalk(Image),
    IceWall(Image),
    SunStrike(Image),
    Tornado(Image),
}

pub struct Spell {
    pub cast: Vec<char>,
    pub object: Image,
    pub position: Position,
    pub speed: f32,
}

impl Spell {
    pub fn new(ctx: &mut Context, speed: f32, settings: &Settings) -> Self {
        let position = Position::new(settings);

        let spells = [
            SpellType::Alacrity(Image::from_bytes(ctx, ALACRITY_IMAGE).unwrap()),
            SpellType::ChaosMeteor(Image::from_bytes(ctx, CHAOS_METEOR_IMAGE).unwrap()),
            SpellType::ColdSnap(Image::from_bytes(ctx, COLD_SNAP_IMAGE).unwrap()),
            SpellType::DeafeningBlast(Image::from_bytes(ctx, DEAFENING_BLAST_IMAGE).unwrap()),
            SpellType::Emp(Image::from_bytes(ctx, EMP_IMAGE).unwrap()),
            SpellType::ForgeSpririt(Image::from_bytes(ctx, FORGE_SPIRIT_IMAGE).unwrap()),
            SpellType::GhostWalk(Image::from_bytes(ctx, GHOST_WALK_IMAGE).unwrap()),
            SpellType::IceWall(Image::from_bytes(ctx, ICE_WALL_IMAGE).unwrap()),
            SpellType::SunStrike(Image::from_bytes(ctx, SUN_STRIKE_IMAGE).unwrap()),
            SpellType::Tornado(Image::from_bytes(ctx, TORNADO_IMAGE).unwrap()),
        ];

        let mut rng = rand::thread_rng();
        let spell_type = spells.choose(&mut rng).unwrap();

        let (cast, object) = match &spell_type {
            SpellType::Alacrity(image) => (vec!['E', 'W', 'W'], image),
            SpellType::ChaosMeteor(image) => (vec!['E', 'E', 'W'], image),

            SpellType::ColdSnap(image) => (vec!['Q', 'Q', 'Q'], image),
            SpellType::DeafeningBlast(image) => (vec!['E', 'Q', 'W'], image),
            SpellType::Emp(image) => (vec!['W', 'W', 'W'], image),
            SpellType::ForgeSpririt(image) => (vec!['E', 'E', 'Q'], image),
            SpellType::GhostWalk(image) => (vec!['Q', 'Q', 'W'], image),
            SpellType::IceWall(image) => (vec!['E', 'Q', 'Q'], image),
            SpellType::SunStrike(image) => (vec!['E', 'E', 'E'], image),
            SpellType::Tornado(image) => (vec!['Q', 'W', 'W'], image),
        };

        Self {
            cast,
            object: object.clone(),
            position,
            speed,
        }
    }
}
