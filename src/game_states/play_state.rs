use crate::assets::BACKGROUND_IMAGE;
use crate::input_buffer::InputBuffer;
use crate::orbs::{Orb, OrbType};
use crate::settings::Settings;
use crate::spells::Spell;
use crate::state_machine::{GameState, Transition};

use std::collections::HashMap;

use ggez::{
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct MainState {
    game_over: bool,
    objects: Vec<Spell>,
    input_buffer: InputBuffer,
    input_buffer_draw_param: graphics::DrawParam,
    score: usize,
    score_draw_param: graphics::DrawParam,
    speed: f32,
    last_spell_time: std::time::Duration,
    settings: Settings,
    background_image: graphics::Image,
    keybindings: HashMap<KeyCode, Orb>,
}

impl MainState {
    pub fn new(settings: Settings, ctx: &mut Context) -> GameResult<Self> {
        let input_buffer_draw_param = Self::calculate_buffer_position(&settings, ctx);
        let score_draw_param = Self::calculate_score_position(&settings, ctx);
        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE)?;

        let quas = Orb::new(ctx, OrbType::Quas)?;
        let wex = Orb::new(ctx, OrbType::Wex)?;
        let exort = Orb::new(ctx, OrbType::Exort)?;
        let invoke = Orb::new(ctx, OrbType::Invoke)?;

        let mut keybindings: HashMap<KeyCode, Orb> = HashMap::with_capacity(4);

        keybindings.insert(settings.quas_key, quas);
        keybindings.insert(settings.wex_key, wex);
        keybindings.insert(settings.exort_key, exort);
        keybindings.insert(settings.invoke_key, invoke);

        let input_buffer = InputBuffer::new();

        Ok(Self {
            game_over: false,
            objects: Vec::new(),
            input_buffer,
            input_buffer_draw_param,
            last_spell_time: std::time::Duration::new(0, 0),
            speed: 0.0,
            score: 0,
            score_draw_param,
            settings,
            background_image,
            keybindings,
        })
    }

    fn get_buffer_text(&self) -> graphics::Text {
        let input: String = self.input_buffer.buffer.iter().collect();
        graphics::Text::new(input)
            .set_scale(self.settings.font_size)
            .clone()
    }

    fn calculate_buffer_position(settings: &Settings, ctx: &mut Context) -> graphics::DrawParam {
        let buffer_text = graphics::TextFragment::new("WWW").scale(settings.font_size);
        let buffer_text = graphics::Text::new(buffer_text);
        let buffer_text_boundary = buffer_text.measure(ctx).unwrap();

        let buffer_position = Vec2::new(
            (settings.window_width / 2.0) - (buffer_text_boundary.x / 2.0),
            settings.window_height - buffer_text_boundary.y * 2.0,
        );

        graphics::DrawParam::new()
            .color(Color::BLACK)
            .dest(buffer_position)
    }

    fn calculate_score_position(settings: &Settings, ctx: &mut Context) -> graphics::DrawParam {
        let score_text = graphics::TextFragment::new("Score 9999").scale(settings.font_size);
        let score_text = graphics::Text::new(score_text);
        let score_text_boundary = score_text.measure(ctx).unwrap();

        let score_position = Vec2::new(
            settings.window_width - score_text_boundary.x,
            settings.window_height - score_text_boundary.y * 2.0,
        );

        graphics::DrawParam::new()
            .color(Color::BLACK)
            .dest(score_position)
    }
}

impl GameState for MainState {
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition> {
        self.last_spell_time += ctx.time.delta();
        if self.last_spell_time > std::time::Duration::new(1, 0) || self.objects.is_empty() {
            self.last_spell_time = std::time::Duration::new(0, 0);
            self.speed += 0.5;
            let new_spell = Spell::new(ctx, self.speed, &self.settings);
            self.objects.push(new_spell);
        }

        if self.game_over {
            return Ok(Transition::GameOver { score: self.score });
        } else {
            for object in self.objects.iter_mut() {
                object.position.y += object.speed;
                if object.position.y > self.settings.window_height {
                    self.game_over = true;
                    return Ok(Transition::GameOver { score: self.score });
                }
            }
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        canvas.draw(&self.background_image, self.settings.background_draw_param);
        for spell in &self.objects {
            canvas.draw(&spell.object, Vec2::new(spell.position.x, spell.position.y));
        }

        let buffer_text = self.get_buffer_text();
        let score_text = graphics::Text::new(format!("Score {}", self.score))
            .set_scale(self.settings.font_size)
            .clone();

        canvas.draw(&buffer_text, self.input_buffer_draw_param);
        canvas.draw(&score_text, self.score_draw_param);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult<Transition> {
        if !self.game_over {
            match keycode.keycode.unwrap() {
                KeyCode::Escape => return Ok(Transition::Menu),

                key => {
                    if let Some(orb) = self.keybindings.get(&key) {
                        let spell_cast = self.input_buffer.update_buffer(&orb);

                        match spell_cast {
                            None => return Ok(Transition::None),
                            Some(cast) => {
                                let mut index_to_remove = None;
                                for (index, object) in self.objects.iter().enumerate() {
                                    if cast == object.cast {
                                        self.score += 1;
                                        index_to_remove = Some(index);
                                        break;
                                    }
                                }
                                if let Some(index) = index_to_remove {
                                    self.objects.remove(index);

                                    return Ok(Transition::None);
                                } else {
                                    self.game_over = true;

                                    return Ok(Transition::GameOver { score: self.score });
                                }
                            }
                        }
                    }
                    Ok(Transition::None)
                }
            }
        } else {
            Ok(Transition::GameOver { score: self.score })
        }
    }
}
