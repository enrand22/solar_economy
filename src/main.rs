use macroquad::prelude::*;

mod models;
mod menu;

use models::{SolarSystem, Spaceship};
use menu::{Menu, GameState};

#[macroquad::main("Solar Economy")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut menu = Menu::new();
    let mut solar_system: Option<SolarSystem> = None;
    let mut spaceship: Option<Spaceship> = None;

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Menu => {
                menu.draw();

                if let Some(selection) = menu.handle_input() {
                    // Place star at world origin (not screen center)
                    let star_center = Vec2::new(0.0, 0.0);
                    solar_system = Some(SolarSystem::new(
                        star_center,
                        selection.star_type,
                        selection.planet_count,
                    ));

                    // Create spaceship near the star (starting position in world space)
                    let spaceship_pos = Vec2::new(300.0, 0.0); // Start to the right of the star
                    spaceship = Some(Spaceship::new(spaceship_pos));

                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if let Some(ref mut system) = solar_system {
                    let dt = get_frame_time();
                    let time = get_time();

                    // Update solar system
                    system.update(dt);

                    // Update and calculate camera
                    let camera_offset = if let Some(ref mut ship) = spaceship {
                        ship.handle_input(&system.planets, system.star.position);
                        ship.update(dt, &system.planets, system.star.position);

                        // Camera follows spaceship - center ship on screen
                        Vec2::new(
                            ship.position.x - screen_width() / 2.0,
                            ship.position.y - screen_height() / 2.0,
                        )
                    } else {
                        Vec2::ZERO
                    };

                    // Draw everything with camera offset
                    system.draw(camera_offset);

                    if let Some(ref ship) = spaceship {
                        ship.draw(camera_offset);

                        // Show blinking "Press SPACE to land" text when near a planet
                        if ship.is_near_planet(&system.planets, system.star.position) {
                            // Blink text by using sine wave
                            let alpha = ((time * 3.0).sin() * 0.5 + 0.5) as f32;
                            let blink_color = Color::new(1.0, 1.0, 0.0, alpha);

                            let text = "Press SPACE to land";
                            let text_size = 25.0;
                            let text_dims = measure_text(text, None, text_size as u16, 1.0);
                            draw_text(
                                text,
                                screen_width() / 2.0 - text_dims.width / 2.0,
                                screen_height() - 100.0,
                                text_size,
                                blink_color,
                            );
                        }

                        // Show status when landed
                        use models::spaceship::SpaceshipState;
                        if ship.state == SpaceshipState::Landed {
                            let text = "Press SPACE to take off";
                            let text_size = 20.0;
                            let text_dims = measure_text(text, None, text_size as u16, 1.0);
                            draw_text(
                                text,
                                screen_width() / 2.0 - text_dims.width / 2.0,
                                screen_height() - 100.0,
                                text_size,
                                GREEN,
                            );
                        }
                    }

                    // Display info (UI elements stay in screen space)
                    let star_name = system.star.star_type.name();
                    let planet_count = system.planets.len();
                    draw_text(
                        &format!("Solar Economy - {} - {} Planets", star_name, planet_count),
                        10.0,
                        20.0,
                        20.0,
                        WHITE,
                    );
                    draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, WHITE);
                    draw_text("Controls: WASD/Arrows to move, SPACE to land/takeoff", 10.0, 60.0, 20.0, GRAY);

                    // Allow returning to menu with ESC
                    if is_key_pressed(KeyCode::Escape) {
                        game_state = GameState::Menu;
                        menu = Menu::new();
                        spaceship = None;
                    }
                }
            }
        }

        next_frame().await
    }
}
