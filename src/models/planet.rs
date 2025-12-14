use macroquad::prelude::*;

pub struct Planet {
    pub angle: f32,
    pub orbital_radius: f32,
    pub orbital_speed: f32,
    pub radius: f32,
    pub color: Color,
}

impl Planet {
    pub fn new(orbital_radius: f32, orbital_speed: f32, radius: f32, color: Color, initial_angle: f32) -> Self {
        Self {
            angle: initial_angle,
            orbital_radius,
            orbital_speed,
            radius,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += self.orbital_speed * dt;
    }

    pub fn position(&self, center: Vec2) -> Vec2 {
        Vec2::new(
            center.x + self.orbital_radius * self.angle.cos(),
            center.y + self.orbital_radius * self.angle.sin(),
        )
    }

    pub fn draw(&self, center: Vec2, camera_offset: Vec2) {
        let pos = self.position(center);
        let screen_pos = pos - camera_offset;
        let screen_center = center - camera_offset;

        draw_circle(screen_pos.x, screen_pos.y, self.radius, self.color);

        // Draw orbit path
        draw_circle_lines(screen_center.x, screen_center.y, self.orbital_radius, 1.0, Color::new(0.3, 0.3, 0.3, 0.5));
    }
}
