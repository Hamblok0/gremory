use ggez::glam::Vec2;

pub struct Player {
    pub position: Vec2,
}

impl Player {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
