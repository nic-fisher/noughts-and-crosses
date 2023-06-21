mod app;
mod computer;
mod input;
mod ui;

use std::io;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

use app::{App, Level};
use computer::{Action, Trigger};
use input::InputKey;
use std::sync::mpsc::{self, Receiver, Sender};
use tui::{backend::CrosstermBackend, Terminal};

pub enum Event {
    UserInput(InputKey),
    ComputerAction(Action),
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::default();

    let (sender, receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
    input::start(sender.clone());
    let computer_sender = computer::start(sender.clone());

    loop {
        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        match receiver.recv().unwrap() {
            Event::UserInput(InputKey::Esc) => break,
            Event::UserInput(input_key) => {
                handle_user_input(&mut app, input_key, &computer_sender);
            }
            Event::ComputerAction(Action::Chat(words)) => {
                app.chat = [format!("{}:", app.game_state.computer_character), words].join(" ");
            }
            Event::ComputerAction(Action::PlaceToken(row, column)) => {
                app.computer_place_token((row, column), &computer_sender);
            }
            Event::ComputerAction(Action::PlaceTokenError) => {
                app.instructions = format!(
                    "Oh no, looks like {} has hit an error when trying to place a token",
                    app.game_state.computer_character
                )
            }
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

fn handle_user_input(app: &mut App, input_key: InputKey, computer_sender: &Sender<Trigger>) {
    match input_key {
        InputKey::Up => {
            app.up();
        }
        InputKey::Down => {
            app.down();
        }
        InputKey::Left => {
            app.left();
        }
        InputKey::Right => {
            app.right();
        }
        InputKey::Enter => {
            app.enter(computer_sender);
        }
        InputKey::Char('s') => {
            app.start_game(computer_sender);
        }
        InputKey::Char('n') => {
            app.new_game();
        }
        InputKey::Char('h') => {
            app.update_level(Level::Hard);
        }
        InputKey::Char('e') => {
            app.update_level(Level::Easy);
        }
        InputKey::Char('c') => {
            app.swap_computer_character();
        }
        InputKey::Unhandled => (),
        _ => (),
    }
}
