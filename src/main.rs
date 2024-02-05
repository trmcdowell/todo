mod app;
mod ui;

use std::io;

use app::{App, CurrentScreen};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
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
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

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

        if let Event::Key(key) = event::read()? {
            // Skip key releases
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                // Main screen
                CurrentScreen::Main => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    _ => {}
                },
                // Editing screen
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Main,
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.increment_selected(),
                    KeyCode::Char('k') | KeyCode::Up => app.decrement_selected(),
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(()),
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
