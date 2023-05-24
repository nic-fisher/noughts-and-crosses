use crossterm::{
    event::{read, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::line::THICK,
    symbols::DOT,
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs},
    Terminal,
};

use crate::app::{App, BoardCell, Player, BoardCellState};

pub fn draw(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let header_block = Block::default()
        .title("Noughts and Crosses")
        .borders(Borders::ALL);
    f.render_widget(header_block, chunks[0]);

    let footer_block = Block::default().title("Instructions").borders(Borders::ALL);
    let footer_text = Span::raw(app.instructions.clone());
    let footer_paragraph = Paragraph::new(footer_text)
        .block(footer_block)
        .alignment(Alignment::Left);
    f.render_widget(footer_paragraph, chunks[2]);

    let game_block = Block::default().title("Game").borders(Borders::ALL);
    f.render_widget(game_block, chunks[1]);

    let game_table = Table::new(app.game_state.board_state.cells.iter().rev().map(|c| {
        Row::new(c.iter().map(|cell_with_state| match cell_with_state {
            BoardCellState::Selected(cell) => build_board_cell(&cell, Color::Green),
            BoardCellState::NotSelected(cell) => build_board_cell(&cell, Color::Reset)
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
        BoardCell::Empty => Cell::from("  *  ").style(Style::default().bg(background_color)),
        BoardCell::Occupied(Player::Player) => Cell::from("  x  ").style(Style::default().fg(Color::Yellow).bg(background_color)),
        BoardCell::Occupied(Player::Computer) => Cell::from("  o  ").style(Style::default().fg(Color::Yellow).bg(background_color)),
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
