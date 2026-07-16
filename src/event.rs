use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Input(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            let timeout = tick_rate;

            if event::poll(timeout).expect("failed to poll events") {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx.send(AppEvent::Input(key)).is_err() {
                        break;
                    }
                } else if let Ok(Event::Resize(_, _)) = event::read() {
                }
            }

            if tx.send(AppEvent::Tick).is_err() {
                break;
            }
        });

        EventHandler { rx }
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub fn handle_input(key: KeyEvent) -> Option<InputCommand> {
    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match c {
                    'c' | 'q' => Some(InputCommand::Quit),
                    'l' => Some(InputCommand::ClearLog),
                    _ => None,
                }
            } else {
                Some(InputCommand::AddChar(c))
            }
        }
        KeyCode::Backspace => Some(InputCommand::Backspace),
        KeyCode::Delete => Some(InputCommand::Delete),
        KeyCode::Enter => Some(InputCommand::Execute),
        KeyCode::Esc => Some(InputCommand::ClearInput),
        KeyCode::Tab => Some(InputCommand::Autocomplete),
        KeyCode::Up => Some(InputCommand::HistoryUp),
        KeyCode::Down => Some(InputCommand::HistoryDown),
        KeyCode::Left => Some(InputCommand::MoveCursorLeft),
        KeyCode::Right => Some(InputCommand::MoveCursorRight),
        KeyCode::Home => Some(InputCommand::MoveCursorHome),
        KeyCode::End => Some(InputCommand::MoveCursorEnd),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub enum InputCommand {
    AddChar(char),
    Backspace,
    Delete,
    ClearInput,
    Execute,
    Autocomplete,
    HistoryUp,
    HistoryDown,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorHome,
    MoveCursorEnd,
    ClearLog,
    Quit,
}