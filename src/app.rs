use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::Backend, widgets::ListState, Terminal};
use serde::{Deserialize, Serialize};
use std::{fmt, io};

#[derive(Debug)]
pub struct App {
    pub current_mode: Mode,
    pub todo_list: TodoList,
}

impl App {
    pub fn new(items: Vec<TodoItem>) -> App {
        App {
            current_mode: Mode::Selecting,
            todo_list: TodoList::from_items(items),
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
        loop {
            self.draw(terminal)?;

            // Main app logic
            if let Event::Key(key) = event::read()? {
                // Skip key releases
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match self.current_mode {
                    // Main screen
                    Mode::Selecting => match key.code {
                        KeyCode::Char('c') => match key.modifiers {
                            KeyModifiers::CONTROL => return Ok(()),
                            KeyModifiers::NONE => {
                                if let Some(idx) = self.todo_list.state.selected() {
                                    self.todo_list.items[idx].change_status()
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('h') => self.todo_list.unselect(),
                        KeyCode::Char('j') | KeyCode::Down => self.todo_list.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.todo_list.previous(),
                        KeyCode::Char('n') => {
                            self.todo_list.items.push(TodoItem::default());
                            self.todo_list.last_selected = self.todo_list.state.selected();
                            self.todo_list
                                .state
                                .select(Some(self.todo_list.items.len() - 1));
                        }
                        KeyCode::Char('d') => {
                            if let Some(idx) = self.todo_list.state.selected() {
                                self.todo_list.items.remove(idx);
                                if self.todo_list.items.is_empty() {
                                    self.todo_list.unselect()
                                } else if idx == self.todo_list.items.len() {
                                    self.todo_list.state.select(Some(idx - 1));
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if self.todo_list.state.selected().is_some() {
                                self.current_mode = Mode::Editing
                            }
                        }
                        _ => {}
                    },
                    // Editing mode. Can only enter mode if an item is selected.
                    Mode::Editing => match (key.code, key.modifiers) {
                        (KeyCode::Char('c') | KeyCode::Char('C'), KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        (KeyCode::Esc | KeyCode::Enter, _) => self.current_mode = Mode::Selecting,
                        (KeyCode::Backspace | KeyCode::Delete, _) => {
                            let item_idx = self.todo_list.state.selected().unwrap();
                            self.todo_list.items[item_idx].text.pop();
                        }
                        (KeyCode::Char(char), _) => {
                            let item_idx = self.todo_list.state.selected().unwrap();
                            self.todo_list.items[item_idx].text.push(char);
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|f| f.render_widget(self, f.size()))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Editing,
    Selecting,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Editing => write!(f, "Editing"),
            Self::Selecting => write!(f, "Selecting"),
        }
    }
}

#[derive(Debug)]
pub struct TodoList {
    pub state: ListState,
    pub items: Vec<TodoItem>,
    pub last_selected: Option<usize>,
}

impl TodoList {
    fn _new() -> Self {
        TodoList {
            state: ListState::default(),
            items: Vec::new(),
            last_selected: None,
        }
    }

    pub fn from_items(items: Vec<TodoItem>) -> Self {
        TodoList {
            state: ListState::default(),
            items,
            last_selected: None,
        }
    }

    fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };

        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

// Struct used for deserializing saved data
#[derive(Debug, Default, Deserialize, Serialize)]
struct TodoItems {
    pub items: Vec<TodoItem>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TodoItem {
    pub text: String,
    pub status: bool,
}

impl TodoItem {
    fn _new(item: &str, status: bool) -> Self {
        TodoItem {
            text: item.to_string(),
            status,
        }
    }

    fn change_status(&mut self) {
        self.status = !self.status;
    }
}
