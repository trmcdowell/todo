pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct App {
    pub todo_input: String,
    pub todo_items: Vec<String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<u32>,
}

impl App {
    pub fn new() -> App {
        App {
            todo_input: String::new(),
            todo_items: Vec::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}
