mod app;
mod ui;

use std::io;

use crossterm::{
    event::{read, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

use app::{App};
use tui::{backend::CrosstermBackend, Terminal};

pub enum Message {
    KeyPressed
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                app.up();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down, ..
            }) => {
                app.down();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left, ..
            }) => {
                app.left();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right, ..
            }) => {
                app.right();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter, ..
            }) => {
                app.enter();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                println!("This is the key: {:?}", c)
            }
            _ => (),
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
