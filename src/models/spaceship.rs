use macroquad::prelude::*;
use super::Planet;
use crate::economy::{Inventory, FUEL_CONSUMPTION_PER_SECOND, FOOD_CONSUMPTION_INTERVAL, FOOD_CONSUMED_PER_INTERVAL};

#[derive(PartialEq, Clone, Copy)]
pub enum SpaceshipState {
    Flying,
    Landing,
    Landed,
    TakingOff,
}

pub struct Spaceship {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub size: f32,
    pub base_size: f32,
    pub speed: f32,
    pub state: SpaceshipState,
    pub landed_planet_index: Option<usize>,
    pub animation_progress: f32,
    pub inventory: Inventory,
    pub food_timer: f32, // Timer for food consumption
}

impl Spaceship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            rotation: 0.0,
            size: 15.0,
            base_size: 15.0,
            speed: 100.0,
            state: SpaceshipState::Flying,
            landed_planet_index: None,
            animation_progress: 0.0,
            inventory: Inventory::new(),
            food_timer: 0.0,
        }
    }

    pub fn handle_input(&mut self, planets: &[Planet], star_position: Vec2) {
        match self.state {
            SpaceshipState::Flying => {
                let mut direction = Vec2::ZERO;

                // WASD controls
                if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                    direction.y -= 1.0;
                }
                if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                    direction.y += 1.0;
                }
                if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                    direction.x -= 1.0;
                }
                if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                    direction.x += 1.0;
                }

                // Normalize direction to prevent faster diagonal movement
                if direction.length() > 0.0 {
                    direction = direction.normalize();
                }

                self.velocity = direction * self.speed;

                // Update rotation to face movement direction
                if direction.length() > 0.0 {
                    self.rotation = direction.y.atan2(direction.x);
                }

                // Check for spacebar to initiate landing
                if is_key_pressed(KeyCode::Space) {
                    if let Some(planet_idx) = self.find_nearby_planet(planets, star_position) {
                        self.state = SpaceshipState::Landing;
                        self.landed_planet_index = Some(planet_idx);
                        self.animation_progress = 0.0;
                        self.velocity = Vec2::ZERO;
                    }
                }
            }
            SpaceshipState::Landed => {
                // Check for spacebar to initiate takeoff
                if is_key_pressed(KeyCode::Space) {
                    self.state = SpaceshipState::TakingOff;
                    self.animation_progress = 0.0;
                }
            }
            _ => {}
        }
    }

    pub fn update(&mut self, dt: f32, planets: &[Planet], star_position: Vec2) {
        // Food consumption timer
        self.food_timer += dt;
        if self.food_timer >= FOOD_CONSUMPTION_INTERVAL {
            self.food_timer = 0.0;
            match self.state {
                SpaceshipState::Flying => {
                    self.inventory.food = (self.inventory.food - FOOD_CONSUMED_PER_INTERVAL).max(0);
                }
                _ => {}
            }
        }

        match self.state {
            SpaceshipState::Flying => {
                // Only move if we have fuel
                if self.inventory.fuel > 0.0 && self.velocity.length() > 0.0 {
                    self.position += self.velocity * dt;

                    // Consume fuel when moving
                    let fuel_consumed = FUEL_CONSUMPTION_PER_SECOND * dt;
                    self.inventory.fuel = (self.inventory.fuel - fuel_consumed).max(0.0);
                } else if self.inventory.fuel == 0.0 {
                    // No fuel - can't move
                    self.velocity = Vec2::ZERO;
                }
            }
            SpaceshipState::Landing => {
                // Animate size decrease
                self.animation_progress += dt * 2.0; // 0.5 seconds to land
                if self.animation_progress >= 1.0 {
                    self.animation_progress = 1.0;
                    self.state = SpaceshipState::Landed;

                    // Auto-sell cargo when landing completes
                    if let Some(planet_idx) = self.landed_planet_index {
                        if let Some(planet) = planets.get(planet_idx) {
                            self.inventory.sell_all_cargo(planet.product);
                        }
                    }
                }
                self.size = self.base_size * (1.0 - self.animation_progress * 0.7); // Shrink to 30% size

                // Follow planet during landing
                if let Some(planet_idx) = self.landed_planet_index {
                    if let Some(planet) = planets.get(planet_idx) {
                        self.position = planet.position(star_position);
                    }
                }
            }
            SpaceshipState::Landed => {
                // Follow planet position
                if let Some(planet_idx) = self.landed_planet_index {
                    if let Some(planet) = planets.get(planet_idx) {
                        self.position = planet.position(star_position);
                    }
                }
            }
            SpaceshipState::TakingOff => {
                // Animate size increase
                self.animation_progress += dt * 2.0; // 0.5 seconds to take off
                if self.animation_progress >= 1.0 {
                    self.animation_progress = 1.0;
                    self.state = SpaceshipState::Flying;
                    self.landed_planet_index = None;
                }
                self.size = self.base_size * (0.3 + self.animation_progress * 0.7); // Grow back to full size

                // Follow planet during takeoff
                if let Some(planet_idx) = self.landed_planet_index {
                    if let Some(planet) = planets.get(planet_idx) {
                        self.position = planet.position(star_position);
                    }
                }
            }
        }
    }

    pub fn find_nearby_planet(&self, planets: &[Planet], star_position: Vec2) -> Option<usize> {
        let proximity_threshold = 20.0;

        for (i, planet) in planets.iter().enumerate() {
            let planet_pos = planet.position(star_position);
            let distance = (self.position - planet_pos).length();

            if distance < proximity_threshold + planet.radius {
                return Some(i);
            }
        }

        None
    }

    pub fn is_near_planet(&self, planets: &[Planet], star_position: Vec2) -> bool {
        self.state == SpaceshipState::Flying && self.find_nearby_planet(planets, star_position).is_some()
    }

    pub fn draw(&self, camera_offset: Vec2) {
        // Draw a triangle pointing in the direction of rotation
        let angle = self.rotation;
        let screen_pos = self.position - camera_offset;

        // Calculate triangle vertices relative to rotation
        let tip = Vec2::new(
            screen_pos.x + (self.size * angle.cos()),
            screen_pos.y + (self.size * angle.sin()),
        );

        let left = Vec2::new(
            screen_pos.x + (self.size * 0.6 * (angle + 2.5).cos()),
            screen_pos.y + (self.size * 0.6 * (angle + 2.5).sin()),
        );

        let right = Vec2::new(
            screen_pos.x + (self.size * 0.6 * (angle - 2.5).cos()),
            screen_pos.y + (self.size * 0.6 * (angle - 2.5).sin()),
        );

        // Draw the triangle
        draw_triangle(tip, left, right, WHITE);

        // Draw outline for better visibility
        draw_line(tip.x, tip.y, left.x, left.y, 2.0, SKYBLUE);
        draw_line(left.x, left.y, right.x, right.y, 2.0, SKYBLUE);
        draw_line(right.x, right.y, tip.x, tip.y, 2.0, SKYBLUE);
    }
}
