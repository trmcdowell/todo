use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Editing,
    Exiting,
    Main,
    Selecting,
}

impl ToString for CurrentScreen {
    fn to_string(&self) -> String {
        match self {
            Self::Editing => String::from("Editing"),
            Self::Exiting => String::from("Exiting"),
            Self::Main => String::from("Main"),
            Self::Selecting => String::from("Selecting"),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct SavedItems {
    items: Vec<String>,
}

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub selected: usize,
    pub todo_list: Vec<String>,
}

impl App {
    // if get_saved_list errors, return vec with empty string to avoid selection errors
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            selected: 0,
            todo_list: get_saved_list().unwrap_or(vec![]),
        }
    }

    pub fn increment_selected(&mut self) {
        if self.selected < self.todo_list.len() - 1 {
            self.selected += 1;
        }
    }

    // != because val < 0 is not possible with usize and it may error
    pub fn decrement_selected(&mut self) {
        if self.selected != 0 {
            self.selected -= 1;
        }
    }
}

// Get saved items from todo_list.json
fn get_saved_list() -> anyhow::Result<Vec<String>> {
    let json = String::from_utf8(fs::read("todo_list.json")?)?;
    let saved_items: SavedItems = serde_json::from_str(&json)?;
    Ok(saved_items.items)
}

// Write items to todo_list.json
pub fn save_list(todo_list: Vec<String>) -> anyhow::Result<()> {
    // filter out whitespace items
    let todo_list = todo_list
        .iter()
        .map(|x| x.trim().to_string())
        .filter(|x| x != "")
        .collect();
    let saved_items = SavedItems { items: todo_list };
    fs::write(
        "todo_list.json",
        serde_json::to_string(&saved_items).unwrap(),
    )?;

    Ok(())
}
