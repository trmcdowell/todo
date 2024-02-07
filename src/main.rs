mod app;
mod ui;

use std::io;

use app::{save_list, App, CurrentScreen};
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
    let _ = save_list(app.todo_list)?;

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
                    KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Selecting;
                        if app.todo_list.is_empty() {
                            app.todo_list.push("".to_string());
                        }
                    }
                    _ => {}
                },
                // Selecting screen
                CurrentScreen::Selecting => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Main,
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.increment_selected(),
                    KeyCode::Char('k') | KeyCode::Up => app.decrement_selected(),
                    KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        app.todo_list.push("".to_string());
                        app.increment_selected();
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        app.todo_list.remove(app.selected);
                        if app.selected == 0 {
                            app.todo_list.push("".to_string())
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
                        if app.todo_list[app.selected].is_empty() {
                            app.todo_list.remove(app.selected);
                        }
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