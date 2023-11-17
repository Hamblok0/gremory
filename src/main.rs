use ggez::{Context, ContextBuilder, GameResult };
use ggez::graphics::{self, Color };

use ggez::event::{self, EventHandler};
use ggez::conf::*;

mod map;
mod tile;
mod camera;

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
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT).fullscreen_type(FullscreenType::Windowed))
        .build()
        .expect("Context could not be built");


    let main_state = MainState::new(&mut ctx);

    event::run(ctx, event_loop, main_state);
}

struct MainState {
    map: map::Map,
    tiles: tile::Tile
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let tiles = tile::Tile::new(ctx);
        let mut map = map::Map::new();
        map.build();

        Self {
            map,
            tiles
        }
    }
}

impl EventHandler for MainState {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // self.map.draw(&mut canvas);
        canvas.finish(ctx)
    }
}