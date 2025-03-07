use std::error;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]


pub enum AppView {
    SearchScreen,
    InfoScreen,
    MainMenu,
}
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub state: AppView,
    pub searched_city: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            state: AppView::MainMenu,
            searched_city: String::new(),
        }
    }
}
