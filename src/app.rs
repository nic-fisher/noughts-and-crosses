use std::fmt::{self, Display};
use std::sync::mpsc::Sender;

use crate::computer::Trigger;

const BOARD_ROWS: usize = 3;
const BOARD_COLUMNS: usize = 3;
const MAX_BOARD_ROWS_INDEX: usize = BOARD_ROWS - 1;
const MAX_BOARD_COLUMNS_INDEX: usize = BOARD_COLUMNS - 1;

pub struct App<'a> {
    pub instructions: &'a str,
    pub chat: String,
    pub game_state: GameState,
    pub cursor_location: CursorLocation,
}

impl App<'_> {
    pub fn default() -> App<'static> {
        App {
            instructions: "Press enter to place your token.",
            chat: String::from(""),
            game_state: GameState {
                current_turn: Player::User,
                winner: None,
                board_state: BoardState::default(),
                difficulty_level: Level::Hard,
            },
            cursor_location: CursorLocation::default(),
        }
    }

    // I couldnt make up my mind on how I wanted to order the cells.
    // This is what I've gone with [row][cell] starting with [0][0] being the bottom left cell.
    //    *       *       *
    // [2][0]  [2][1]  [2][2]
    //    *       *       *
    // [1][0]  [1][1]  [1][2]
    //    *       *       *
    // [0][0]  [0][1]  [0][2]

    pub fn up(&mut self) {
        let CursorLocation { row, column } = self.cursor_location;

        match row {
            MAX_BOARD_ROWS_INDEX => (),
            _ => self.move_cursor_location(row + 1, column),
        }
    }

    pub fn down(&mut self) {
        let CursorLocation { row, column } = self.cursor_location;

        match row {
            0 => (),
            _ => self.move_cursor_location(row - 1, column),
        }
    }

    pub fn left(&mut self) {
        let CursorLocation { row, column } = self.cursor_location;

        match column {
            0 => (),
            _ => self.move_cursor_location(row, column - 1),
        }
    }

    pub fn right(&mut self) {
        let CursorLocation { row, column } = self.cursor_location;

        match column {
            MAX_BOARD_COLUMNS_INDEX => (),
            _ => self.move_cursor_location(row, column + 1),
        }
    }

    pub fn enter(&mut self, computer_sender: &Sender<Trigger>) {
        let CursorLocation { row, column } = self.cursor_location;

        if self.game_state.winner != None {
            return;
        }

        match self.game_state.current_turn {
            Player::Computer => {
                self.instructions = "Computers turns. Please wait.";
            }
            Player::User => {
                self.game_state
                    .board_state
                    .try_place_token((row, column), Player::User)
                    .handle_user_place_token_result(self, &computer_sender);
            }
        }
    }

    pub fn new_game(&mut self) {
        if self.game_finished() || self.game_state.winner != None {
            self.restart_game();
        } else {
            self.instructions = "Unable to start a new game until the current game is finished.";
        }
    }

    pub fn update_level(&mut self, level: Level) {
        if self.game_finished() || self.game_state.winner != None || !self.game_started() {
            self.game_state.difficulty_level = level;
        } else {
            self.instructions = "Unable to update the difficulty while the game has started.";
        }
    }

    fn game_started(&mut self) -> bool {
        self.game_state.board_state.cells.into_iter().any(|row| {
            row.into_iter().any(|cell| match cell {
                BoardCellState::NotSelected(BoardCell::Occupied(_player))
                | BoardCellState::Selected(BoardCell::Occupied(_player)) => true,
                _ => false,
            })
        })
    }

    fn game_finished(&mut self) -> bool {
        self.game_state.board_state.cells.into_iter().all(|row| {
            row.into_iter().all(|cell| match cell {
                BoardCellState::NotSelected(BoardCell::Occupied(_player))
                | BoardCellState::Selected(BoardCell::Occupied(_player)) => true,
                _ => false,
            })
        })
    }

    pub fn computer_place_token(
        &mut self,
        (row, column): (usize, usize),
        computer_sender: &Sender<Trigger>,
    ) {
        self.game_state
            .board_state
            .try_place_token((row, column), Player::Computer)
            .handle_computer_place_token_result(self, &computer_sender);
    }

    pub fn move_cursor_location(&mut self, row: usize, column: usize) {
        let CursorLocation {
            row: current_row,
            column: current_column,
        } = self.cursor_location;

        if let BoardCellState::Selected(selected_cell) =
            &self.game_state.board_state.cells[current_row][current_column]
        {
            self.game_state.board_state.cells[current_row][current_column] =
                BoardCellState::NotSelected(*selected_cell);
        }
        if let BoardCellState::NotSelected(cell) = &self.game_state.board_state.cells[row][column] {
            self.game_state.board_state.cells[row][column] = BoardCellState::Selected(*cell);
        }

        self.cursor_location = CursorLocation {
            row: row,
            column: column,
        };
    }

    pub fn restart_game(&mut self) {
        self.instructions = "New game";
        self.game_state.board_state = BoardState::default();
        self.cursor_location = CursorLocation::default();
        self.game_state.current_turn = Player::User;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CursorLocation {
    pub row: usize,
    pub column: usize,
}

impl Default for CursorLocation {
    fn default() -> Self {
        CursorLocation { row: 1, column: 1 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub board_state: BoardState,
    pub current_turn: Player,
    pub difficulty_level: Level,
    winner: Option<Player>,
}

impl GameState {
    pub fn swap_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Computer => Player::User,
            Player::User => Player::Computer,
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BoardState {
    pub cells: [[BoardCellState; BOARD_ROWS]; BOARD_COLUMNS],
}

impl BoardState {
    pub fn default() -> BoardState {
        BoardState {
            cells: [
                [
                    BoardCellState::NotSelected(BoardCell::Empty),
                    BoardCellState::NotSelected(BoardCell::Empty),
                    BoardCellState::NotSelected(BoardCell::Empty),
                ],
                [
                    BoardCellState::NotSelected(BoardCell::Empty),
                    BoardCellState::Selected(BoardCell::Empty),
                    BoardCellState::NotSelected(BoardCell::Empty),
                ],
                [
                    BoardCellState::NotSelected(BoardCell::Empty),
                    BoardCellState::NotSelected(BoardCell::Empty),
                    BoardCellState::NotSelected(BoardCell::Empty),
                ],
            ],
        }
    }

    pub fn try_place_token(
        &mut self,
        (row, column): (usize, usize),
        player: Player,
    ) -> PlaceTokenResult {
        match (self.cells[row][column], player) {
            (BoardCellState::Selected(BoardCell::Empty), _) => {
                self.cells[row][column] = BoardCellState::Selected(BoardCell::Occupied(player));
                match self.check_for_winner() {
                    Some(player) => PlaceTokenResult::SuccessWithWinner(player),
                    None => PlaceTokenResult::Success,
                }
            }
            (BoardCellState::Selected(BoardCell::Occupied(Player::User)), _) => {
                PlaceTokenResult::OccupiedByUser
            }
            (BoardCellState::Selected(BoardCell::Occupied(Player::Computer)), _) => {
                PlaceTokenResult::OccupiedByComputer
            }
            (BoardCellState::NotSelected(_), Player::Computer) => {
                self.cells[row][column] = BoardCellState::NotSelected(BoardCell::Occupied(player));

                match self.check_for_winner() {
                    Some(player) => PlaceTokenResult::SuccessWithWinner(player),
                    None => PlaceTokenResult::Success,
                }
            }
            (BoardCellState::NotSelected(_), Player::User) => PlaceTokenResult::Error,
        }
    }

    // You know the code logic is awful when you have to add comments to explain what you're doing.
    // This function will loop through the board cells and compare them to a list of winning combinations.
    // It will return a winning player if all the cells in a winning combinations are occupied by the same player.
    // The winning_combinations consists of three tuples containing a row & column index within an array nested inside
    // another array. Simples
    fn check_for_winner(self) -> Option<Player> {
        let (_winner, player) = BoardState::winning_combinations().iter().fold(
            (false, None),
            |(winning_combination, player): (bool, Option<Player>), combination| {
                // If there is a winning combination we no longer need to continue checking.
                if winning_combination {
                    return (true, player);
                }

                let (winner, player): (Option<bool>, Option<Player>) = combination.iter().fold(
                    (None, None),
                    |(valid_combination, player): (Option<bool>, Option<Player>), (row, column)| {
                        if Some(false) == valid_combination {
                            return (Some(false), None);
                        }

                        match self.cells[*row][*column] {
                            BoardCellState::NotSelected(BoardCell::Occupied(Player::User))
                            | BoardCellState::Selected(BoardCell::Occupied(Player::User)) => {
                                // Checks to see if the player is None which will mean it's the first loop
                                // for this combination. Or if the player is the User which means the previous
                                // cell in the combination was occupied by the user.
                                if player == None || player.unwrap() == Player::User {
                                    return (Some(true), Some(Player::User));
                                } else {
                                    return (Some(false), None);
                                }
                            }
                            BoardCellState::NotSelected(BoardCell::Occupied(Player::Computer))
                            | BoardCellState::Selected(BoardCell::Occupied(Player::Computer)) => {
                                if player == None || player.unwrap() == Player::Computer {
                                    return (Some(true), Some(Player::Computer));
                                } else {
                                    return (Some(false), None);
                                }
                            }
                            _ => return (Some(false), None),
                        }
                    },
                );

                (winner.unwrap(), player)
            },
        );

        player
    }

    pub fn winning_combinations() -> &'static [[(usize, usize); 3]; 8] {
        const WINNING_COMBINATIONS: [[(usize, usize); 3]; 8] = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        &WINNING_COMBINATIONS
    }
}

pub enum PlaceTokenResult {
    SuccessWithWinner(Player),
    Success,
    OccupiedByUser,
    OccupiedByComputer,
    Error,
}

impl PlaceTokenResult {
    pub fn handle_computer_place_token_result(
        self,
        app: &mut App,
        computer_sender: &Sender<Trigger>,
    ) {
        match self {
            PlaceTokenResult::Success => {
                if app.game_finished() && app.game_state.winner.is_none() {
                    computer_sender.send(Trigger::Draw).unwrap();

                    app.instructions = "It's a tie.";
                } else {
                    app.chat = String::from("Computer: Ok, your turn!");
                    app.instructions = "Press enter to your place token.";
                    app.game_state.swap_current_turn()
                }
            }
            PlaceTokenResult::SuccessWithWinner(_player) => {
                app.game_state.winner = Some(Player::Computer);
                app.instructions = "The computer wins!";
                computer_sender.send(Trigger::Winner).unwrap();
            }
            _ => {
                app.instructions = "Oops, something went wrong.";
            }
        }
    }

    pub fn handle_user_place_token_result(self, app: &mut App, computer_sender: &Sender<Trigger>) {
        match self {
            PlaceTokenResult::Success => {
                app.game_state.swap_current_turn();

                if app.game_finished() && app.game_state.winner.is_none() {
                    computer_sender.send(Trigger::Draw).unwrap();

                    app.instructions = "It's a tie.";
                } else {
                    computer_sender
                        .send(Trigger::ComputersTurn(app.game_state))
                        .unwrap();

                    app.instructions = "Computers turn.";
                }
            }
            PlaceTokenResult::SuccessWithWinner(_player) => {
                app.game_state.winner = Some(Player::User);
                app.instructions = "You win! Press N to start a new game.";
                computer_sender.send(Trigger::Loser).unwrap();
            }
            PlaceTokenResult::OccupiedByComputer => {
                app.instructions = "This cell is already occupied by the computer.";
            }
            PlaceTokenResult::OccupiedByUser => {
                app.instructions = "This cell is already occupied by you.";
            }
            PlaceTokenResult::Error => {
                app.instructions = "Oops, something went wrong.";
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BoardCellState {
    Selected(BoardCell),
    NotSelected(BoardCell),
}
#[derive(Clone, Copy, Debug)]
pub enum BoardCell {
    Empty,
    Occupied(Player),
}

impl Display for BoardCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board_cell = match &self {
            BoardCell::Empty => "  *  ",
            BoardCell::Occupied(Player::Computer) => "  o  ",
            BoardCell::Occupied(Player::User) => "  x  ",
        };

        f.write_str(board_cell)
    }
}

impl Default for BoardCellState {
    fn default() -> Self {
        BoardCellState::NotSelected(BoardCell::Empty)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    User,
    Computer,
}

#[derive(Clone, Copy, Debug)]
pub enum Level {
    Easy,
    Hard,
}

impl Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match &self {
            Level::Easy => "Easy",
            Level::Hard => "Hard",
        };

        f.write_str(level)
    }
}
