use macroquad::prelude::*;

mod models;
mod menu;
mod economy;

use models::{SolarSystem, Spaceship};
use menu::{Menu, GameState};
use economy::{BUY_PRODUCT_PRICE, SELL_PRODUCT_PRICE, FUEL_PRICE, FOOD_PRICE, FUEL_BUY_AMOUNT};

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

                    // Draw and handle spaceship UI
                    let (is_near_planet, is_landed, landed_planet_idx, is_out_of_food) = if let Some(ref ship) = spaceship {
                        ship.draw(camera_offset);
                        (
                            ship.is_near_planet(&system.planets, system.star.position),
                            ship.state == models::spaceship::SpaceshipState::Landed,
                            ship.landed_planet_index,
                            ship.inventory.food == 0,
                        )
                    } else {
                        (false, false, None, false)
                    };

                    // Show blinking "Press SPACE to land" text when near a planet
                    if is_near_planet {
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

                    // Show trading UI when landed
                    if is_landed {
                        if let Some(planet_idx) = landed_planet_idx {
                            if let Some(planet) = system.planets.get(planet_idx) {
                                // Trading UI
                                let ui_x = screen_width() / 2.0 - 200.0;
                                let ui_y = screen_height() / 2.0 - 150.0;

                                // Background
                                draw_rectangle(ui_x - 10.0, ui_y - 10.0, 420.0, 280.0, Color::new(0.0, 0.0, 0.0, 0.8));

                                // Title
                                draw_text(
                                    &format!("Trading at Planet - Produces: {}", planet.product.name()),
                                    ui_x,
                                    ui_y + 20.0,
                                    20.0,
                                    YELLOW,
                                );

                                // Available items to buy
                                let mut y_offset = 50.0;
                                draw_text("Press keys to buy:", ui_x, ui_y + y_offset, 18.0, WHITE);
                                y_offset += 30.0;

                                let available_space = spaceship.as_ref().map(|s| s.inventory.available_space()).unwrap_or(0);

                                draw_text(&format!("[1] Buy {} - ${}ea", planet.product.name(), SELL_PRODUCT_PRICE), ui_x, ui_y + y_offset, 16.0, GREEN);
                                y_offset += 25.0;
                                draw_text(&format!("[2] Buy Fuel - ${}ea", FUEL_PRICE), ui_x, ui_y + y_offset, 16.0, GREEN);
                                y_offset += 25.0;
                                draw_text(&format!("[3] Buy Food - ${}ea", FOOD_PRICE), ui_x, ui_y + y_offset, 16.0, GREEN);
                                y_offset += 35.0;

                                draw_text(&format!("Available space: {}", available_space), ui_x, ui_y + y_offset, 16.0, GRAY);
                                y_offset += 25.0;
                                draw_text("Press SPACE to take off", ui_x, ui_y + y_offset, 18.0, SKYBLUE);

                                // Handle trading input
                                if let Some(ref mut ship_mut) = spaceship {
                                    if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Kp1) {
                                        // Buy planet's product
                                        if ship_mut.inventory.money >= BUY_PRODUCT_PRICE && ship_mut.inventory.available_space() >= 1 {
                                            ship_mut.inventory.add_cargo(planet.product, 1);
                                            ship_mut.inventory.money -= BUY_PRODUCT_PRICE;
                                        }
                                    }
                                    if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Kp2) {
                                        // Buy fuel
                                        let fuel_space_needed = FUEL_BUY_AMOUNT.ceil() as i32;
                                        if ship_mut.inventory.money >= FUEL_PRICE && ship_mut.inventory.available_space() >= fuel_space_needed {
                                            ship_mut.inventory.fuel += FUEL_BUY_AMOUNT;
                                            ship_mut.inventory.money -= FUEL_PRICE;
                                        }
                                    }
                                    if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Kp3) {
                                        // Buy food
                                        if ship_mut.inventory.money >= FOOD_PRICE && ship_mut.inventory.available_space() >= 1 {
                                            ship_mut.inventory.food += 1;
                                            ship_mut.inventory.money -= FOOD_PRICE;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Check for game over (out of food)
                    if is_out_of_food {
                        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));
                        let game_over_text = "GAME OVER - Out of Food!";
                        let text_size = 40.0;
                        let text_dims = measure_text(game_over_text, None, text_size as u16, 1.0);
                        draw_text(
                            game_over_text,
                            screen_width() / 2.0 - text_dims.width / 2.0,
                            screen_height() / 2.0,
                            text_size,
                            RED,
                        );
                        let restart_text = "Press ESC to return to menu";
                        let restart_dims = measure_text(restart_text, None, 20, 1.0);
                        draw_text(
                            restart_text,
                            screen_width() / 2.0 - restart_dims.width / 2.0,
                            screen_height() / 2.0 + 50.0,
                            20.0,
                            WHITE,
                        );
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

                    // Display spaceship inventory and resources
                    if let Some(ref ship) = spaceship {
                        let inv = &ship.inventory;
                        let y_start = 50.0;
                        let line_height = 20.0;

                        // Money
                        draw_text(&format!("Money: ${}", inv.money), 10.0, y_start, 18.0, GOLD);

                        // Fuel (red if low)
                        let fuel_color = if inv.fuel < 10.0 { RED } else { WHITE };
                        draw_text(&format!("Fuel: {:.1}", inv.fuel), 10.0, y_start + line_height, 18.0, fuel_color);

                        // Food (red if low)
                        let food_color = if inv.food < 5 { RED } else { WHITE };
                        draw_text(&format!("Food: {}", inv.food), 10.0, y_start + line_height * 2.0, 18.0, food_color);

                        // Cargo
                        draw_text(&format!("Cargo: {}/100", inv.total_cargo()), 10.0, y_start + line_height * 3.0, 18.0, WHITE);

                        // List cargo items
                        let mut i = 0;
                        for (product, amount) in &inv.cargo {
                            draw_text(
                                &format!("  {}: {}", product.name(), amount),
                                10.0,
                                y_start + line_height * (4.0 + i as f32),
                                16.0,
                                GRAY,
                            );
                            i += 1;
                        }
                    }

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
