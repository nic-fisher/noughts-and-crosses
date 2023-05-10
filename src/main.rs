use crossterm::{
    event::{DisableMouseCapture, Event, KeyEvent, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
mod ui;
use std::io;
use tui::{
    symbols::DOT,
    symbols::line::THICK,
    layout::{Layout, Constraint, Direction, Alignment, Rect},
    backend::CrosstermBackend,
    text::{Span, Spans},
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, Paragraph, Table, Cell, Row, Tabs, TableState},
    Terminal,
};

struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> App {
        App { should_quit: false }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
        // self
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            ui::draw(f);
        })?;

        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) =>  { quit_app(&mut app) }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => {println!("This is the up key")}
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {println!("This is the key: {:?}", c)}
            _ => ()
        }


        if app.should_quit {
            println!("Quitting app");
            break;
        }

        // ux::draw(&mut terminal, &mut app)?;
        // match events.next()? {
        //     Event::Input(key) => match key {
        //         Key::Char(c) => app.on_key(c),
        //         Key::Up => app.on_up(),
        //         Key::Down => app.on_down(),
        //         Key::Left => app.on_left(),
        //         Key::Right => app.on_right(),
        //         Key::Esc => app.on_ctrl_key('q'),
        //         Key::Backspace => app.on_backspace(),
        //         Key::Delete => app.on_delete(),
        //         Key::Ctrl(c) => app.on_ctrl_key(c),
        //         _ => {}
        //     },
        //     Event::Tick => {
        //         app.on_tick();
        //     }
        // }
        // if app.should_quit {
        //     terminal.clear()?;
        //     break;
        // }
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

fn quit_app(app: &mut App) {
    app.quit();
}