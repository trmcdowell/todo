use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let main_block = Block::default()
        .title(" TODO List ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let todo_list = Paragraph::new(Text::styled(
        "test".to_string(),
        Style::default().fg(Color::White),
    ))
    .block(main_block);

    frame.render_widget(todo_list, frame.size());
}
