#[derive(PartialEq)]
pub enum CurrentScreen {
    Editing,
    Exiting,
    Main,
    Selecting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected: usize,
    pub todo_list: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            selected: 0,
            todo_list: vec!["test 1".to_string(), "test 2".to_string()],
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
