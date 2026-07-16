use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorConfig {
    pub cpu_border: String,
    pub ram_border: String,
    pub bat_border: String,
    pub process_border: String,
    pub input_border: String,
    pub image_border: String,
    pub text_normal: String,
    pub text_warning: String,
    pub text_critical: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig {
            cpu_border: "Cyan".to_string(),
            ram_border: "Green".to_string(),
            bat_border: "Yellow".to_string(),
            process_border: "White".to_string(),
            input_border: "Magenta".to_string(),
            image_border: "Blue".to_string(),
            text_normal: "White".to_string(),
            text_warning: "Yellow".to_string(),
            text_critical: "Red".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub top_left_height_percent: u16,
    pub top_right_height_percent: u16,
    pub left_width_percent: u16,
    pub right_width_percent: u16,
    pub cpu_section_height: u16,
    pub ram_section_height: u16,
    pub bat_section_height: u16,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            top_left_height_percent: 50,
            top_right_height_percent: 50,
            left_width_percent: 60,
            right_width_percent: 40,
            cpu_section_height: 10,
            ram_section_height: 10,
            bat_section_height: 8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub size: u16,
    pub family: String,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            size: 10,
            family: "monospace".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    pub cpu_warning: u8,
    pub cpu_critical: u8,
    pub ram_warning: u8,
    pub ram_critical: u8,
    pub temp_warning: u16,
    pub temp_critical: u16,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        ThresholdConfig {
            cpu_warning: 60,
            cpu_critical: 85,
            ram_warning: 70,
            ram_critical: 90,
            temp_warning: 70,
            temp_critical: 90,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub colors: ColorConfig,
    pub layout: LayoutConfig,
    pub font: FontConfig,
    pub thresholds: ThresholdConfig,
    pub battery_path: Option<String>,
    pub update_interval_ms: u64,
    pub image_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            colors: ColorConfig::default(),
            layout: LayoutConfig::default(),
            font: FontConfig::default(),
            thresholds: ThresholdConfig::default(),
            battery_path: None,
            update_interval_ms: 1000,
            image_path: None,
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "monitor", "monitor")
            .context("Failed to determine project directories")?;
        let config_dir = proj_dirs.config_dir().to_path_buf();
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_file = Self::config_file()?;
        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            toml::from_str(&content).context("Failed to parse config file")
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_file = Self::config_file()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_file, content)?;
        Ok(())
    }

    pub fn reset_to_default() -> Result<()> {
        Self::default().save()
    }
}

#[allow(dead_code)]
pub fn parse_color(color_str: &str) -> ratatui::style::Color {
    match color_str.to_lowercase().as_str() {
        "black" => ratatui::style::Color::Black,
        "red" => ratatui::style::Color::Red,
        "green" => ratatui::style::Color::Green,
        "yellow" => ratatui::style::Color::Yellow,
        "blue" => ratatui::style::Color::Blue,
        "magenta" => ratatui::style::Color::Magenta,
        "cyan" => ratatui::style::Color::Cyan,
        "white" => ratatui::style::Color::White,
        "gray" => ratatui::style::Color::Gray,
        "darkgray" => ratatui::style::Color::DarkGray,
        "lightred" => ratatui::style::Color::LightRed,
        "lightgreen" => ratatui::style::Color::LightGreen,
        "lightyellow" => ratatui::style::Color::LightYellow,
        "lightblue" => ratatui::style::Color::LightBlue,
        "lightmagenta" => ratatui::style::Color::LightMagenta,
        "lightcyan" => ratatui::style::Color::LightCyan,
        _ => ratatui::style::Color::White,
    }
}