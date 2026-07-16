use crate::config::AppConfig;
use crate::sysinfo::SystemStats;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct UiState {
    pub command_input: String,
    pub image_data: Option<Vec<u8>>,
    pub logs: Vec<String>,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            command_input: String::new(),
            image_data: None,
            logs: Vec::new(),
        }
    }

    pub fn add_log(&mut self, log: String) {
        self.logs.push(log);
    }

    pub fn clear_log(&mut self) {
        self.logs.clear();
    }
}

pub struct Ui {
    pub config: AppConfig,
    pub state: UiState,
}

impl Ui {
    pub fn new(config: AppConfig) -> Self {
        Ui {
            config,
            state: UiState::new(),
        }
    }

    pub fn draw(&self, f: &mut Frame, stats: &SystemStats) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(self.config.layout.left_width_percent),
                Constraint::Percentage(self.config.layout.right_width_percent),
            ])
            .split(f.area());

        self.draw_left_panel(f, chunks[0], stats);
        self.draw_right_panel(f, chunks[1], stats);
    }

    fn draw_left_panel(&self, f: &mut Frame, area: Rect, stats: &SystemStats) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.config.layout.cpu_section_height),
                Constraint::Length(self.config.layout.ram_section_height),
                Constraint::Length(self.config.layout.bat_section_height),
                Constraint::Min(0),
            ])
            .split(area);

        let cpu_text = format!("CPU Usage: {:.2}%\nTemp: {:.1}°C", stats.cpu.usage_percent, stats.cpu.temp_celsius);
        let cpu_block = Block::default()
            .title(" CPU Info ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(Paragraph::new(cpu_text).block(cpu_block), chunks[0]);

        let ram_text = format!(
            "RAM: {:.2} GB / {:.2} GB ({:.1}%)",
            stats.memory.used_gb, stats.memory.total_gb, stats.memory.usage_percent
        );
        let ram_block = Block::default()
            .title(" Memory Info ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        f.render_widget(Paragraph::new(ram_text).block(ram_block), chunks[1]);

        if let Some(bat) = &stats.battery {
            let bat_text = format!(
                "Capacity: {}%\nStatus: {}\nVoltage: {} mV",
                bat.capacity_percent, bat.status, bat.voltage_mv
            );
            let bat_block = Block::default()
                .title(" Battery Info ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));
            f.render_widget(Paragraph::new(bat_text).block(bat_block), chunks[2]);
        } else {
            let bat_block = Block::default()
                .title(" Battery Info ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            f.render_widget(Paragraph::new("No Battery Detected").block(bat_block), chunks[2]);
        }

        let hours = stats.uptime_secs / 3600;
        let minutes = (stats.uptime_secs % 3600) / 60;
        let uptime_text = format!("System Uptime: {}h {}m", hours, minutes);
        let uptime_block = Block::default()
            .title(" System Status ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(Paragraph::new(uptime_text).block(uptime_block), chunks[3]);
    }

    fn draw_right_panel(&self, f: &mut Frame, area: Rect, _stats: &SystemStats) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(12),
                Constraint::Length(3),
            ])
            .split(area);

        let log_content = self.state.logs.join("\n");
        let log_block = Block::default()
            .title(" Output Log ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));
        f.render_widget(
            Paragraph::new(log_content)
                .block(log_block)
                .wrap(Wrap { trim: true }),
            chunks[0],
        );

        let image_block = Block::default()
            .title(" Image View (512x512 Area) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue));
        
        let image_status = if let Some(ref data) = self.state.image_data {
            format!("Loaded Image Data Size: {} bytes\nReady to render pixels...", data.len())
        } else {
            "No Image Loaded.\nUse: image load <path> (Raw 512x512 RGBA data)".to_string()
        };
        f.render_widget(Paragraph::new(image_status).block(image_block), chunks[1]);

        let input_block = Block::default()
            .title(" Command Input ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
        f.render_widget(
            Paragraph::new(self.state.command_input.as_str()).block(input_block),
            chunks[2],
        );
    }
}