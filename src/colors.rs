use ggez::graphics::Color;

pub struct Colors {
    pub grey: Color,
    pub dark_grey: Color,
    pub player_color: Color,
}

impl Colors {
    pub fn new() -> Self {
        Self {
            grey: Color::from_rgb(79, 79, 79),
            dark_grey: Color::from_rgb(26, 26, 26),
            player_color: Color::from_rgb(115, 77, 227)
        }
    }
}