use ggez::conf::*;
use ggez::event::{self, EventHandler};
use ggez::glam::{vec2, Vec2};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};

mod camera;
mod map;
mod player;
mod tile;

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
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut map = map::Map::new();
        map.build();
        let camera = camera::Camera::new();
        let player = player::Player::new(map.player_start);
        Self {
            player,
            map,
            camera,
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
            .render(ctx, &mut canvas, &self.map.map, &self.player.position);
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        use ggez::input::keyboard::KeyCode;
        match input.keycode {
            Some(KeyCode::K) => Ok(()),
            Some(KeyCode::J) => Ok(()),
            Some(KeyCode::H) => Ok(()),
            Some(KeyCode::L) => Ok(()),
            _ => Ok(()),
        }
    }
}
