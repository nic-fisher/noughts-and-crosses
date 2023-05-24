use std::{thread, time};

const BOARD_ROWS: usize = 3;
const BOARD_COLUMNS: usize = 3;
const MAX_BOARD_ROWS_INDEX: usize = BOARD_ROWS - 1;
const MAX_BOARD_COLUMNS_INDEX: usize = BOARD_COLUMNS - 1;

pub struct App {
    pub instructions: String,
    pub game_state: GameState,
    menu_state: MenuState,
    pub cursor_location: CursorLocation,
}

impl App {
    pub fn default() -> App {
        App {
            instructions: String::from("Start game"),
            game_state: GameState {
                current_turn: Player::Player,
                winner: None,
                board_state: BoardState {
                    cells: [
                        [
                            BoardCellState::Selected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                        ],
                        [
                            BoardCellState::NotSelected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                        ],
                        [
                            BoardCellState::NotSelected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                            BoardCellState::NotSelected(BoardCell::Empty),
                        ],
                    ],
                },
            },
            cursor_location: CursorLocation::default(),
            menu_state: MenuState {},
        }
    }

    pub fn up(&mut self) {
        let CursorLocation{row, column} = self.cursor_location;

        match row {
            MAX_BOARD_ROWS_INDEX => return,
            _ => self.move_cursor_location(row + 1, column)

        }
    }

    pub fn down(&mut self) {
        let CursorLocation{row, column} = self.cursor_location;

        match row {
            0 => return,
            _ => self.move_cursor_location(row - 1, column)

        }
    }

    pub fn left(&mut self) {
        let CursorLocation{row, column} = self.cursor_location;

        match column {
            0 => return,
            _ => self.move_cursor_location(row, column - 1)
        }
    }

    pub fn right(&mut self) {
        let CursorLocation{row, column} = self.cursor_location;

        match column {
            MAX_BOARD_COLUMNS_INDEX => return,
            _ => self.move_cursor_location(row, column + 1)
        }
    }

    pub fn enter(&mut self) {
        let CursorLocation{row, column} = self.cursor_location;

        match self.game_state.current_turn {
            Player::Computer => {
                self.instructions = String::from("Computers turns. Please wait.");
            }
            Player::Player => {
                self.game_state.try_place_token(self.cursor_location).and_then(|()| Some(self.game_state.swap_current_turn()));
                self.instructions = String::from("Are you sure that's a good choice.");

                // trigger computers turn
            }
        }
        // if let Player::Player = self.game_state.current_turn {

        // }
        // Check if it's the players turn
        // Check if the cell is free
        // Change cell to players symbol
        // Check if there's a winner
        // Change players turn to the computer
    }

    pub fn move_cursor_location(&mut self, row: usize, column: usize) {
        // let (current_row, current_column) = self.cursor_location;
        let CursorLocation{row: current_row, column: current_column} = self.cursor_location;


        if let BoardCellState::Selected(selected_cell) = &self.game_state.board_state.cells[current_row][current_column] {
            self.game_state.board_state.cells[current_row][current_column] = BoardCellState::NotSelected(*selected_cell);
        }
        if let BoardCellState::NotSelected(cell) = &self.game_state.board_state.cells[row][column] {
            self.game_state.board_state.cells[row][column] = BoardCellState::Selected(*cell);
        }

        self.cursor_location = CursorLocation{row: row, column: column};
    }
}


#[derive(Clone, Copy, Debug)]
pub struct CursorLocation {
    row: usize,
    column: usize,
}

impl Default for CursorLocation {
    fn default() -> Self {
        CursorLocation { row: 0, column: 0 }
    }
}

pub struct GameState {
    pub board_state: BoardState,
    pub current_turn: Player,
    winner: Option<Player>,
}

impl GameState {
    // Move this to board state
    pub fn try_place_token(&mut self, cursor_location: CursorLocation) -> Option<()> {
        let CursorLocation{row, column } = cursor_location;

        match self.board_state.cells[row][column] {
            BoardCellState::Selected(BoardCell::Empty) => {
                self.board_state.cells[row][column] = BoardCellState::Selected(BoardCell::Occupied(Player::Player));
                Some(())
            },
            BoardCellState::Selected(BoardCell::Occupied(_)) => {
                // let new_instructions = String::from("Looks like a token has already been placed on that cell. Please select another cell.")
                // instructions = new_instructions.as_mut_str();
                None
            }
            BoardCellState::NotSelected(_) => {
                None
                // This case should never get hit
            },
        }

    }

    pub fn swap_current_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::Computer => Player::Player,
            Player::Player => Player::Computer,
        };
    }
}

pub struct BoardState {
    // pub cells: [[BoardCell::Empty, BoardCell::Empty, BoardCell::Empty], [BoardCell::Empty, BoardCell::Empty, BoardCell::Empty], [BoardCell::Empty, BoardCell::Empty, BoardCell::Empty]]
    pub cells: [[BoardCellState; BOARD_ROWS]; BOARD_COLUMNS],
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

// board cell can be
// - empty
// - occupied with player 1
// - occupied with player 2
// - selected and empty
// - selected occupied with player 1
// - selected occupied with player 2

impl Default for BoardCellState {
    fn default() -> Self {
        BoardCellState::NotSelected(BoardCell::Empty)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Player,
    Computer,
}

struct MenuState {}
