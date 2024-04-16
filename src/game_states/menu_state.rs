use crate::assets::BACKGROUND_IMAGE;
use crate::buttons::MenuButton;
use crate::settings::Settings;
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct MenuState {
    background_image: graphics::Image,
    settings: Settings,
    buttons: Vec<MenuButton>,
}

impl MenuState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let mut buttons = Vec::new();

        let button_x = settings.window_width / 2.0;
        let play_button_y = settings.window_height / 3.0;
        let play_button = MenuButton::new(
            ctx,
            "Start Game",
            "RET",
            settings,
            button_x,
            play_button_y,
            None,
        )?;

        let pro_mode_button_y = settings.window_height / 2.5;
        let pro_mode_button = MenuButton::new(
            ctx,
            "ProMode",
            "P",
            settings,
            button_x,
            pro_mode_button_y,
            None,
        )?;

        let quit_button_y = settings.window_height / 2.14;
        let quit_button =
            MenuButton::new(ctx, "Quit", "ESC", settings, button_x, quit_button_y, None)?;

        buttons.push(play_button);
        buttons.push(pro_mode_button);
        buttons.push(quit_button);

        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE)?;

        Ok(Self {
            background_image,
            settings: settings.clone(),
            buttons,
        })
    }
}

impl GameState for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.draw(&self.background_image, self.settings.background_draw_param);

        // That drove me mad untill I found this:
        // https://github.com/ggez/ggez/issues/659
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
            let start_game_rect = Rect::new(
                self.buttons[0].position.x,
                self.buttons[0].position.y,
                self.buttons[0].dimensions.w,
                self.buttons[0].dimensions.h,
            );

            let pro_mode_rect = Rect::new(
                self.buttons[1].position.x,
                self.buttons[1].position.y,
                self.buttons[1].dimensions.w,
                self.buttons[1].dimensions.h,
            );

            let quit_game_rect = Rect::new(
                self.buttons[2].position.x,
                self.buttons[2].position.y,
                self.buttons[2].dimensions.w,
                self.buttons[2].dimensions.h,
            );

            if start_game_rect.contains(cursor_location) {
                return Ok(Transition::Game);
            }

            if pro_mode_rect.contains(cursor_location) {
                return Ok(Transition::ProMode);
            }

            if quit_game_rect.contains(cursor_location) {
                return Ok(Transition::Quit);
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
            KeyCode::Escape => return Ok(Transition::Quit),
            KeyCode::P => return Ok(Transition::ProMode),
            _ => return Ok(Transition::None),
        }
    }
}
