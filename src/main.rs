use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection::{get_data, CityInfo};
use ratatui_templates::event::EventHandler;
//use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui_templates::tui::{self, Tui};
use reqwest::blocking::get;
use reqwest::Error;
use tokio::io::stderr;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::terminal::{LeaveAlternateScreen};
use crossterm::event::DisableMouseCapture;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    // let app = 
    // Initialize the terminal user interface.
    let mut app: App = App::new();
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    // TODO:  the terminal user interface
    // let mut tui =
    let mut tui = Tui::new(terminal, EventHandler::new(1));
    tui.init()?;
    while app.running {
        match get_data(app.searched_city.clone()).await {
            Ok(c) => tui.draw(&mut app, c),
            Err(_) => tui.draw(&mut app, CityInfo::default()),        
        };
        if let Event::Key(key) = event::read()? {
            handle_key_events(key, &mut app)?;
        }
        
    }
    tui.exit()?;
    
    
    // TODO: init the terminal

    // Start the main loop.
    // while app.running {
        // TODO: Render the user interface.

        // TODO: Handle events.
        
    // }


    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}
