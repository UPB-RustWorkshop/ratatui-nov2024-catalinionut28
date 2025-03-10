use crate::app::{App, AppResult, AppView};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};


pub struct SearchError(String);

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, key_event.modifiers) {
        // TODO: define actions for quitting the app
        (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
            app.running = false;
        }
        // Search a city
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
            app.state = AppView::SearchScreen;
        }

        _ => {
            match app.state {
                AppView::SearchScreen => {
                    match key_event.code {
                        KeyCode::Char(c) => app.searched_city.push(c),
                        KeyCode::Delete | KeyCode::Backspace => {
                            app.searched_city.pop();    
                        }
                        KeyCode::Esc => {
                            app.state = AppView::MainMenu;
                            app.searched_city.clear();
                        }
                        KeyCode::Enter => {
                            app.state = AppView::InfoScreen;
                        }
                        KeyCode::CapsLock => {},
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}


