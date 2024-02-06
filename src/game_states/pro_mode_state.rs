use crate::assets::BACKGROUND_IMAGE;
use crate::input_buffer::InputBuffer;
use crate::orbs::{Orb, OrbType};
use crate::settings::Settings;
use crate::spells::Spell;
use crate::state_machine::{GameState, Transition};

use std::collections::HashMap;

use ggez::{
    glam::*,
    graphics::{self},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct ProMode {
    game_over: bool,
    objects: Vec<Spell>,
    input_buffer: InputBuffer,
    score: usize,
    speed: f32,
    last_spell_time: std::time::Duration,
    settings: Settings,
    background_image: graphics::Image,
    keybindings: HashMap<KeyCode, Orb>,
    orbs: HashMap<char, graphics::Image>,
}

impl ProMode {
    pub fn new(settings: Settings, ctx: &mut Context) -> GameResult<Self> {
        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE)?;

        let quas = Orb::new(ctx, OrbType::Quas)?;
        let wex = Orb::new(ctx, OrbType::Wex)?;
        let exort = Orb::new(ctx, OrbType::Exort)?;
        let invoke = Orb::new(ctx, OrbType::Invoke)?;

        let quas_image = quas.orb_image.clone();
        let wex_image = wex.orb_image.clone();
        let exort_image = exort.orb_image.clone();

        let mut orbs = HashMap::with_capacity(3);
        orbs.insert('Q', quas_image);
        orbs.insert('W', wex_image);
        orbs.insert('E', exort_image);

        let mut keybindings: HashMap<KeyCode, Orb> = HashMap::with_capacity(4);

        keybindings.insert(settings.quas_key, quas);
        keybindings.insert(settings.wex_key, wex);
        keybindings.insert(settings.exort_key, exort);
        keybindings.insert(settings.invoke_key, invoke);

        let input_buffer = InputBuffer::new(&settings);

        Ok(Self {
            game_over: false,
            objects: Vec::new(),
            input_buffer,
            last_spell_time: std::time::Duration::new(0, 0),
            speed: 0.0,
            score: 0,
            settings,
            background_image,
            keybindings,
            orbs,
        })
    }
}

impl GameState for ProMode {
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

        for (pos, key) in self.input_buffer.buffer.iter().enumerate() {
            let orb_image = self.orbs.get(&key).unwrap();

            let draw_param = self.input_buffer.draw_params[pos];

            canvas.draw(orb_image, draw_param);
        }
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
