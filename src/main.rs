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

use tui::app::{App, DeserializedNode};
use tui::crossterm::Tui;
use crate::database::create_session;
use tui::event_handler::{Event, EventHandler};
use ::crossterm::event::{KeyCode, KeyEventKind};
use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::sync::Arc;
use std::{error::Error, io, time::Duration};
use tokio::sync::Mutex;

mod database;
mod utils;
mod tui;
mod args;
mod jetstream;
mod repositories;
mod models;

/// Demo
#[derive(Debug, Parser)]
struct Cli {
    /// time in ms between two ticks.
    #[arg(short, long, default_value_t = 100)]
    tick_rate: u64,

    /// whether unicode symbols are used to improve the overall look of the app
    #[arg(short, long, default_value_t = true)]
    unicode: bool,
}
#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let session = create_session().await?;

    let app_session = Arc::clone(&session);

    let mut app = Arc::new(Mutex::new(App::new(
        "Bluesky Sentinel Demo - @scylladb".to_string(),
        cli.unicode,
    )));
    let db_app = Arc::clone(&app);
    let event_app = Arc::clone(&app);
    let db = Arc::clone(&app_session);
    tokio::spawn(async move {
        loop {
            db.query_iter("SELECT * FROM system.local", ()).await.unwrap();
            let metrics = db.get_metrics();
            let cluster = db.get_cluster_data();
            {
                let mut app = db_app.lock().await;
                app.metrics.update(metrics);
                app.nodes = DeserializedNode::transform_nodes(cluster.get_nodes_info());
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });



    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
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
