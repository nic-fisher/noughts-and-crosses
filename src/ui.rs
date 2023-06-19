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

    let items = [
        ListItem::new(" "),
        ListItem::new("Use the below keys to update the game"),
        ListItem::new(" "),
        ListItem::new("New game => N"),
        ListItem::new("Easy => E"),
        ListItem::new("Hard => H"),
        ListItem::new(" "),
        ListItem::new(format!(
            "Selected level: {}",
            app.game_state.difficulty_level
        )),
    ];
    let v = List::new(items)
        .block(Block::default().title("Game Options").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(v, chunks[0]);

    let instructions_block = Block::default().title("Instructions").borders(Borders::ALL);
    let instructions_text = Span::raw(app.instructions.clone());
    let instructions_paragraph = Paragraph::new(instructions_text)
        .block(instructions_block)
        .alignment(Alignment::Left);
    f.render_widget(instructions_paragraph, chunks[2]);

    let chat_block = Block::default().title("Chat").borders(Borders::ALL);
    let chat_text = Span::raw(app.chat.clone());
    let chat_paragraph = Paragraph::new(chat_text)
        .block(chat_block)
        .alignment(Alignment::Left);
    f.render_widget(chat_paragraph, chunks[3]);

    let game_block = Block::default().title("Game").borders(Borders::ALL);
    f.render_widget(game_block, chunks[1]);

    let game_table = Table::new(app.game_state.board_state.cells.iter().rev().map(|c| {
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
    .column_spacing(0);

    f.render_widget(game_table, generate_game_area(chunks[1]));
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
