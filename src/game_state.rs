use crate::settings::Settings;
use crate::spells::Spell;

use std::{env, path};

use ggez::{
    event::{self, EventHandler, EventLoop},
    glam::*,
    graphics::{self, Color, Drawable, Rect},
    input::keyboard::{KeyCode, KeyInput},
    winit::event_loop,
    Context, GameResult,
};

enum Transition {
    None,
    Menu,
    Game,
}

pub trait GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult<Transition>;
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult<Transition>;
}

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

impl GameState for MainState {
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
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
            println!("Game is over.");
            return Ok(Transition::Menu);
        } else {
            for object in self.objects.iter_mut() {
                object.position.y += object.speed;
                if object.position.y > self.settings.window_height {
                    self.game_over = true;
                    return Ok(Transition::Menu);
                }
            }
        }
        Ok(Transition::None)
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
    ) -> GameResult<Transition> {
        if !self.game_over {
            match keycode.keycode.unwrap() {
                KeyCode::Escape => return Ok(Transition::Menu),

                KeyCode::Q => {
                    self.update_buffer('Q');
                    return Ok(Transition::None);
                }

                KeyCode::W => {
                    self.update_buffer('W');
                    return Ok(Transition::None);
                }

                KeyCode::E => {
                    self.update_buffer('E');

                    return Ok(Transition::None);
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

                        return Ok(Transition::None);
                    } else {
                        self.game_over = true;

                        return Ok(Transition::Menu);
                    }
                }
                _ => Ok(Transition::None),
            }
        } else {
            Ok(Transition::Menu)
        }
    }
}

pub struct MenuState {
    pub start_game_position: Vec2,
    pub start_game_dimensions: Rect,
}

impl MenuState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let x = (settings.window_width / 2.0) - 50.0;
        let y = (settings.window_height / 2.0) - 20.0;

        let text = String::from("Start Game");

        // Use ctx to get the text dimensions
        let play_button = graphics::Text::new(&text).set_scale(40.0).clone();
        let start_game_dimensions = play_button.dimensions(ctx).unwrap();

        Ok(Self {
            start_game_position: Vec2::new(x, y),
            start_game_dimensions,
        })
    }
}

impl GameState for MenuState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let text = String::from("Start Game");

        // That drove me mad untill I found this:
        // https://github.com/ggez/ggez/issues/659
        let play_button = graphics::Text::new(&text).set_scale(40.0).clone();
        canvas.draw(&play_button, self.start_game_position);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult<Transition> {
        if button == ggez::event::MouseButton::Left {
            let cursor_location = Vec2::new(x, y);
            println!("{:?}", cursor_location);

            let start_game_rect = Rect::new(
                self.start_game_position.x,
                self.start_game_position.y,
                self.start_game_dimensions.w,
                self.start_game_dimensions.h,
            );

            if start_game_rect.contains(cursor_location) {
                return Ok(Transition::Game);
            }
        }
        Ok(Transition::None)
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }
}

pub struct StateMachine {
    current_state: Box<dyn GameState>,
    settings: Settings,
}

impl StateMachine {
    pub fn new(initial_state: Box<dyn GameState>, settings: Settings) -> Self {
        Self {
            current_state: initial_state,
            settings,
        }
    }

    fn switch_state(&mut self, new_state: Box<dyn GameState>) {
        self.current_state = new_state;
    }
}

impl EventHandler for StateMachine {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let transition = self.current_state.update(ctx)?;

        match transition {
            Transition::None => Ok(()),
            Transition::Menu => {
                Ok(self.switch_state(Box::new(MenuState::new(ctx, &self.settings.clone())?)))
            }
            Transition::Game => {
                Ok(self.switch_state(Box::new(MainState::new(self.settings.clone())?)))
            }
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(self.current_state.draw(ctx)?)
    }
    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let transition = self
            .current_state
            .mouse_button_up_event(ctx, button, x, y)?;
        match transition {
            Transition::None => {}
            Transition::Menu => {
                self.switch_state(Box::new(MenuState::new(ctx, &self.settings.clone())?));
            }
            Transition::Game => {
                self.switch_state(Box::new(MainState::new(self.settings.clone())?));
            }
        };

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        let transition = self.current_state.key_down_event(ctx, keycode, _repeat)?;
        match transition {
            Transition::None => {}
            Transition::Menu => {
                self.switch_state(Box::new(MenuState::new(ctx, &self.settings.clone())?));
            }
            Transition::Game => {
                self.switch_state(Box::new(MainState::new(self.settings.clone())?));
            }
        };

        Ok(())
    }
}
