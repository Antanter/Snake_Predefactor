use crate::game::Game;

use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Color};
use ggez::graphics::Canvas;
use ggez::input::keyboard::{KeyCode};
use crate::game::snake::Direction;

mod game;

struct MainState {
    game: Game,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let game = Game::start_game(25, 25);
        Ok(MainState { game })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.game.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        self.game.get_map().render_graphics(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            use Direction::*;
            let dir = match keycode {
                KeyCode::Up => Some(Up),
                KeyCode::Down => Some(Down),
                KeyCode::Left => Some(Left),
                KeyCode::Right => Some(Right),
                _ => None,
            };

            if let Some(new_dir) = dir {
                self.game.get_snake().set_dir(new_dir);
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("snake_predefactor", "antanter")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake Predefactor"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0))
        .build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
