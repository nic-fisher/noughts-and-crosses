use crossterm::{
    event::{DisableMouseCapture, Event, KeyEvent, KeyCode, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};

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

pub fn draw(f: &mut tui::Frame<CrosstermBackend<io::Stdout>>) {
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

    let header_block = Block::default().title("Noughts and Crosses").borders(Borders::ALL);
    f.render_widget(header_block, chunks[0]);

    let footer_block = Block::default().title("Instructions").borders(Borders::ALL);
    let text = Span::raw("My text");
    let paragraph = Paragraph::new(text).block(footer_block).alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[2]);

    let game_block = Block::default().title("Game").borders(Borders::ALL);
    f.render_widget(game_block, chunks[1]);

    let game_table = Table::new(vec![
        Row::new(vec![Cell::from("*"), Cell::from("*").style(Style::default().fg(Color::Yellow)), Cell::from("*")]).style(Style::default().fg(Color::Blue)),
        Row::new(vec![Cell::from("*"), Cell::from("*"), Cell::from("*")]).style(Style::default().fg(Color::Blue)),
        Row::new(vec![Cell::from("*"), Cell::from("*"), Cell::from("*")]).style(Style::default().fg(Color::Blue)),
    ])
    .style(Style::default().fg(Color::White))
    .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(5)])
    .column_spacing(0);

    f.render_widget(game_table, generate_game_area(chunks[1]));
}

fn generate_game_area(game_block: Rect) -> Rect {
    let game_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [Constraint::Length(5), Constraint::Length(100)].as_ref(),
    )
    .split(game_block);

    let v = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
        [Constraint::Length(2), Constraint::Length(100)].as_ref(),
    )
    .split(game_layout[1]);

    v[1]
}