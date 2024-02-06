use crate::app::{App, CurrentScreen};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Clear, List, ListItem, Paragraph},
};

// Main theme color
static THEME_COLOR: Color = Color::Cyan;

pub fn ui(frame: &mut Frame, app: &mut App) {
    render_main_widget(frame, app);
    if let CurrentScreen::Exiting = app.current_screen {
        render_exit_widget(frame);
    }
}

fn render_main_widget(frame: &mut Frame, app: &App) {
    let main_block = Block::bordered()
        .title(" TODO List ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    // Create main todo list widget
    let list_items = build_list_items(app);
    let todo_list = List::new(list_items).block(main_block);

    frame.render_widget(todo_list, frame.size());
}

fn render_exit_widget(frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());
    let popup_block = Block::bordered()
        .title(" Y/N ")
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(THEME_COLOR));

    let exit_text = Line::styled(
        "Would you like to exit todo list?",
        Style::default().fg(THEME_COLOR),
    );
    let exit_paragraph = Paragraph::new(exit_text).block(popup_block);
    let area = centered_rect(60, 25, frame.size());
    frame.render_widget(exit_paragraph, area);
}

/// Build styled todo list items for ui
fn build_list_items(app: &App) -> Vec<ListItem> {
    app.todo_list
        .iter()
        .enumerate()
        .map(|(idx, todo_str)| {
            // If selecting or editing, highlight selected item
            if (app.current_screen == CurrentScreen::Selecting
                || app.current_screen == CurrentScreen::Editing)
                && app.selected == idx
            {
                return ListItem::new(Line::styled(
                    todo_str,
                    Style::default().fg(Color::Black).bg(THEME_COLOR),
                ));
            } else {
                // Default item appearance
                return ListItem::new(Line::styled(todo_str, Style::default().fg(THEME_COLOR)));
            }
        })
        .collect()
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
