use ggez::{
    event::{self, EventHandler},
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput, KeyMods},
    Context, GameResult,
};

struct MainState {
    game_over: bool,
    win: bool,
    pos_y: f32,
    circle: graphics::Mesh,
    input_buffer: String,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        Ok(Self {
            game_over: false,
            win: false,
            pos_y: 0.0,
            circle,
            input_buffer: String::from(""),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            println!("Game is over. Win {}", self.win);
            Ok(())
        } else {
            self.pos_y += 1.0;
            if self.pos_y > 800.0 {
                self.game_over = true;
            }
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.circle, Vec2::new(300.0, self.pos_y));

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
                    self.input_buffer.push_str("Q");
                }

                KeyCode::W => {
                    self.input_buffer.push_str("W");
                }

                KeyCode::R => {
                    self.input_buffer.push_str("R");

                    if self.input_buffer.ends_with("QQWR") {
                        self.win = true;
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
    let window_mode = ggez::conf::WindowMode::default().dimensions(600.0, 800.0); // Set your desired window size here

    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(window_mode);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
