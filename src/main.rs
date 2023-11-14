use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color };
use ggez::event::{self, EventHandler};
use ggez::conf::*;

mod map;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Gremory", "hambloko@gmail.com")
        .window_mode(WindowMode::default().dimensions(1920.0, 1080.0).fullscreen_type(FullscreenType::Windowed))
        .build()
        .expect("Context could not be built");


    let main_state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, main_state);
}

struct MainState {
    map: map::Map,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut map = map::Map::new(ctx);
        map.build();
        Self {
            map
        }
    }
}

impl EventHandler for MainState {
   fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.map.draw(&mut canvas);
        canvas.finish(ctx)
    }
}