//! # [Ratatui] Original Demo example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use crate::args::AppSettings;
use crate::database::create_caching_session;
use crate::jetstream::start_jetstream;
use crate::repositories::DatabaseRepository;
use clap::Parser;
use ::crossterm::event::{KeyCode, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use scylla::CachingSession;
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;
use tui::app::{App, DeserializedNode};
use tui::crossterm::Tui;
use tui::event_handler::{Event, EventHandler};

mod args;
mod database;
mod jetstream;
mod models;
mod repositories;
mod tui;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let app_settings = AppSettings::new();

    let session = create_caching_session().await?;

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

fn start_hydration(db_app: &Arc<Mutex<App>>, db: Arc<CachingSession>) {
    let db_app = Arc::clone(db_app);
    tokio::spawn(async move {
        loop {
            let db = db.get_session();
            let metrics = db.get_metrics();
            let cluster = db.get_cluster_data();
            {
                let mut app = db_app.lock().await;
                app.metrics.update(metrics);
                app.nodes = DeserializedNode::transform_nodes(cluster.get_nodes_info());
            }
            // info!("Hydrated app with metrics and cluster data");
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}

async fn start_terminal(app: &mut Arc<Mutex<App>>) -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    let event_app = Arc::clone(&app);
    let events = EventHandler::new(event_app, 250);

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
            Event::Quit => break,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
