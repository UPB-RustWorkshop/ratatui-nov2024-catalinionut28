use crossterm::style::Stylize;
use ratatui::{layout::{self, Constraint, Layout, Rect}, style::{Style, Stylize as OtherStylize}, text::{Span, Text}, widgets::{block, Block, Borders, Padding, Paragraph, Wrap}, Frame};
use crate::{app::{App, AppResult}, connection::{get_data, CityInfo, MyError}};
use crate::app::AppView;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, city: CityInfo) {
    match app.state {
        AppView::MainMenu => render_main_menu(frame),
        AppView::SearchScreen => {
            match render_search_bar(frame, app) {
                Ok(_) => return,
                Err(_) => return
            }    
        }
        AppView::InfoScreen => render_weather_info(frame, city),
    }


}

fn render_weather_info(frame: &mut Frame, city: CityInfo) { 
    let area = frame.size();
    let main_layout = Layout::new(ratatui::layout::Direction::Vertical, 
        [Constraint::Length(1), Constraint::Min(0)]);
    let block_layout = Layout::new(ratatui::layout::Direction::Vertical,
    [Constraint::Max(10); 3]);
    let [title_area, main_area] = *main_layout.split(area) else { return; };
    let main_areas: Vec<_> = block_layout.split(main_area)
                    .iter().map(|&area| {
                        Layout::new(ratatui::layout::Direction::Horizontal, 
                            [Constraint::Percentage(50), Constraint::Percentage(50)])
                            .split(area)
                            .to_vec()
                    }).collect();
    frame.render_widget(Paragraph::new("Weather info. Press Ctrl + S to search a city or Ctrl + Q to quit.")
    .alignment(ratatui::layout::Alignment::Center), title_area);
    let mut paragraph = place_paragraph(city.name);
    render_title(&paragraph, frame, main_areas[0][0], &String::from("City"));
    let temperature = (city.temperature * 2.0).floor() / 2.0;
    format!("{:.1}", temperature);
    let temp = String::from(temperature.to_string()) + &String::from(" °C");
    paragraph = place_paragraph(temp);
    render_title(&paragraph, frame, main_areas[0][1], &String::from("Temperature"));
    paragraph = place_paragraph(city.description);
    render_title(&paragraph, frame, main_areas[1][0], &String::from("Description"));
    let feels_like = ((city.feels_like * 2.0).floor() / 2.0).to_string();
    format!("{:.1}", feels_like);
    let fl = String::from(feels_like.to_string()) + &String::from(" °C");
    paragraph = place_paragraph(fl);
    render_title(&paragraph, frame, main_areas[1][1], &String::from("Apparent Temperature"));
    let humidity = city.humidity.to_string();
    let hum = humidity.to_string() + &String::from("%");
    paragraph = place_paragraph(hum);
    render_title(&paragraph, frame, main_areas[2][0], &String::from("Humidity (%)"));
    let date_time = city.date_time.to_string();
    paragraph = place_paragraph(date_time);
    render_title(&paragraph, frame, main_areas[2][1], &"Date Time".to_string());


    
    
    
    
    


}


fn render_title(paragraph: &Paragraph, frame: &mut Frame, area: Rect, title: &String) {
    let block =  Block::default()
    .borders(Borders::ALL)
    .title(title.as_str());
    frame.render_widget(paragraph.clone().block(block), area);
}


fn place_paragraph(paragraph: String) -> Paragraph<'static> {
    let colored_paragraph = Span::styled(paragraph, Style::default().fg(ratatui::prelude::Color::White));
    Paragraph::new(Text::from(colored_paragraph)).wrap(Wrap { trim: true })  
}


fn render_main_menu(frame: &mut Frame) {
    let area = frame.size();
    let main_text = "Welcome!\nPress Ctrl + S to search the weather of a city."
                    .to_string();
    let p = Paragraph::new(Text::from(main_text))
        .block(Block::new().style(Style::new()
        .fg(ratatui::prelude::Color::White))
        .padding(Padding::new(0, 0, area.height / 2, 0)))
        .alignment(layout::Alignment::Center);
    frame.render_widget(p, area);
}

fn render_search_bar(frame: &mut Frame, app: &mut App) -> AppResult<()> {
    let area = frame.size();
    let search_bar = create_rectangle_centered(50, 3, area);
    let block = Block::new().borders(Borders::ALL)
    .title("Please type to search a city.");
    let paragraph = Paragraph::new(Text::from(app.searched_city.clone()))
                                .block(block.clone());
    frame.render_widget(paragraph.clone(), search_bar);
            
    Ok(())
}



fn create_rectangle_centered(width: u16, height: u16, area: Rect) -> Rect {
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    Rect {x, y, width, height}
}    