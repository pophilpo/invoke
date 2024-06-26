use crate::game_states::{
    game_over_pro_mode_state::GameOverProState, game_over_state::GameOverState,
    menu_state::MenuState, play_state::MainState, pro_mode_state::ProMode,
};
use crate::settings::Settings;

use ggez::{event::EventHandler, glam::*, input::keyboard::KeyInput, Context, GameResult};

pub enum Transition {
    None,
    Menu,
    Game,
    ProMode,
    GameOver { score: usize },
    GameOverPro { score: usize, info: Option<String> },
    Quit,
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
                Ok(self.switch_state(Box::new(MainState::new(self.settings.clone(), ctx)?)))
            }
            Transition::ProMode => {
                Ok(self.switch_state(Box::new(ProMode::new(self.settings.clone(), ctx)?)))
            }

            Transition::GameOver { score } => Ok(self.switch_state(Box::new(GameOverState::new(
                ctx,
                score,
                &self.settings.clone(),
            )?))),

            Transition::GameOverPro { score, info } => Ok(self.switch_state(Box::new(
                GameOverProState::new(ctx, score, &self.settings.clone(), info)?,
            ))),

            Transition::Quit => Ok(ctx.request_quit()),
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
                self.switch_state(Box::new(MainState::new(self.settings.clone(), ctx)?));
            }
            Transition::ProMode => {
                self.switch_state(Box::new(ProMode::new(self.settings.clone(), ctx)?));
            }

            Transition::GameOver { score } => {
                self.switch_state(Box::new(GameOverState::new(
                    ctx,
                    score,
                    &self.settings.clone(),
                )?));
            }

            Transition::GameOverPro { score, info } => {
                self.switch_state(Box::new(GameOverProState::new(
                    ctx,
                    score,
                    &self.settings.clone(),
                    info,
                )?));
            }

            Transition::Quit => ctx.request_quit(),
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
                self.switch_state(Box::new(MainState::new(self.settings.clone(), ctx)?));
            }

            Transition::ProMode => {
                self.switch_state(Box::new(ProMode::new(self.settings.clone(), ctx)?));
            }

            Transition::GameOver { score } => {
                self.switch_state(Box::new(GameOverState::new(
                    ctx,
                    score,
                    &self.settings.clone(),
                )?));
            }

            Transition::GameOverPro { score, info } => {
                self.switch_state(Box::new(GameOverProState::new(
                    ctx,
                    score,
                    &self.settings.clone(),
                    info,
                )?));
            }

            Transition::Quit => ctx.request_quit(),
        };

        Ok(())
    }
}
