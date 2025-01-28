use crate::tui::app::App;
use ratatui::prelude::{Modifier, Span, Style};
use ratatui::style::Color;
use ratatui::text;
use ratatui::widgets::{Block, List, ListItem};
use text::Line;

pub fn listening_events_view(app: &App) -> List {
    let events: Vec<ListItem> = app
        .listened_events
        .iter()
        .enumerate()
        .map(|(i, event)| {
            // render the selected state
            let style = if app.selected_event == i {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::raw(event.event_type.clone()).style(style),
                    Span::raw(format!(" {}", event.created_count.unwrap_or_default().0)).style(Style::default().fg(Color::Green)),
                    Span::raw(format!(" {}", event.updated_count.unwrap_or_default().0)).style(Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" {}", event.deleted_count.unwrap_or_default().0)).style(Style::default().fg(Color::Red)),
                ]),
            ])
        })
        .collect();

    List::new(events)
        .block(Block::bordered().title("Jetstream Events"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}
