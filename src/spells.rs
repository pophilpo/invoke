use ggez::{
    glam::*,
    graphics::{self, Color},
    Context,
};
use rand::Rng;

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

pub enum SpellType {
    Tornado,
}

pub struct Spell {
    pub spell_type: SpellType,
    pub cast: Vec<char>,
    pub object: graphics::Mesh,
    pub position: Position,
    pub speed: f32,
}

impl Spell {
    pub fn new(ctx: &mut Context, spell_type: SpellType, speed: f32) -> Self {
        let position = Position::new();
        let cast = match spell_type {
            SpellType::Tornado => vec!['Q', 'Q', 'W'],
            _ => unreachable!(),
        };

        let object = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            20.0,
            2.0,
            Color::WHITE,
        )
        .unwrap();

        Self {
            spell_type,
            cast,
            object,
            position,
            speed,
        }
    }
}
