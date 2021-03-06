#[macro_use]
extern crate lazy_static;
extern crate image;

use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use cursive::views::Layer;
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::{fs, net::ToSocketAddrs};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Sparkline,
    },
    Terminal,
};
mod mapgen;
use mapgen::{Map, MapMeta};

const DB_PATH: &str = "./data/db.json";
const MAP_PATH: &str = "./data/map.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Serialize, Deserialize, Clone)]
struct Login{
    uname: String,
    password: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Character {
    id: usize,
    login: Login,
    vitals: u8,
    spirit: u8
}


#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Sheet,
    Map
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Sheet => 0,
            MenuItem::Map => 1,
        }
    }
}


#[derive(Copy, Clone, Debug)]
enum MapState {
    Welcome,
    Map,
    Conflict,
    Converse
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let map = read_map().unwrap();
    let mut active_map_state = MapState::Map;

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Sheet", "Map"];
    let mut active_menu_item = MenuItem::Map;

    let mut me = Character {
        id: 1,
        login: Login { uname: "Samhain".to_owned() , password: "pass".to_owned() },
        vitals: 100,
        spirit: 20
    };
    

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let uiFrame = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(20),
                        Constraint::Min(2)
                    ]
                    .as_ref(),
                )
                .split(size);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(tui::layout::Rect {
                    x: 20,
                    y: 0,
                    width: size.width-20,
                    height: size.height,
                });

            
            let heart_rate = Sparkline::default()
            .block(Block::default().title("Heart Rate").borders(Borders::ALL))
            .data(&[0, 2, 3, 4, 1, 4, 10])
            .max(5)
            .style(Style::default());

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(render_info_tab(&me), uiFrame[0]);
            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Map => rect.render_widget(render_map( &active_map_state), chunks[1]),
                MenuItem::Sheet => rect.render_widget(render_sheet(), chunks[1])
            }
            rect.render_widget(heart_rate, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('m') => active_menu_item = MenuItem::Map,
                KeyCode::Char('h') => active_menu_item = MenuItem::Sheet,
                _ => {}
                KeyCode::Char('w') => {
                    match active_menu_item{
                        MenuItem::Map => {
                            //go up
                        }
                        MenuItem::Sheet => todo!(),
                    }
                },
                KeyCode::Char('a') => {
                    match active_menu_item{
                        MenuItem::Map => {
                            //go left
                        }
                        MenuItem::Sheet => todo!(),
                    }
                },
                KeyCode::Char('s') => {
                    match active_menu_item{
                        MenuItem::Map => {
                            //go down
                        }
                        MenuItem::Sheet => todo!(),
                    }
                },
                KeyCode::Char('d') => {
                    match active_menu_item{
                        MenuItem::Map => {
                            //go right
                        }
                        MenuItem::Sheet => todo!(),
                    }
                },
                KeyCode::Char('l')=>{
                    //move map to draw map instead of home
                    active_map_state = MapState::Map;
                    println!("{:?}", active_map_state);
                }
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_info_tab<'a>(charStats: &Character) -> Paragraph<'a>{
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(charStats.login.uname.to_string())]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Info")
            .border_type(BorderType::Plain),
    );
    return home
}

fn render_map<'a>( mapState: &MapState) -> Paragraph<'a> {
    
    let map = read_map().unwrap();

    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Gibbering Mouther",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Map")
            .border_type(BorderType::Plain),
    );
    
    let mut span_vec = vec![Spans::from(vec![Span::raw("")]); map.meta.height];
    let mut i =0;
    for s in map.map{
        span_vec[i] = Spans::from(vec![Span::raw(s)]);
        i+=1;
    }   
    let map_view = Paragraph::new(span_vec)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Map")
                .border_type(BorderType::Plain),
        );
        
    match mapState {
        MapState::Welcome =>{
            return home;
        },
        MapState::Map => {
            return map_view;
        },
        MapState::Conflict => todo!(),
        MapState::Converse => todo!(),
    }

    return home;
}

fn render_sheet<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("sheet here")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Sheet")
            .border_type(BorderType::Plain),
    );
    home
}

fn read_map() -> Result<Map, Error>{

    let db_content = fs::read_to_string(MAP_PATH)?;
    
    let parsed: Map = serde_json::from_str(&db_content)?;

    Ok(parsed)
}

