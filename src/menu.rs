use macroquad::prelude::*;
use crate::models::StarType;

pub enum GameState {
    Menu,
    Playing,
}

enum MenuScreen {
    StarSelection,
    PlanetCount,
}

pub struct MenuSelection {
    pub star_type: StarType,
    pub planet_count: usize,
}

pub struct Menu {
    current_screen: MenuScreen,
    selected_index: usize,
    star_types: Vec<StarType>,
    selected_star: Option<StarType>,
    planet_counts: Vec<usize>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            current_screen: MenuScreen::StarSelection,
            selected_index: 0,
            star_types: vec![
                StarType::YellowDwarf,
                StarType::RedDwarf,
                StarType::BlueGiant,
                StarType::BlackHole,
            ],
            selected_star: None,
            planet_counts: (2..=9).collect(),
        }
    }

    pub fn handle_input(&mut self) -> Option<MenuSelection> {
        match self.current_screen {
            MenuScreen::StarSelection => {
                if is_key_pressed(KeyCode::Up) {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                if is_key_pressed(KeyCode::Down) {
                    if self.selected_index < self.star_types.len() - 1 {
                        self.selected_index += 1;
                    }
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    self.selected_star = Some(self.star_types[self.selected_index]);
                    self.current_screen = MenuScreen::PlanetCount;
                    self.selected_index = 4; // Default to 6 planets (index 4 in 2-9 range)
                }
            }
            MenuScreen::PlanetCount => {
                if is_key_pressed(KeyCode::Up) {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                if is_key_pressed(KeyCode::Down) {
                    if self.selected_index < self.planet_counts.len() - 1 {
                        self.selected_index += 1;
                    }
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    if let Some(star_type) = self.selected_star {
                        return Some(MenuSelection {
                            star_type,
                            planet_count: self.planet_counts[self.selected_index],
                        });
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.current_screen = MenuScreen::StarSelection;
                    self.selected_index = 0;
                    self.selected_star = None;
                }
            }
        }
        None
    }

    pub fn draw(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Title
        let title = "SOLAR ECONOMY";
        let title_size = 60.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            screen_w / 2.0 - title_dims.width / 2.0,
            screen_h / 4.0,
            title_size,
            WHITE,
        );

        match self.current_screen {
            MenuScreen::StarSelection => self.draw_star_selection(screen_w, screen_h),
            MenuScreen::PlanetCount => self.draw_planet_count(screen_w, screen_h),
        }
    }

    fn draw_star_selection(&self, screen_w: f32, screen_h: f32) {
        // Subtitle
        let subtitle = "Choose Your Star";
        let subtitle_size = 30.0;
        let subtitle_dims = measure_text(subtitle, None, subtitle_size as u16, 1.0);
        draw_text(
            subtitle,
            screen_w / 2.0 - subtitle_dims.width / 2.0,
            screen_h / 4.0 + 60.0,
            subtitle_size,
            GRAY,
        );

        // Star options
        let start_y = screen_h / 2.0 - 50.0;
        let spacing = 60.0;

        for (i, star_type) in self.star_types.iter().enumerate() {
            let y = start_y + i as f32 * spacing;
            let is_selected = i == self.selected_index;

            // Selection indicator
            if is_selected {
                draw_text(">", screen_w / 2.0 - 200.0, y, 40.0, WHITE);
            }

            // Star type name
            let text = star_type.name();
            let text_size = if is_selected { 40.0 } else { 35.0 };
            let color = if is_selected { WHITE } else { GRAY };

            draw_text(text, screen_w / 2.0 - 150.0, y, text_size, color);

            // Draw a preview circle of the star
            let preview_x = screen_w / 2.0 + 150.0;
            let preview_radius = if is_selected { 20.0 } else { 15.0 };
            draw_circle(preview_x, y - 15.0, preview_radius, star_type.color());

            // Special effect for black hole preview
            if matches!(star_type, StarType::BlackHole) {
                draw_circle_lines(
                    preview_x,
                    y - 15.0,
                    preview_radius + 3.0,
                    2.0,
                    Color::new(0.5, 0.0, 0.8, 0.8),
                );
            }
        }

        // Instructions
        let instructions = "Use UP/DOWN arrows to select, ENTER to continue";
        let inst_size = 20.0;
        let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
        draw_text(
            instructions,
            screen_w / 2.0 - inst_dims.width / 2.0,
            screen_h - 50.0,
            inst_size,
            DARKGRAY,
        );
    }

    fn draw_planet_count(&self, screen_w: f32, screen_h: f32) {
        // Subtitle with selected star
        let subtitle = if let Some(star) = self.selected_star {
            format!("{} - Choose Planet Count", star.name())
        } else {
            "Choose Planet Count".to_string()
        };
        let subtitle_size = 30.0;
        let subtitle_dims = measure_text(&subtitle, None, subtitle_size as u16, 1.0);
        draw_text(
            &subtitle,
            screen_w / 2.0 - subtitle_dims.width / 2.0,
            screen_h / 4.0 + 60.0,
            subtitle_size,
            GRAY,
        );

        // Planet count options in a grid
        let start_y = screen_h / 2.0 - 100.0;
        let spacing = 55.0;

        for (i, &count) in self.planet_counts.iter().enumerate() {
            let y = start_y + i as f32 * spacing;
            let is_selected = i == self.selected_index;

            // Selection indicator
            if is_selected {
                draw_text(">", screen_w / 2.0 - 100.0, y, 40.0, WHITE);
            }

            // Planet count text
            let text = format!("{} Planets", count);
            let text_size = if is_selected { 40.0 } else { 35.0 };
            let color = if is_selected { WHITE } else { GRAY };

            draw_text(&text, screen_w / 2.0 - 70.0, y, text_size, color);

            // Draw preview planets (mini version)
            if is_selected {
                let preview_start_x = screen_w / 2.0 + 120.0;
                let preview_y = y - 15.0;
                for j in 0..count.min(5) {
                    let x = preview_start_x + j as f32 * 20.0;
                    draw_circle(x, preview_y, 5.0, Color::new(0.5, 0.7, 1.0, 0.8));
                }
                if count > 5 {
                    draw_text("...", preview_start_x + 100.0, y, 30.0, GRAY);
                }
            }
        }

        // Instructions
        let instructions = "UP/DOWN to select, ENTER to start, ESC to go back";
        let inst_size = 20.0;
        let inst_dims = measure_text(instructions, None, inst_size as u16, 1.0);
        draw_text(
            instructions,
            screen_w / 2.0 - inst_dims.width / 2.0,
            screen_h - 50.0,
            inst_size,
            DARKGRAY,
        );
    }
}
