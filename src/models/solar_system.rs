use macroquad::prelude::*;
use super::{Star, Planet};
use super::star::StarType;

pub struct SolarSystem {
    pub star: Star,
    pub planets: Vec<Planet>,
}

impl SolarSystem {
    pub fn new(center: Vec2, star_type: StarType, planet_count: usize) -> Self {
        // First generate planets to find the biggest one
        let (planets, max_planet_radius) = Self::generate_random_planets(planet_count);

        // Ensure star is at least 3x the biggest planet
        let min_star_radius = max_planet_radius * 3.0;
        let mut star = Star::new(center, star_type);
        if star.radius < min_star_radius {
            star.radius = min_star_radius;
        }

        Self { star, planets }
    }

    fn generate_random_planets(count: usize) -> (Vec<Planet>, f32) {
        let mut planets = Vec::new();
        let mut max_planet_radius: f32 = 0.0;

        // Much larger solar system - make it huge!
        let min_orbital_radius = 200.0;
        let max_orbital_radius = 2000.0; // Increased from 320 to 2000

        // Calculate spacing to ensure no overlaps
        let spacing = (max_orbital_radius - min_orbital_radius) / count as f32;

        for i in 0..count {
            // Calculate orbital radius with spacing to prevent overlaps
            let base_orbital_radius = min_orbital_radius + (i as f32 * spacing);

            // Add random variation to orbital radius (±20% of spacing)
            let variation = rand::gen_range(-spacing * 0.2, spacing * 0.2);
            let orbital_radius = (base_orbital_radius + variation).max(min_orbital_radius);

            // Random orbital speed (slower for outer planets, faster for inner)
            let base_speed = 0.5 - (i as f32 / count as f32) * 0.35; // Slower overall for bigger system
            let orbital_speed = base_speed + rand::gen_range(-0.05, 0.05);

            // Random planet radius (between 8 and 35 pixels) - bigger planets for bigger system
            let radius = rand::gen_range(8.0, 35.0);
            max_planet_radius = max_planet_radius.max(radius);

            // Random color
            let color = Color::new(
                rand::gen_range(0.3, 1.0),
                rand::gen_range(0.3, 1.0),
                rand::gen_range(0.3, 1.0),
                1.0,
            );

            // Random initial angle
            let initial_angle = rand::gen_range(0.0, std::f32::consts::TAU);

            planets.push(Planet::new(
                orbital_radius,
                orbital_speed,
                radius,
                color,
                initial_angle,
            ));
        }

        (planets, max_planet_radius)
    }

    pub fn update(&mut self, dt: f32) {
        for planet in &mut self.planets {
            planet.update(dt);
        }
    }

    pub fn draw(&self, camera_offset: Vec2) {
        self.star.draw(camera_offset);
        for planet in &self.planets {
            planet.draw(self.star.position, camera_offset);
        }
    }
}
