use crate::app::{App, CurrentScreen};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Clear, List, ListItem, Paragraph},
};

// Main theme color
const THEME_COLOR: Color = Color::LightGreen;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(frame.size());
    render_main_widget(app, frame, layout[0]);
    render_mode_widget(app, frame, layout[1]);
    if let CurrentScreen::Exiting = app.current_screen {
        render_exit_widget(frame);
    }
}

fn render_main_widget(app: &App, frame: &mut Frame, area: Rect) {
    let main_block = Block::bordered()
        .title(" TODO ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    // Create main todo list widget
    let list_items = build_list_items(app);
    let todo_list = List::new(list_items).block(main_block);

    frame.render_widget(todo_list, area);
}

fn render_mode_widget(app: &App, frame: &mut Frame, area: Rect) {
    let mode_block = Block::bordered()
        .title(format!(
            " Current Mode: {} ",
            app.current_screen.to_string()
        ))
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    let mode_widget = Paragraph::new({
        match app.current_screen {
            CurrentScreen::Main => {
                vec![
                    Line::raw("Welcome to todo!"),
                    Line::raw("Press 'q' or 'Esc' to save and quit."),
                ]
            }
            CurrentScreen::Selecting => {
                vec![
                    Line::raw(
                        "Press 'j' or 'k' to navigate items. Press 'e' or 'Enter' to edit an item. 
                         Add a new item with 'a', and delete items with 'd'. Mark an item as completed with 'c'.",
                    ),
                    Line::raw("Press 'q' or 'Esc' to stop selecting."),
                ]
            }
            CurrentScreen::Editing => vec![Line::raw("Press 'Enter' or 'Esc' to stop editing.")],
            _ => vec![],
        }
    })
    .block(mode_block);

    frame.render_widget(mode_widget, area);
}

fn _render_help_widget(frame: &mut Frame) {
    let popup_block = Block::bordered()
        .title(" Help ")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    let exit_text = Line::styled("Test", Style::default().fg(THEME_COLOR));
    let exit_paragraph = Paragraph::new(exit_text).block(popup_block);
    let area = centered_rect(60, 25, frame.size());
    frame.render_widget(exit_paragraph, area);
}

fn render_exit_widget(frame: &mut Frame) {
    frame.render_widget(Clear, frame.size());
    let popup_block = Block::bordered()
        .title(" Exit ")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(THEME_COLOR));

    let exit_text = Line::styled(
        "Would you like to quit todo? (Y/N)",
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
        .map(|(idx, todo_item)| {
            if app.selected == idx {
                match app.current_screen {
                    CurrentScreen::Selecting => {
                        return ListItem::new(Line::styled(
                            format!(" {} {}", todo_item.completion_box(), todo_item),
                            Style::default().fg(Color::Black).bg(THEME_COLOR),
                        ));
                    }
                    CurrentScreen::Editing => {
                        return ListItem::new(Line::styled(
                            format!(">{} {}", todo_item.completion_box(), todo_item),
                            Style::default().fg(Color::Black).bg(THEME_COLOR),
                        ));
                    }
                    _ => {
                        return ListItem::new(Line::styled(
                            format!(" {} {}", todo_item.completion_box(), todo_item),
                            Style::default().fg(THEME_COLOR),
                        ));
                    }
                }
            }
            // Default item appearance
            ListItem::new(Line::styled(
                format!(" {} {}", todo_item.completion_box(), todo_item),
                Style::default().fg(THEME_COLOR),
            ))
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
