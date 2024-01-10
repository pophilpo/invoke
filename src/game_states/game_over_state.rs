use crate::buttons::MenuButton;
use crate::settings::{Settings, BACKGROUND_IMAGE};
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Color, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct GameOverState {
    background_image: graphics::Image,
    settings: Settings,
    buttons: Vec<MenuButton>,
}

impl GameOverState {
    pub fn new(ctx: &mut Context, score: usize, settings: &Settings) -> GameResult<Self> {
        let mut buttons = Vec::new();

        let game_over_text = format!("Score {}", score);

        let game_over_button_x = settings.window_width / 2.0;
        let game_over_button_y = settings.window_height / 3.0;

        let game_over_button = MenuButton::new(
            ctx,
            &game_over_text,
            "",
            settings,
            game_over_button_x,
            game_over_button_y,
        )?;

        let go_to_menu_button_x = settings.window_width / 2.5;
        let go_to_menu_button_y = settings.window_height / 2.5;
        let go_to_menu_button = MenuButton::new(
            ctx,
            "Menu",
            "ESC",
            settings,
            go_to_menu_button_x,
            go_to_menu_button_y,
        )?;

        let try_again_button_x = settings.window_width / 1.5;
        let try_again_button_y = settings.window_height / 2.5;
        let try_again_button = MenuButton::new(
            ctx,
            "Try again",
            "RET",
            settings,
            try_again_button_x,
            try_again_button_y,
        )?;

        buttons.push(game_over_button);
        buttons.push(go_to_menu_button);
        buttons.push(try_again_button);

        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE)?;

        Ok(Self {
            background_image,
            settings: settings.clone(),
            buttons,
        })
    }
}

impl GameState for GameOverState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.background_image, self.settings.background_draw_param);

        for button in &self.buttons {
            canvas.draw(&button.background, button.draw_param);
            canvas.draw(&button.text, button.text_draw_param);
            canvas.draw(&button.hint, button.hint_draw_param);
        }

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
            let go_to_menu_rect = Rect::new(
                self.buttons[1].position.x,
                self.buttons[1].position.y,
                self.buttons[1].dimensions.w,
                self.buttons[1].dimensions.h,
            );

            let try_again_rect = Rect::new(
                self.buttons[2].position.x,
                self.buttons[2].position.y,
                self.buttons[2].dimensions.w,
                self.buttons[2].dimensions.h,
            );

            if go_to_menu_rect.contains(cursor_location) {
                return Ok(Transition::Menu);
            }
            if try_again_rect.contains(cursor_location) {
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
        match keycode.keycode.unwrap() {
            KeyCode::Return => return Ok(Transition::Game),
            KeyCode::Escape => return Ok(Transition::Menu),
            _ => return Ok(Transition::None),
        }
    }
}
