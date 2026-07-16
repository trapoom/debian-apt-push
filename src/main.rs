mod app;
mod config;
mod event;
mod sysinfo;
mod ui;

use anyhow::Result;
use app::App;
use config::AppConfig;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use event::{AppEvent, EventHandler, handle_input};
use ratatui::{backend::CrosstermBackend, Terminal};
use sysinfo::detect_battery_path;
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    let mut config = AppConfig::load()?;
    
    if config.battery_path.is_none() {
        config.battery_path = detect_battery_path();
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut app = App::new(config)?;

    let event_handler = EventHandler::new(Duration::from_millis(
        app.config.update_interval_ms.min(100u64),
    ));

    let result = run_app(&mut terminal, &mut app, &event_handler);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.clear()?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<()> {
    loop {
        let stats = app.sys_monitor.get_system_stats()?;

        app.update(&stats)?;

        terminal.draw(|f| {
            app.ui.draw(f, &stats);
        })?;

        match event_handler.next() {
            Ok(AppEvent::Tick) => {}
            Ok(AppEvent::Input(key)) => {
                if let Some(cmd) = handle_input(key) {
                    app.handle_input(cmd)?;
                    if !app.running {
                        break;
                    }
                }
            }
            Err(_) => break,
        }

        if !app.running {
            break;
        }
    }

    let _ = app.config.save();

    Ok(())
}