use crate::{
    app::{BoardCell, BoardCellState, BoardState, GameState, Level, Player},
    Event,
};
use rand::Rng;
use std::{
    fmt::{self, Display},
    sync::mpsc::{self, Receiver, Sender},
};
use std::{thread, time::Duration};

// Events sent
pub enum Action {
    Chat(String),
    PlaceToken(usize, usize),
    PlaceTokenError,
}

#[derive(Clone, Copy, Debug)]
pub enum Character {
    ChattyDave,
    SpeedySteve,
}

impl Character {
    pub fn full_name(&self) -> String {
        match self {
            Character::ChattyDave => String::from("Chatty Dave"),
            Character::SpeedySteve => String::from("Speedy Steve"),
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match &self {
            Character::ChattyDave => "Dave",
            Character::SpeedySteve => "Steve",
        };

        f.write_str(level)
    }
}

// Events received
pub enum Trigger {
    ComputersTurn(GameState),
    // TODO - think of a better name
    ComputersTurnFirst,
    Loser,
    Winner,
    Draw,
}

pub fn start(sender: Sender<Event>) -> Sender<Trigger> {
    let (computer_sender, computer_receiver): (Sender<Trigger>, Receiver<Trigger>) =
        mpsc::channel();

    std::thread::spawn(move || loop {
        match computer_receiver.recv().unwrap() {
            Trigger::ComputersTurn(game_state) => {
                match game_state.computer_character {
                    Character::ChattyDave => {
                        wait_in_seconds(2);
                        thinking(&sender);
                        wait_in_seconds(5);
                    }
                    Character::SpeedySteve => {
                        wait_in_seconds(1);
                        send_chat_event("🤔", &sender);
                        wait_in_seconds(2);
                    }
                }

                match find_empty_cell(game_state) {
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
                    "Want to play again? Press N to clear the game board and I can beat you again.",
                    &sender,
                );
            }

            Trigger::Loser => {
                send_chat_event("Nicely played 👏", &sender);
            }

            Trigger::Draw => {
                send_chat_event("Looks like it's a draw. Want to play again?", &sender);
            }

            Trigger::ComputersTurnFirst => {
                wait_in_seconds(3);
                let place_token_in_centre_cell = Action::PlaceToken(1, 1);

                sender
                    .send(Event::ComputerAction(place_token_in_centre_cell))
                    .unwrap();

                send_chat_event("Alright, you're up.", &sender);
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

fn find_empty_cell(game_state: GameState) -> Option<(usize, usize)> {
    match game_state.difficulty_level {
        Level::Easy => find_any_empty_cell(game_state.board_state),
        Level::Hard => find_best_empty_cell(game_state.board_state),
    }
}

fn find_best_empty_cell(board_state: BoardState) -> Option<(usize, usize)> {
    find_winning_cell(board_state)
        .or_else(|| find_defending_cell(board_state).or_else(|| find_any_empty_cell(board_state)))
}

fn find_any_empty_cell(board_state: BoardState) -> Option<(usize, usize)> {
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

fn find_winning_cell(board_state: BoardState) -> Option<(usize, usize)> {
    let two_in_combination = player_with_two_in_combination(board_state, Player::Computer);

    find_empty_cell_in_combination(board_state, two_in_combination)
}

fn find_defending_cell(board_state: BoardState) -> Option<(usize, usize)> {
    let two_in_combination = player_with_two_in_combination(board_state, Player::User);

    find_empty_cell_in_combination(board_state, two_in_combination)
}

fn player_with_two_in_combination(
    board_state: BoardState,
    player: Player,
) -> Option<&'static [(usize, usize); 3]> {
    BoardState::winning_combinations()
        .iter()
        .find(|combination| {
            let in_combination = combination.iter().fold(0, |mut count: i32, cell_position| {
                let (row, column) = cell_position;

                match board_state.cells[*row][*column] {
                    BoardCellState::Selected(BoardCell::Occupied(cell_player))
                    | BoardCellState::NotSelected(BoardCell::Occupied(cell_player)) => {
                        if cell_player == player {
                            count = count + 1;
                        } else {
                            count = count - 1;
                        }
                    }
                    _ => (),
                }
                count
            });

            in_combination == 2
        })
}

fn find_empty_cell_in_combination(
    board_state: BoardState,
    combination: Option<&[(usize, usize); 3]>,
) -> Option<(usize, usize)> {
    if combination.is_none() {
        return None;
    }

    let cell_option =
        combination
            .unwrap()
            .iter()
            .find(|(row, column)| match board_state.cells[*row][*column] {
                BoardCellState::Selected(BoardCell::Empty)
                | BoardCellState::NotSelected(BoardCell::Empty) => true,
                _ => false,
            });

    if let Some((row, column)) = cell_option {
        return Some((*row, *column));
    } else {
        None
    }
}
