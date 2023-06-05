use crate::{
    app::{BoardCell, BoardCellState, BoardState},
    Event,
};
use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::{thread, time::Duration};

// Events sent
pub enum Action {
    Chat(String),
    PlaceToken(usize, usize),
    PlaceTokenError,
}

// Events received
pub enum Trigger {
    ComputersTurn(BoardState),
    Loser,
    Winner,
}

pub fn start(sender: Sender<Event>) -> Sender<Trigger> {
    let (computer_sender, computer_receiver): (Sender<Trigger>, Receiver<Trigger>) =
        mpsc::channel();

    std::thread::spawn(move || loop {
        match computer_receiver.recv().unwrap() {
            Trigger::ComputersTurn(board_state) => {
                thread::sleep(Duration::from_secs(2));
                thinking(&sender);
                thread::sleep(Duration::from_secs(5));

                match find_empty_cell(board_state) {
                    Some((row, column)) => sender
                        .send(Event::ComputerAction(Action::PlaceToken(row, column)))
                        .unwrap(),
                    None => sender
                        .send(Event::ComputerAction(Action::PlaceTokenError))
                        .unwrap(),
                }
            }

            Trigger::Winner => {
                wait_in_seconds(2);
                send_chat_event("Winner, winner, chicken dinner 🏆", &sender);
                wait_in_seconds(5);
                send_chat_event(
                    "Want to play again? Press N to start a new game and I can beat you again.",
                    &sender,
                );
            }

            Trigger::Loser => {
                send_chat_event("Nicely played 👏", &sender);
            }
        }
    });

    computer_sender
}

fn thinking(sender: &Sender<Event>) {
    match rand::thread_rng().gen_range(0..=10) {
        0..=3 => {
            send_chat_event("🤔", sender);
        }
        4 | 5 => {
            send_chat_event("Hmm this is tough.", sender);
        }
        6 | 7 => {
            send_chat_event("BRB, just going to grab a coffee.", sender);
            wait_in_seconds(6);
            send_chat_event("Ok, back!", sender);
        }
        8 | 9 => {
            send_chat_event("We should really go for a beer soon 🍺", sender);
            wait_in_seconds(4);
            send_chat_event("Oh, it's my turn! Let me think 🤔", sender);
        }
        10 => send_chat_event("Really? You're going there 😂", sender),
        _ => {}
    }
}

fn wait_in_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

fn send_chat_event(chat_message: &str, sender: &Sender<Event>) {
    sender
        .send(Event::ComputerAction(Action::Chat(String::from(
            chat_message,
        ))))
        .unwrap();
}

fn find_empty_cell(board_state: BoardState) -> Option<(usize, usize)> {
    let available_cells = board_state.cells.iter().enumerate().fold(
        vec![],
        |mut acc: Vec<(usize, usize)>, (row, cells)| {
            cells.iter().enumerate().for_each(|(column, cell)| {
                match *cell {
                    BoardCellState::NotSelected(BoardCell::Empty) => acc.push((row, column)),
                    BoardCellState::Selected(BoardCell::Empty) => acc.push((row, column)),
                    _ => (),
                };
            });
            acc
        },
    );

    if available_cells.len() == 0 {
        return None;
    }

    let random_available_cell_index = rand::thread_rng().gen_range(0..available_cells.len());

    Some(available_cells[random_available_cell_index])
}
