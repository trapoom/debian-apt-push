use crate::config::AppConfig;
use crate::event::InputCommand;
use crate::sysinfo::{SystemMonitor, SystemStats};
use crate::ui::Ui;
use anyhow::Result;
use std::collections::VecDeque;

pub struct App {
    pub config: AppConfig,
    pub ui: Ui,
    pub sys_monitor: SystemMonitor,
    pub running: bool,
    pub command_history: VecDeque<String>,
    pub history_index: Option<usize>,
    pub cursor_pos: usize,
}

impl App {
    pub fn new(config: AppConfig) -> Result<Self> {
        let sys_monitor = SystemMonitor::new(config.battery_path.clone());
        let ui = Ui::new(config.clone());

        Ok(App {
            config,
            ui,
            sys_monitor,
            running: true,
            command_history: VecDeque::new(),
            history_index: None,
            cursor_pos: 0,
        })
    }

    pub fn update(&mut self, stats: &SystemStats) -> Result<()> {
        self.update_colors_from_status(stats);
        Ok(())
    }

    pub fn handle_input(&mut self, cmd: InputCommand) -> Result<()> {
        match cmd {
            InputCommand::AddChar(c) => {
                self.ui.state.command_input.insert(self.cursor_pos, c);
                self.cursor_pos = (self.cursor_pos + 1).min(self.ui.state.command_input.len());
            }
            InputCommand::Backspace => {
                if self.cursor_pos > 0 {
                    self.ui.state.command_input.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            InputCommand::Delete => {
                if self.cursor_pos < self.ui.state.command_input.len() {
                    self.ui.state.command_input.remove(self.cursor_pos);
                }
            }
            InputCommand::ClearInput => {
                self.ui.state.command_input.clear();
                self.cursor_pos = 0;
                self.history_index = None;
            }
            InputCommand::Execute => {
                self.execute_command()?;
            }
            InputCommand::MoveCursorLeft => {
                self.cursor_pos = self.cursor_pos.saturating_sub(1);
            }
            InputCommand::MoveCursorRight => {
                self.cursor_pos = (self.cursor_pos + 1).min(self.ui.state.command_input.len());
            }
            InputCommand::MoveCursorHome => {
                self.cursor_pos = 0;
            }
            InputCommand::MoveCursorEnd => {
                self.cursor_pos = self.ui.state.command_input.len();
            }
            InputCommand::HistoryUp => {
                if !self.command_history.is_empty() {
                    if let Some(idx) = self.history_index {
                        if idx > 0 {
                            self.history_index = Some(idx - 1);
                            self.ui.state.command_input = self.command_history[idx - 1].clone();
                            self.cursor_pos = self.ui.state.command_input.len();
                        }
                    } else {
                        self.history_index = Some(self.command_history.len() - 1);
                        self.ui.state.command_input = self.command_history[self.command_history.len() - 1].clone();
                        self.cursor_pos = self.ui.state.command_input.len();
                    }
                }
            }
            InputCommand::HistoryDown => {
                if let Some(idx) = self.history_index {
                    if idx < self.command_history.len() - 1 {
                        self.history_index = Some(idx + 1);
                        self.ui.state.command_input = self.command_history[idx + 1].clone();
                        self.cursor_pos = self.ui.state.command_input.len();
                    } else {
                        self.history_index = None;
                        self.ui.state.command_input.clear();
                        self.cursor_pos = 0;
                    }
                }
            }
            InputCommand::Autocomplete => {
                self.autocomplete_command();
            }
            InputCommand::ClearLog => {
                self.ui.state.clear_log();
            }
            InputCommand::Quit => {
                self.running = false;
            }
        }
        Ok(())
    }

    fn execute_command(&mut self) -> Result<()> {
        let cmd = self.ui.state.command_input.trim().to_string();

        if cmd.is_empty() {
            return Ok(());
        }

        self.command_history.push_back(cmd.clone());
        if self.command_history.len() > 100 {
            self.command_history.pop_front();
        }

        self.history_index = None;
        self.ui.state.add_log(format!("> {}", cmd));

        match cmd.split_whitespace().next().unwrap_or("") {
            "help" => self.cmd_help(),
            "clear" => self.ui.state.clear_log(),
            "config" => self.cmd_config(&cmd),
            "battery" => self.cmd_battery(&cmd),
            "colors" => self.cmd_colors(&cmd),
            "layout" => self.cmd_layout(&cmd),
            "status" => self.cmd_status(),
            "reset" => self.cmd_reset()?,
            "save" => self.cmd_save()?,
            "load" => self.cmd_load()?,
            "image" => self.cmd_image(&cmd),
            "echo" => {
                let msg = cmd.strip_prefix("echo ").unwrap_or("").to_string();
                self.ui.state.add_log(msg);
            }
            _ => {
                self.ui.state.add_log(format!("Unknown command: {}", cmd));
            }
        }

        self.ui.state.command_input.clear();
        self.cursor_pos = 0;
        Ok(())
    }

    fn cmd_help(&mut self) {
        let commands = vec![
            "Commands:",
            "  help              - Show this help message",
            "  clear             - Clear output log",
            "  config            - Show current configuration",
            "  battery set PATH  - Set battery path",
            "  battery reset     - Auto-detect battery",
            "  colors show       - Show available colors",
            "  colors set NAME   - Set theme color",
            "  layout            - Show layout settings",
            "  layout set KEY VAL - Modify layout",
            "  status            - Show system status",
            "  save              - Save configuration",
            "  load              - Load configuration",
            "  reset             - Reset to defaults",
            "  image load PATH   - Load image (512x512)",
            "  echo TEXT         - Echo text",
            "  Ctrl+C            - Quit",
            "  Ctrl+L            - Clear log",
        ];

        for cmd in commands {
            self.ui.state.add_log(cmd.to_string());
        }
    }

    fn cmd_config(&mut self, _cmd: &str) {
        self.ui.state.add_log(format!("=== System Monitor v{} ===", env!("CARGO_PKG_VERSION")));
        self.ui.state.add_log(format!("Battery Path: {:?}", self.config.battery_path));
        self.ui.state.add_log(format!("Update Interval: {}ms", self.config.update_interval_ms));
        self.ui.state.add_log(format!("CPU Warning: {}%", self.config.thresholds.cpu_warning));
        self.ui.state.add_log(format!("CPU Critical: {}%", self.config.thresholds.cpu_critical));
        self.ui.state.add_log(format!("RAM Warning: {}%", self.config.thresholds.ram_warning));
        self.ui.state.add_log(format!("RAM Critical: {}%", self.config.thresholds.ram_critical));
    }

    fn cmd_battery(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            self.ui.state.add_log("battery set PATH | battery reset".to_string());
            return;
        }

        match parts[1] {
            "set" => {
                if let Some(path) = parts.get(2) {
                    if std::path::Path::new(path).exists() {
                        self.config.battery_path = Some(path.to_string());
                        self.ui.state.add_log(format!("Battery path set to: {}", path));
                    } else {
                        self.ui.state.add_log(format!("Path not found: {}", path));
                    }
                }
            }
            "reset" => {
                self.config.battery_path = None;
                self.ui.state.add_log("Battery path reset (auto-detect)".to_string());
            }
            _ => {
                self.ui.state.add_log("Invalid battery command".to_string());
            }
        }
    }

    fn cmd_colors(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            self.ui.state.add_log("colors show | colors set NAME".to_string());
            return;
        }

        match parts[1] {
            "show" => {
                let colors = vec![
                    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
                    "gray", "darkgray", "lightred", "lightgreen", "lightyellow",
                ];
                self.ui.state.add_log("Available colors:".to_string());
                for color in colors {
                    self.ui.state.add_log(format!("  {}", color));
                }
            }
            "set" => {
                if let Some(theme) = parts.get(2) {
                    self.ui.state.add_log(format!("Theme set to: {}", theme));
                }
            }
            _ => {
                self.ui.state.add_log("Invalid color command".to_string());
            }
        }
    }

    fn cmd_layout(&mut self, _cmd: &str) {
        self.ui.state.add_log("=== Layout Configuration ===".to_string());
        self.ui.state.add_log(format!("Left Width: {}%", self.config.layout.left_width_percent));
        self.ui.state.add_log(format!("Right Width: {}%", self.config.layout.right_width_percent));
        self.ui.state.add_log(format!("CPU Section Height: {}", self.config.layout.cpu_section_height));
        self.ui.state.add_log(format!("RAM Section Height: {}", self.config.layout.ram_section_height));
    }

    fn cmd_status(&mut self) {
        self.ui.state.add_log("System is running normally".to_string());
    }

    fn cmd_reset(&mut self) -> Result<()> {
        AppConfig::reset_to_default()?;
        self.config = AppConfig::load()?;
        self.ui.state.add_log("Configuration reset to defaults".to_string());
        Ok(())
    }

    fn cmd_save(&mut self) -> Result<()> {
        self.config.save()?;
        self.ui.state.add_log("Configuration saved".to_string());
        Ok(())
    }

    fn cmd_load(&mut self) -> Result<()> {
        self.config = AppConfig::load()?;
        self.ui.state.add_log("Configuration loaded".to_string());
        Ok(())
    }

    fn cmd_image(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.len() < 2 {
            self.ui.state.add_log("image load PATH".to_string());
            return;
        }

        match parts[1] {
            "load" => {
                if let Some(path) = parts.get(2) {
                    if let Ok(data) = std::fs::read(path) {
                        if data.len() <= 512 * 512 * 4 {
                            self.ui.state.image_data = Some(data);
                            self.config.image_path = Some(path.to_string());
                            self.ui.state.add_log(format!("Image loaded: {}", path));
                        } else {
                            self.ui.state.add_log("Image too large (max 512x512)".to_string());
                        }
                    } else {
                        self.ui.state.add_log("Failed to load image".to_string());
                    }
                }
            }
            _ => {
                self.ui.state.add_log("Invalid image command".to_string());
            }
        }
    }

    fn autocomplete_command(&mut self) {
        let commands = vec![
            "help", "clear", "config", "battery", "colors", "layout",
            "status", "reset", "save", "load", "image", "echo",
        ];

        let input = &self.ui.state.command_input;
        for cmd in commands {
            if cmd.starts_with(input) && !input.is_empty() {
                self.ui.state.command_input = cmd.to_string();
                self.cursor_pos = self.ui.state.command_input.len();
                break;
            }
        }
    }

    fn update_colors_from_status(&mut self, _stats: &SystemStats) {}
}