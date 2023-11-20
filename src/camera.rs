// TODO, put map's render function here, have camera follow character

use ggez::glam::*;
use ggez::graphics::{Canvas, Color, DrawParam};
use ggez::Context;

use crate::map::*;
use crate::prelude::*;
use crate::tile::*;

const FWIDTH: f32 = WINDOW_WIDTH / 2.;
const FHEIGHT: f32 = WINDOW_HEIGHT / 2.;

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn on_player_move(&mut self) {
    //     self.left = self.player_position.x - FWIDTH;
    //     self.right = self.player_position.x + FWIDTH;
    //     self.top = self.player_position.y - FHEIGHT;
    //     self.bottom = self.player_position.y + FHEIGHT;
    // }

    pub fn render(
        &self,
        ctx: &mut Context,
        canvas: &mut Canvas,
        map: &Vec<TileType>,
        player: &Vec2,
    ) {
        let tiles = Tile::new(ctx);
        let px = player.x * 32.;
        let py = player.y * 32.;
        let grey = Color::from_rgb(79, 79, 79);
        let dark_grey = Color::from_rgb(26, 26, 26);
        let player_color = Color::from_rgb(115, 77, 227);

        for y in 0..NUM_Y {
            for x in 0..NUM_X {
                let idx = idx(x, y);
                let fx = (x * 32) as f32;
                let yx = (y * 32) as f32;
                match map[idx] {
                    TileType::Floor => canvas.draw(
                        &tiles.tile_map,
                        DrawParam::default()
                            .dest(vec2(fx, yx))
                            .z(0)
                            .color(dark_grey)
                            .src(*tiles.get_tile(TileType::Floor)),
                    ),
                    TileType::Wall => canvas.draw(
                        &tiles.tile_map,
                        DrawParam::default()
                            .dest(vec2(fx, yx))
                            .z(0)
                            .color(grey)
                            .src(*tiles.get_tile(TileType::Wall)),
                    ),
                    _ => (),
                }
            }
        }

        canvas.draw(
            &tiles.tile_map,
            DrawParam::default()
                .z(1)
                .dest(vec2(px, py))
                .color(player_color)
                .src(*tiles.get_tile(TileType::Player)),
        )
    }
}
