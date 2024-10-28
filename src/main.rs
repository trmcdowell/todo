mod app;
mod ui;

use std::{
    fs,
    io::{self, stdout, Stdout},
};

use app::{App, TodoItem, TodoList};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    let mut terminal = init_terminal()?;

    // Setup and run app
    let saved_items = get_saved_list().unwrap_or_default();
    let mut app = App::new(saved_items);
    let result = app.run(&mut terminal);

    // Restore terminal after quitting app
    restore_terminal()?;

    // Save list state data
    save_todo_list(app.todo_list)?;

    // Print error if one occured
    if let Err(e) = result {
        println!("{e:?}");
    }

    Ok(())
}

pub fn init_terminal() -> anyhow::Result<Tui> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(io::stdout());
    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

/// Get saved items from todo_list.json
/// Note that path to todo_list.json may not work on windows (/ instead of \\)
fn get_saved_list() -> anyhow::Result<Vec<TodoItem>> {
    let path = format!(
        "{}/todo",
        dirs::config_dir()
            .expect("Could not find config dir")
            .to_str()
            .unwrap()
    );
    std::fs::create_dir_all(&path)?;
    let json = String::from_utf8(fs::read(format!("{}/todo_list.json", path))?)?;
    let saved_items: Vec<TodoItem> = serde_json::from_str(&json)?;
    Ok(saved_items)
}
// Write items to todo_list.json
pub fn save_todo_list(todo_list: TodoList) -> anyhow::Result<()> {
    let path = format!(
        "{}/todo/todo_list.json",
        dirs::config_dir()
            .expect("Could not find config dir")
            .to_str()
            .unwrap()
    );
    let save_items: Vec<TodoItem> = todo_list
        .items
        .into_iter()
        .filter(|item| !item.text.is_empty())
        .collect();
    fs::write(path, serde_json::to_string(&save_items).unwrap())?;
    Ok(())
}
