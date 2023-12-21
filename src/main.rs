mod settings;
mod spells;

use settings::Settings;
use spells::Spell;

use ggez::{
    event::{self, EventHandler},
    glam::*,
    graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use std::{env, path};

struct MainState {
    game_over: bool,
    objects: Vec<Spell>,
    input_buffer: Vec<char>,
    score: usize,
    speed: f32,
    last_spell_time: std::time::Duration,
    settings: Settings,
}

impl MainState {
    fn new(settings: Settings) -> GameResult<Self> {
        Ok(Self {
            game_over: false,
            objects: Vec::new(),
            input_buffer: Vec::with_capacity(3),
            last_spell_time: std::time::Duration::new(0, 0),
            speed: 0.0,
            score: 0,
            settings,
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
            let new_spell = Spell::new(ctx, self.speed, &self.settings);
            self.objects.push(new_spell);
        }

        if self.game_over {
            println!("Game is over.");
            return Ok(());
        } else {
            for object in self.objects.iter_mut() {
                object.position.y += object.speed;
                if object.position.y > self.settings.window_height {
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
            .set_scale(self.settings.score_font_size)
            .clone();

        canvas.draw(&text, Vec2::new(960.0, 1000.0));

        canvas.draw(&score_text, self.settings.score_position.unwrap());

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

    let mut settings = Settings::new();

    let window_mode = ggez::conf::WindowMode::default()
        .resizable(true)
        .dimensions(settings.window_width, settings.window_height);

    let cb = ggez::ContextBuilder::new("Invoke", "pophilpo")
        .window_mode(window_mode)
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    settings.calculate_score_position(&mut ctx);

    settings.change_resolution(&mut ctx, 640.0, 800.0);

    settings.calculate_score_position(&mut ctx);

    settings.calculate_score_position(&mut ctx);
    let window_mode = ggez::conf::WindowMode::default()
        .resizable(true)
        .dimensions(settings.window_width, settings.window_height);

    ctx.gfx.set_mode(window_mode)?;

    println!("{:?}", settings);

    let state = MainState::new(settings)?;
    event::run(ctx, event_loop, state)
}
