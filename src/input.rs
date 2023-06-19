use crate::Event;
use crossterm::event::{read, Event as CrosstermEvent, KeyCode, KeyEvent};
use std::sync::mpsc::Sender;

pub enum InputKey {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Unhandled,
    Char(char),
}

pub fn start(sender: Sender<Event>) {
    std::thread::spawn(move || loop {
        match read().unwrap() {
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                sender.send(Event::UserInput(InputKey::Esc)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                sender.send(Event::UserInput(InputKey::Up)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Down)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Left)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Right)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Enter)).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('n'),
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Char('n'))).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('e'),
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Char('e'))).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char('h'),
                ..
            }) => {
                sender.send(Event::UserInput(InputKey::Char('h'))).unwrap();
            }
            CrosstermEvent::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                println!("This is the key: {:?}", c)
            }
            _ => (),
        }
    });
}
