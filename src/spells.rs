use ggez::{glam::*, graphics, Context};
use rand::{seq::SliceRandom, Rng};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let y = 0.0;
        let x = rng.gen_range(200..WINDOW_HEIGHT as u32) as f32;

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
    pub fn new(ctx: &mut Context, speed: f32) -> Self {
        let position = Position::new();

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
                graphics::Image::from_path(ctx, "/alacrity.png").unwrap(),
            ),
            SpellType::ChaosMeteor => (
                vec!['E', 'E', 'W'],
                graphics::Image::from_path(ctx, "/chaos_meteor.png").unwrap(),
            ),

            SpellType::ColdSnap => (
                vec!['Q', 'Q', 'Q'],
                graphics::Image::from_path(ctx, "/cold_snap.png").unwrap(),
            ),
            SpellType::DeafeningBlast => (
                vec!['E', 'Q', 'W'],
                graphics::Image::from_path(ctx, "/deafening_blast.png").unwrap(),
            ),
            SpellType::Emp => (
                vec!['W', 'W', 'W'],
                graphics::Image::from_path(ctx, "/emp.png").unwrap(),
            ),
            SpellType::ForgeSpririt => (
                vec!['E', 'E', 'Q'],
                graphics::Image::from_path(ctx, "/forge_spirit.png").unwrap(),
            ),
            SpellType::GhostWalk => (
                vec!['Q', 'Q', 'W'],
                graphics::Image::from_path(ctx, "/ghost_walk.png").unwrap(),
            ),

            SpellType::IceWall => (
                vec!['E', 'Q', 'Q'],
                graphics::Image::from_path(ctx, "/ice_wall.png").unwrap(),
            ),

            SpellType::SunStrike => (
                vec!['E', 'E', 'E'],
                graphics::Image::from_path(ctx, "/sun_strike.png").unwrap(),
            ),

            SpellType::Tornado => (
                vec!['Q', 'Q', 'W'],
                graphics::Image::from_path(ctx, "/tornado.png").unwrap(),
            ),
            _ => unreachable!(),
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
