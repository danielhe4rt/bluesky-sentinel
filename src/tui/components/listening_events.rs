use crate::tui::app::App;
use ratatui::prelude::{Modifier, Span, Style};
use ratatui::text;
use ratatui::widgets::{Block, List, ListItem};

pub fn listening_events_view(app: &App) -> List {
    let events: Vec<ListItem> = app
        .listened_events
        .iter()
        .enumerate()
        .map(|(i, text)| {
            // render the selected state
            let style = if app.task_selected == i {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(vec![text::Line::from(Span::raw(text).style(style))])
        })
        .collect();

    List::new(events)
        .block(Block::bordered().title("Pick an event"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}
