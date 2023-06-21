use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::app::{App, BoardCell, BoardCellState, Player};

pub fn draw(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(55),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    f.render_widget(build_menu_widget(app), chunks[0]);
    f.render_widget(build_game_border_widget(), chunks[1]);
    f.render_widget(build_instructions_widget(app), chunks[2]);
    f.render_widget(build_chat_widget(app), chunks[3]);
    f.render_widget(build_game_table_widget(app), generate_game_area(chunks[1]));
}

fn build_board_cell(cell: &BoardCell, background_color: Color) -> Cell {
    match cell {
        BoardCell::Empty => {
            Cell::from(format!("{}", cell)).style(Style::default().bg(background_color))
        }
        BoardCell::Occupied(Player::User) => Cell::from(format!("{}", cell))
            .style(Style::default().fg(Color::Yellow).bg(background_color)),
        BoardCell::Occupied(Player::Computer) => Cell::from(format!("{}", cell))
            .style(Style::default().fg(Color::Yellow).bg(background_color)),
    }
}

fn build_menu_widget(app: &App) -> List<'_> {
    let items = [
        ListItem::new(" "),
        ListItem::new("Use the below keys to update the game"),
        ListItem::new(" "),
        ListItem::new("Start game => S"),
        ListItem::new("New game => N"),
        ListItem::new("Change opponent => C"),
        ListItem::new("Easy => E"),
        ListItem::new("Hard => H"),
        ListItem::new(" "),
        ListItem::new(format!(
            "Selected level: {}",
            app.game_state.difficulty_level
        )),
        ListItem::new(format!(
            "Selected opponent: {}",
            app.game_state.computer_character.full_name()
        )),
    ];
    List::new(items)
        .block(Block::default().title("Game Options").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
}

fn build_instructions_widget(app: &App) -> Paragraph<'_> {
    let instructions_block = Block::default().title("Instructions").borders(Borders::ALL);
    let instructions_text = Span::raw(app.instructions.clone());
    Paragraph::new(instructions_text)
        .block(instructions_block)
        .alignment(Alignment::Left)
}

fn build_chat_widget(app: &App) -> Paragraph<'_> {
    let chat_block = Block::default().title("Chat").borders(Borders::ALL);
    let chat_text = Span::raw(app.chat.clone());
    Paragraph::new(chat_text)
        .block(chat_block)
        .alignment(Alignment::Left)
}

fn build_game_border_widget() -> Block<'static> {
    Block::default().title("Game").borders(Borders::ALL)
}

fn build_game_table_widget(app: &App) -> Table<'_> {
    Table::new(app.game_state.board_state.cells.iter().rev().map(|c| {
        Row::new(c.iter().map(|cell_with_state| match cell_with_state {
            BoardCellState::Selected(cell) => build_board_cell(&cell, Color::Green),
            BoardCellState::NotSelected(cell) => build_board_cell(&cell, Color::Reset),
        }))
    }))
    .style(Style::default().fg(Color::White))
    .widths(&[
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .column_spacing(0)
}

fn generate_game_area(game_block: Rect) -> Rect {
    let game_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Length(100)].as_ref())
        .split(game_block);

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(100)].as_ref())
        .split(game_layout[1]);

    v[1]
}
