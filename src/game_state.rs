use crate::settings::Settings;
use crate::spells::Spell;

use ggez::{
    event::EventHandler,
    glam::*,
    graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct MainState {
    pub game_over: bool,
    pub objects: Vec<Spell>,
    pub input_buffer: Vec<char>,
    pub score: usize,
    pub speed: f32,
    pub last_spell_time: std::time::Duration,
    pub settings: Settings,
}

impl MainState {
    pub fn new(settings: Settings) -> GameResult<Self> {
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

    pub fn update_buffer(&mut self, input: char) {
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

pub struct MenuState {
    pub play_button: graphics::Rect,
}

impl MenuState {
    pub fn new(ctx: &mut Context) -> Self {
        todo!();
    }
}
