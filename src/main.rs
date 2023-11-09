use ggez::glam::vec2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Image, DrawParam};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Gremory", "hambloko@gmail.com")
        .build()
        .expect("Context could not be built");


    let main_state = MainState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, main_state);
}

struct MainState {
    tile_map: Image,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let tile_map = Image::from_path(ctx, "/tiles.png").unwrap();
        MainState {
            tile_map
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
        // canvas.draw(&self.tile_map, DrawParam::default());
        canvas.finish(ctx)
    }
}