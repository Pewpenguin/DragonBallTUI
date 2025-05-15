use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use tui::style::Color;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub text: String,
    pub highlight: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
    pub themes: Vec<ThemeConfig>,
    pub items_per_page: usize,
    pub show_keyboard_shortcuts: bool,
    pub enable_animations: bool,
    pub default_sort_method: String,
    pub default_sort_order: String,
    pub enable_fuzzy_search: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: ThemeColors,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            themes: vec![
                ThemeConfig {
                    name: "default".to_string(),
                    colors: ThemeColors {
                        primary: "#3498db".to_string(),
                        secondary: "#2ecc71".to_string(),
                        accent: "#e74c3c".to_string(),
                        background: "#2c3e50".to_string(),
                        text: "#ecf0f1".to_string(),
                        highlight: "#f39c12".to_string(),
                    },
                },
                ThemeConfig {
                    name: "dark".to_string(),
                    colors: ThemeColors {
                        primary: "#6c5ce7".to_string(),
                        secondary: "#00b894".to_string(),
                        accent: "#d63031".to_string(),
                        background: "#2d3436".to_string(),
                        text: "#dfe6e9".to_string(),
                        highlight: "#fdcb6e".to_string(),
                    },
                },
                ThemeConfig {
                    name: "light".to_string(),
                    colors: ThemeColors {
                        primary: "#0984e3".to_string(),
                        secondary: "#00cec9".to_string(),
                        accent: "#d63031".to_string(),
                        background: "#dfe6e9".to_string(),
                        text: "#2d3436".to_string(),
                        highlight: "#fdcb6e".to_string(),
                    },
                },
            ],
            items_per_page: 10,
            show_keyboard_shortcuts: true,
            enable_animations: true,
            default_sort_method: "number".to_string(),
            default_sort_order: "ascending".to_string(),
            enable_fuzzy_search: true,
        }
    }
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        if !path.exists() {
            let default_config = Config::default();
            default_config.save_to_file(file_path)?;
            return Ok(default_config);
        }

        let file_content = fs::read_to_string(path)?;
        serde_json::from_str(&file_content).map_err(|e| e.into())
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_content = serde_json::to_string_pretty(self)?;
        let path = Path::new(file_path);
        let mut file = fs::File::create(path)?;
        file.write_all(file_content.as_bytes())
            .map_err(|e| e.into())
    }

    pub fn get_theme_colors(&self) -> &ThemeColors {
        for theme_config in &self.themes {
            if theme_config.name == self.theme {
                return &theme_config.colors;
            }
        }
        // Return default theme if not found
        &self.themes[0].colors
    }

    pub fn hex_to_color(&self, hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Color::Reset;
        }

        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            Color::Rgb(r, g, b)
        } else {
            Color::Reset
        }
    }

    pub fn get_color(&self, color_name: &str) -> Color {
        let theme_colors = self.get_theme_colors();
        let hex = match color_name {
            "primary" => &theme_colors.primary,
            "secondary" => &theme_colors.secondary,
            "accent" => &theme_colors.accent,
            "background" => &theme_colors.background,
            "text" => &theme_colors.text,
            "highlight" => &theme_colors.highlight,
            _ => "#ffffff",
        };

        self.hex_to_color(hex)
    }
}
