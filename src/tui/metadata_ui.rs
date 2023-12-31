use rusqlite::Connection;
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, List, ListItem};
use tui::layout::{Layout, Constraint, Direction};
use crate::io;
use crate::db_func::metadata_func::{get_table_names, get_column_names};



pub enum Event<I> {
    Input(I),
    Tick,
}

pub(crate) fn run_tui(connection: &Connection) {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        loop {
            // Send tick rate
            tx.send(Event::Tick).unwrap();
            thread::sleep(tick_rate);
        }
    });

    let table_names = get_table_names(&connection).unwrap(); // Fetch table names

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Divide screen into vertical chunks for tables and columns
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                        .as_ref(),
                )
                .split(size);

            let tables: Vec<ListItem> = table_names.iter().map(|t| ListItem::new(t.as_ref())).collect();
            let tables = List::new(tables).block(Block::default().borders(Borders::ALL).title("Tables"));
            f.render_widget(tables, chunks[0]);

            // Assume first table is selected for demo, fetch and display its columns
            if let Ok(column_names) = get_column_names(&connection, &table_names[0]) {
                let columns: Vec<ListItem> = column_names.iter().map(|c| ListItem::new(c.as_ref())).collect();
                let columns = List::new(columns).block(Block::default().borders(Borders::ALL).title("Columns"));
                f.render_widget(columns, chunks[1]);
            }
        }).unwrap();

        match rx.recv().unwrap() {
            Event::Tick => {},
            Event::Input(event) => match event {
                CEvent::Key(key) => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
                _ => {}
            },
        }
    }
}
