#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected: usize,
    pub todo_input: String,
    pub todo_list: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            selected: 0,
            todo_input: String::new(),
            todo_list: vec!["test 1".to_string(), "test 2".to_string()],
        }
    }

    pub fn increment_selected(&mut self) {
        if self.selected != self.todo_list.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn decrement_selected(&mut self) {
        if self.selected != 0 {
            self.selected -= 1;
        }
    }
}
