use rusqlite::Connection;
use crossterm::event::{self, Event as CEvent, KeyCode};

use crossterm::{execute, terminal};
use crossterm::terminal::ClearType;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, List, ListItem};
use crate::db_func::metadata_func::{get_table_names, get_column_names};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
};
use std::{fs::OpenOptions, io::Write};
use std::time::{Duration, Instant};

struct AppState {
    selected_table_index: usize,
}


pub(crate) fn run_tui(connection: &Connection) {
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app_state = AppState { selected_table_index: 0 };
    let table_names = get_table_names(connection).unwrap();
    //clear terminal
    terminal::enable_raw_mode().expect("TODO: panic message");
    execute!(
    std::io::stdout(),
    terminal::Clear(ClearType::All),
    terminal::LeaveAlternateScreen
).expect("TODO: panic message");
    println!("Press q to exit from this mode");
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("tui_log.txt")
        .expect("Unable to open log file");
    let mut prev_key_code: Option<KeyCode> = None;
    let mut last_keypress_time = Instant::now();



    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let tables: Vec<ListItem> = table_names.iter().enumerate().map(|(i, t)| {
                let style = if i == app_state.selected_table_index {
                    writeln!(
                        log_file,
                        "To render Current Index: {}, Current Table: {}",
                        app_state.selected_table_index, &table_names[app_state.selected_table_index]
                    ).expect("Unable to write to log file");
                    Style::default().add_modifier(tui::style::Modifier::REVERSED)
                } else {
                    Style::default()
                };
                ListItem::new(Text::raw(t)).style(style)
            }).collect();


            let tables = List::new(tables).block(Block::default().borders(Borders::ALL).title("Tables"));


            f.render_widget(tables, chunks[0]);

            // Assume first table is selected for demo, fetch and display its columns
            if let Ok(column_names) = get_column_names(connection, &table_names[app_state.selected_table_index]) {
                let columns: Vec<ListItem> = column_names.iter().map(|c| ListItem::new(Text::raw(c))).collect();
                let columns = List::new(columns).block(Block::default().borders(Borders::ALL).title("Columns"));
                f.render_widget(columns, chunks[1]);
            }
        }).unwrap();


        if let CEvent::Key(key) = event::read().unwrap() {
            let current_key_code = key.code;
            let now = Instant::now();

            // Calculate the time elapsed since the last keypress.
            let time_since_last_keypress = now.duration_since(last_keypress_time);
            if time_since_last_keypress >= Duration::from_millis(100) {

            // Compare the current key code to the previous key code.
            //if Some(current_key_code) != prev_key_code {

                match current_key_code  {
                KeyCode::Up => {
                    if app_state.selected_table_index > 0 {
                        app_state.selected_table_index -= 1;
                        writeln!(
                            log_file,
                            "Index decremented to: {} - {}",
                            app_state.selected_table_index, table_names[app_state.selected_table_index]
                        ).expect("Unable to write to log file");
                    }
                }
                KeyCode::Down => {
                    if app_state.selected_table_index < table_names.len() - 1 {
                        app_state.selected_table_index += 1;
                        writeln!(
                            log_file,
                            "Index incremented to: {} - {}",
                            app_state.selected_table_index, table_names[app_state.selected_table_index]
                        ).expect("Unable to write to log file");
                    }
                }
                KeyCode::Char('q') => {
                    execute!(
    std::io::stdout(),
    terminal::Clear(ClearType::All),
    terminal::LeaveAlternateScreen
).expect("TODO: panic message");

                    terminal::disable_raw_mode().expect("TODO: panic message");
                    break;
                }
                _ => {}
            }
        }
            last_keypress_time = now;
    }
            //prev_key_code = Some(key.code);}
}}
