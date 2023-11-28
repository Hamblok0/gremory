use ggez::conf::*;
use ggez::event::{self, EventHandler};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

mod camera;
mod map;
mod player;
mod tile;
mod colors;

mod prelude {
    pub const WINDOW_HEIGHT: f32 = 1080.;
    pub const WINDOW_WIDTH: f32 = 1920.;
    pub const NUM_X: usize = (WINDOW_WIDTH as usize) / 32;
    pub const NUM_Y: usize = (WINDOW_HEIGHT as usize) / 32;
    pub const NUM_TILES: usize = NUM_Y * NUM_X;
}

use prelude::*;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Gremory", "hambloko@gmail.com")
        .window_mode(
            WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .fullscreen_type(FullscreenType::Windowed),
        )
        .build()
        .expect("Context could not be built");

    let main_state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, main_state);
}

struct MainState {
    player: player::Player,
    map: map::Map,
    camera: camera::Camera,
    colors: colors::Colors,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut map = map::Map::new();
        map.build();
        let camera = camera::Camera::new(ctx, map.player_start);
        let player = player::Player::new(map.player_start);
        let colors = colors::Colors::new();

        Self {
            player,
            map,
            camera,
            colors,
        }
    }

    fn handle_move(&mut self, pos: &Vec2) {
        if self.map.in_bounds(pos) {
            self.player.position = *pos;
            self.camera.on_player_move(*pos);
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.camera
            .render(ctx, &mut canvas, &self.map.map, &self.player.position, &self.colors);
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        use ggez::input::keyboard::KeyCode;
        let px = self.player.position.x;
        let py = self.player.position.y;

        match input.keycode {
            Some(KeyCode::K) => {
                let new_pos = vec2(px, py - 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::J) => {
                let new_pos = vec2(px, py + 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::H) => {
                let new_pos = vec2(px - 1., py);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::L) => {
                let new_pos = vec2(px + 1., py);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::Y) => {
                let new_pos = vec2(px - 1., py - 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::U) => {
                let new_pos = vec2(px + 1., py - 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::B) => {
                let new_pos = vec2(px - 1., py + 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            Some(KeyCode::N) => {
                let new_pos = vec2(px + 1., py + 1.);
                self.handle_move(&new_pos);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
