mod spells;

use spells::{Spell, SpellType};

use ggez::{
    event::{self, EventHandler},
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use std::{env, path};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

struct MainState {
    game_over: bool,
    win: bool,
    objects: Vec<Spell>,
    input_buffer: Vec<char>,
    score: usize,
    speed: f32,
    last_spell_time: std::time::Duration,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            game_over: false,
            win: false,
            objects: Vec::new(),
            input_buffer: Vec::with_capacity(3),
            last_spell_time: std::time::Duration::new(0, 0),
            speed: 0.0,
            score: 0,
        })
    }

    fn update_buffer(&mut self, input: char) {
        if self.input_buffer.len() == 3 {
            self.input_buffer.remove(0);
        }
        self.input_buffer.push(input);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.last_spell_time += ctx.time.delta();
        if self.last_spell_time > std::time::Duration::new(1, 0) || self.objects.is_empty() {
            self.last_spell_time = std::time::Duration::new(0, 0);
            self.speed += 0.5;
            let new_spell = Spell::new(ctx, self.speed);
            self.objects.push(new_spell);
        }

        if self.game_over {
            println!("Game is over. Win {}", self.win);
            return Ok(());
        } else {
            for object in self.objects.iter_mut() {
                object.position.y += object.speed;
                if object.position.y > WINDOW_HEIGHT {
                    self.game_over = true;
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for spell in &self.objects {
            canvas.draw(&spell.object, Vec2::new(spell.position.x, spell.position.y));
        }
        let input: String = self.input_buffer.iter().collect();
        let text = graphics::Text::new(input).set_scale(48.).clone();

        let score_text = graphics::Text::new(format!("Score {}", self.score))
            .set_scale(30.)
            .clone();

        canvas.draw(&text, Vec2::new(960.0, 1000.0));

        canvas.draw(&score_text, Vec2::new(1700.0, 1000.0));

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if !self.game_over {
            match keycode.keycode.unwrap() {
                KeyCode::Q => {
                    self.update_buffer('Q');
                }

                KeyCode::W => {
                    self.update_buffer('W');
                }

                KeyCode::E => {
                    self.update_buffer('E');
                }

                KeyCode::R => {
                    let mut index_to_remove = None;
                    for (index, object) in self.objects.iter().enumerate() {
                        let mut sorted_buffer = self.input_buffer.clone();
                        sorted_buffer.sort_unstable();
                        if sorted_buffer == object.cast {
                            self.score += 1;
                            index_to_remove = Some(index);
                            break;
                        }
                    }
                    if let Some(index) = index_to_remove {
                        self.objects.remove(index);
                    } else {
                        self.game_over = true;
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let window_mode = ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT); // Set your desired window size here

    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(window_mode)
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
