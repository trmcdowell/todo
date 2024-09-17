use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude,
    style::{palette::tailwind, Color, Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget,
        Widget,
    },
};

use crate::app::{App, Mode, TodoItem};

const THEME_COLOR: Color = tailwind::EMERALD.c500;

impl Widget for &mut App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer) {
        let layout =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).split(area);

        render_todo_widget(self, layout[0], buf);
        render_info_widget(self, layout[1], buf);
    }
}

fn render_todo_widget(app: &mut App, area: prelude::Rect, buf: &mut prelude::Buffer) {
    let main_block = Block::bordered()
        .title(" TODO ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().fg(THEME_COLOR));

    // Iterate through all elements in the `items` and stylize them.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .enumerate()
        .map(|(item_idx, todo_item)| {
            todo_item.to_list_item(app.items.state.selected(), item_idx, &app.current_mode)
        })
        .collect();

    let todo_list_widget = List::new(items)
        .block(main_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(todo_list_widget, area, buf, &mut app.items.state);
}

fn render_info_widget(app: &App, area: prelude::Rect, buf: &mut prelude::Buffer) {
    let info_block = Block::bordered()
        .title(format!(" Current Mode: {} ", app.current_mode))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().fg(THEME_COLOR));

    let info_widget = Paragraph::new({
        match app.current_mode {
            Mode::Selecting => {
                vec![Line::raw(
                    "[j] and [k] select item, [n]ew item, [c]omplete item, [h] unselect",
                )]
            }
            Mode::Editing => {
                vec![Line::raw("[Enter] or [Esc] to stop editing")]
            }
        }
    })
    .block(info_block);

    Widget::render(info_widget, area, buf)
}

impl TodoItem {
    fn to_list_item(&self, selected_idx: Option<usize>, item_idx: usize, mode: &Mode) -> ListItem {
        let line = match self.status {
            false => {
                // Check if editing cursor is needed for line
                let text = if selected_idx.is_some()
                    && selected_idx.unwrap() == item_idx
                    && mode == &Mode::Editing
                {
                    format!(" ☐ {}_", self.text)
                } else {
                    format!(" ☐ {}", self.text)
                };
                Line::styled(text, Style::default())
            }
            true => {
                let text = if selected_idx.is_some()
                    && selected_idx.unwrap() == item_idx
                    && mode == &Mode::Editing
                {
                    format!(" ✓ {}_", self.text)
                } else {
                    format!(" ✓ {}", self.text)
                };
                Line::styled(text, Style::default())
            }
        };

        ListItem::new(line)
    }
}
