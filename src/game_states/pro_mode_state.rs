use crate::assets::BACKGROUND_IMAGE;
use crate::input_buffer::InputBuffer;
use crate::orbs::{Orb, OrbType};
use crate::settings::Settings;
use crate::spells::Spell;
use crate::state_machine::{GameState, Transition};

use std::collections::{HashMap, HashSet, VecDeque};

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
    required_key_presses: usize,
    current_key_presses: usize,
    next_spell: Spell,
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

        let required_key_presses = 4;
        let current_key_presses = 0;

        let next_spell = Spell::new(ctx, 0.0, &settings);
        let initial_spell = Spell::new(ctx, 0.3, &settings);
        let objects = vec![initial_spell];

        Ok(Self {
            game_over: false,
            objects,
            input_buffer,
            last_spell_time: std::time::Duration::new(0, 0),
            speed: 0.3,
            score: 0,
            settings,
            background_image,
            keybindings,
            orbs,
            required_key_presses,
            current_key_presses,
            next_spell,
        })
    }

    fn buffer_transition_steps(left: &Vec<char>, right: &Vec<char>) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let right_sorted = {
            let mut tmp = right.clone();
            tmp.sort_unstable();
            tmp
        };

        queue.push_back((left.clone(), 0));
        visited.insert(left.clone());

        while let Some((current, steps)) = queue.pop_front() {
            if {
                let mut tmp = current.clone();
                tmp.sort_unstable();
                tmp
            } == right_sorted
            {
                return steps;
            }

            for &key in ['Q', 'W', 'E'].iter() {
                let mut new_config = current[1..].to_vec(); // Remove the first character, shift left
                new_config.push(key); // Append the key to the end

                if visited.insert(new_config.clone()) {
                    queue.push_back((new_config.clone(), steps + 1));
                    // Debug: New configuration being enqueued
                }
            }
        }

        0
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
        if self.last_spell_time > std::time::Duration::new(2, 0) || self.objects.is_empty() {
            self.last_spell_time = std::time::Duration::new(0, 0);
            self.speed += 0.3;
            self.next_spell = Spell::new(ctx, self.speed, &self.settings);
        }

        if self.game_over {
            return Ok(Transition::GameOverPro {
                score: self.score,
                info: None,
            });
        } else {
            for object in self.objects.iter_mut() {
                object.position.y += self.speed;
                if object.position.y > self.settings.window_height {
                    self.game_over = true;
                    return Ok(Transition::GameOverPro {
                        score: self.score,
                        info: None,
                    });
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
            let orb_image = self.orbs.get(key).unwrap();

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
                    self.current_key_presses += 1;
                    println!("Current key presses {}", self.current_key_presses);
                    if let Some(orb) = self.keybindings.get(&key) {
                        let spell_cast = self.input_buffer.update_buffer(orb);

                        match spell_cast {
                            None => return Ok(Transition::None),
                            Some(cast) => {
                                // Default set to 3
                                if self.current_key_presses != self.required_key_presses {
                                    let info_text = format!(
                                        "pressed: {}; Required: {}",
                                        self.current_key_presses, self.required_key_presses
                                    );

                                    return Ok(Transition::GameOverPro {
                                        score: self.score,
                                        info: Some(info_text),
                                    });
                                }

                                let mut sorted_cast = cast.clone();
                                sorted_cast.sort_unstable();

                                if sorted_cast == self.objects[0].cast {
                                    self.score += 1;
                                    self.objects.remove(0);

                                    self.objects.push(self.next_spell.clone());

                                    println!(
                                        "Counting from {:?} --- > {:?}",
                                        &cast, &self.objects[0].cast
                                    );
                                    self.required_key_presses = ProMode::buffer_transition_steps(
                                        &cast,
                                        &self.objects[0].cast,
                                    ) + 1;
                                    println!("Req Press {}", self.required_key_presses);
                                    self.current_key_presses = 0;

                                    return Ok(Transition::None);
                                } else {
                                    self.game_over = true;

                                    return Ok(Transition::GameOverPro {
                                        score: self.score,
                                        info: None,
                                    });
                                }
                            }
                        }
                    }
                    Ok(Transition::None)
                }
            }
        } else {
            Ok(Transition::GameOverPro {
                score: self.score,
                info: None,
            })
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_buffer_transition_steps() {
        let left = vec!['Q', 'W', 'E'];
        let right = vec!['W', 'W', 'W'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 3);

        let left = vec!['W', 'W', 'W'];
        let right = vec!['Q', 'W', 'W'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 1);

        let left = vec!['Q', 'W', 'W'];
        let right = vec!['Q', 'W', 'E'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 2);

        let left = vec!['Q', 'Q', 'W'];
        let right = vec!['Q', 'Q', 'E'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 3);

        let left = vec!['W', 'Q', 'Q'];
        let right = vec!['Q', 'Q', 'E'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 1);

        let left = vec!['Q', 'W', 'E'];
        let right = vec!['E', 'E', 'E'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 2);

        let left = vec!['Q', 'W', 'E'];
        let right = vec!['Q', 'W', 'W'];

        let steps = ProMode::buffer_transition_steps(&left, &right);
        assert_eq!(steps, 3);
    }
}
