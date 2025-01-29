#![feature(iterator_try_collect)]

use crate::args::AppSettings;
use crate::database::{create_caching_session, create_session};
use crate::jetstream::start_jetstream;
use crate::repositories::DatabaseRepository;
use ::crossterm::event::{KeyCode, KeyEventKind};

use crate::tui::hydrate::start_hydration;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::sync::Arc;
use std::{error::Error, io};
use tokio::sync::Mutex;
use tui::app::App;
use tui::crossterm::Tui;
use tui::event_handler::{Event, EventHandler};
use crate::tui::app::DeserializedNode;

mod args;
mod database;
mod jetstream;
mod models;
mod repositories;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let app_settings = AppSettings::new();

    let session = create_caching_session(&app_settings).await?;

    let app_session = Arc::clone(&session);
    let hydration_session = Arc::clone(&session);

    let mut app = App::new(app_settings.clone());

    let repository = Arc::new(DatabaseRepository::new(app_session));

    tokio::spawn(async move {
        let _ = start_jetstream(app_settings, &repository).await;
    });

    start_hydration(&mut app, hydration_session);

    start_terminal(&mut app).await?;
    Ok(())
}

async fn start_terminal(app: &mut Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    // Start the main loop.
    loop {
        // Render the user interface.

        let mut app = app.lock().await;
        tui.draw(&mut app)?;

        // Handle events.
        if app.should_quit {
            break;
        }

        match tui.events.next().await? {
            Event::Tick => app.on_tick(),
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Left | KeyCode::Char('h') => app.on_left(),
                        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
                        KeyCode::Right | KeyCode::Char('l') => app.on_right(),
                        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
