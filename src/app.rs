use serde::{Deserialize, Serialize};
use std::{fmt, fs};

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub selected: usize,
    pub todo_list: Vec<TodoItem>,
}

impl App {
    // if get_saved_list errors, return vec with empty string to avoid selection errors
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            selected: 0,
            todo_list: get_saved_list().unwrap_or_default(),
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

// Struct used for deserializing saved data
#[derive(Debug, Deserialize, Serialize)]
struct TodoItems {
    items: Vec<TodoItem>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TodoItem {
    item: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn completion_box(&self) -> &str {
        if self.completed {
            return "[x]";
        }
        "[ ]"
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_empty()
    }

    pub fn mark_item(&mut self) {
        self.completed = !self.completed
    }

    pub fn push(&mut self, c: char) {
        self.item.push(c)
    }

    pub fn pop(&mut self) {
        self.item.pop();
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.item)
    }
}

// Get saved items from todo_list.json
fn get_saved_list() -> anyhow::Result<Vec<TodoItem>> {
    let json = String::from_utf8(fs::read("todo_list.json")?)?;
    let saved_items: TodoItems = serde_json::from_str(&json)?;
    Ok(saved_items.items)
}

// Write items to todo_list.json
pub fn save_list(todo_list: Vec<TodoItem>) -> anyhow::Result<()> {
    // filter out whitespace items
    let todo_list: Vec<TodoItem> = todo_list
        .into_iter()
        .filter(|x| x.item.trim() != "")
        .collect();
    let saved_items = TodoItems { items: todo_list };
    fs::write(
        "todo_list.json",
        serde_json::to_string(&saved_items).unwrap(),
    )?;

    Ok(())
}
