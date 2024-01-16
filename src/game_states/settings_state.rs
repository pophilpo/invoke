use crate::buttons::MenuButton;
use crate::settings::{Settings, BACKGROUND_IMAGE, EXORT, QUAS, WEX};
use crate::state_machine::{GameState, Transition};

use ggez::{
    glam::*,
    graphics::{self, Rect},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

pub struct SettingsState {
    settings: Settings,
    background_image: graphics::Image,
    quas: graphics::Image,
    quas_draw_param: graphics::DrawParam,
    wex: graphics::Image,
    wex_draw_param: graphics::DrawParam,
    exort: graphics::Image,
    exort_draw_param: graphics::DrawParam,
    buttons: Vec<MenuButton>,
}

impl SettingsState {
    pub fn new(ctx: &mut Context, settings: &Settings) -> GameResult<Self> {
        let background_image = graphics::Image::from_bytes(ctx, BACKGROUND_IMAGE)?;
        let quas = graphics::Image::from_bytes(ctx, QUAS)?;
        let wex = graphics::Image::from_bytes(ctx, WEX)?;
        let exort = graphics::Image::from_bytes(ctx, EXORT)?;

        let (quas_draw_param, wex_draw_param, exort_draw_param) =
            Self::calculate_orb_positions(settings, ctx);

        let mut buttons = Vec::new();

        Ok(Self {
            settings: settings.clone(),
            background_image,
            quas,
            quas_draw_param,
            wex,
            wex_draw_param,
            exort,
            exort_draw_param,
            buttons,
        })
    }

    fn calculate_orb_positions(
        settings: &Settings,
        ctx: &mut Context,
    ) -> (
        graphics::DrawParam,
        graphics::DrawParam,
        graphics::DrawParam,
    ) {
        // TODO: remove hardcoded image sizes
        let offset = settings.window_width / 50.0;
        let box_width = 77.0 * 3.0 + 50.0 * 2.0;
        let orb_position_h = settings.window_height / 2.0 - 77.0 / 2.0;
        let quas_position_w = settings.window_width / 2.0 - box_width / 2.0;
        let wex_position_w = quas_position_w + 77.0 + offset;
        let exort_position_w = wex_position_w + 77.0 + offset;

        let quas_position = Vec2::new(quas_position_w, orb_position_h);
        let wex_position = Vec2::new(wex_position_w, orb_position_h);
        let exort_position = Vec2::new(exort_position_w, orb_position_h);

        let quas_draw_param = graphics::DrawParam::new().dest(quas_position);
        let wex_draw_param = graphics::DrawParam::new().dest(wex_position);
        let exort_draw_param = graphics::DrawParam::new().dest(exort_position);

        (quas_draw_param, wex_draw_param, exort_draw_param)
    }
}

impl GameState for SettingsState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.draw(&self.background_image, self.settings.background_draw_param);
        canvas.draw(&self.quas, self.quas_draw_param);
        canvas.draw(&self.wex, self.wex_draw_param);
        canvas.draw(&self.exort, self.exort_draw_param);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<Transition> {
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
