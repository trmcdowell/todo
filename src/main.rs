mod app;
mod ui;

use std::io;

use app::{save_todo_list, App, CurrentScreen, TodoItem};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Setup app
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);
    save_todo_list(app.todo_list)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // Print error if one occurs
    if let Err(e) = result {
        println!("{e:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        // Main app logic
        if let Event::Key(key) = event::read()? {
            // Skip key releases
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                // Main screen
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('e')
                    | KeyCode::Char('E')
                    | KeyCode::Char('j')
                    | KeyCode::Char('J')
                    | KeyCode::Char('k')
                    | KeyCode::Char('K')
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Selecting;
                    }
                    KeyCode::Char('d') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            app.remove_completed_items()
                        }
                    }
                    _ => {}
                },
                // Selecting screen
                CurrentScreen::Selecting => match key.code {
                    // Exit selecting
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main;
                        app.remove_empty_items();
                    }

                    // Mark item completed with c or x, quit app with ctrl + c
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                        if !app.todo_list.is_empty() {
                            app.todo_list[app.selected].mark_item();
                        }
                    }
                    KeyCode::Char('x') | KeyCode::Char('X') => {
                        if !app.todo_list.is_empty() {
                            app.todo_list[app.selected].mark_item();
                        }
                    }

                    // Item selection
                    KeyCode::Char('j') | KeyCode::Down => app.increment_selected(),
                    KeyCode::Char('k') | KeyCode::Up => app.decrement_selected(),

                    // Edit selected item
                    KeyCode::Char('a')
                    | KeyCode::Char('A')
                    | KeyCode::Char('e')
                    | KeyCode::Char('E')
                    | KeyCode::Enter => {
                        if app.todo_list.is_empty() {
                            app.todo_list.push(TodoItem::default());
                            app.increment_selected();
                        }
                        app.current_screen = CurrentScreen::Editing;
                    }

                    // Add new item
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        app.todo_list.push(TodoItem::default());
                        app.increment_selected();
                    }

                    // Delete an item with d or all completed items with ctrl + d
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            app.remove_completed_items()
                        } else if !app.todo_list.is_empty() {
                            app.todo_list.remove(app.selected);
                            if app.selected == app.todo_list.len() {
                                app.decrement_selected()
                            }
                        }
                    }
                    _ => {}
                },
                // Editing mode
                CurrentScreen::Editing => match (key.code, key.modifiers) {
                    (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    (KeyCode::Esc | KeyCode::Enter, _) => {
                        app.current_screen = CurrentScreen::Selecting;
                    }
                    (KeyCode::Backspace | KeyCode::Delete, _) => {
                        app.todo_list[app.selected].pop();
                    }
                    (KeyCode::Char(char), _) => {
                        app.todo_list[app.selected].push(char);
                    }
                    _ => {}
                },
                // Exit screen
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        return Ok(());
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
            }
        }
    }
}
