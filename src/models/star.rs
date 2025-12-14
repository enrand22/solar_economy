use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum StarType {
    YellowDwarf,
    RedDwarf,
    BlueGiant,
    BlackHole,
}

impl StarType {
    pub fn color(&self) -> Color {
        match self {
            StarType::YellowDwarf => YELLOW,
            StarType::RedDwarf => Color::new(0.8, 0.2, 0.1, 1.0),
            StarType::BlueGiant => Color::new(0.5, 0.7, 1.0, 1.0),
            StarType::BlackHole => Color::new(0.1, 0.0, 0.2, 1.0),
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            StarType::YellowDwarf => 30.0,
            StarType::RedDwarf => 20.0,
            StarType::BlueGiant => 50.0,
            StarType::BlackHole => 25.0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StarType::YellowDwarf => "Yellow Dwarf",
            StarType::RedDwarf => "Red Dwarf",
            StarType::BlueGiant => "Blue Giant",
            StarType::BlackHole => "Black Hole",
        }
    }
}

pub struct Star {
    pub position: Vec2,
    pub radius: f32,
    pub color: Color,
    pub star_type: StarType,
}

impl Star {
    pub fn new(position: Vec2, star_type: StarType) -> Self {
        Self {
            position,
            radius: star_type.radius(),
            color: star_type.color(),
            star_type,
        }
    }

    pub fn draw(&self, camera_offset: Vec2) {
        let screen_pos = self.position - camera_offset;
        draw_circle(screen_pos.x, screen_pos.y, self.radius, self.color);

        // Black hole special effect - draw event horizon
        if matches!(self.star_type, StarType::BlackHole) {
            draw_circle_lines(screen_pos.x, screen_pos.y, self.radius + 5.0, 2.0, Color::new(0.5, 0.0, 0.8, 0.8));
        }
    }
}
