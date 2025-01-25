use crate::tui::app::App;
use ratatui::prelude::{Color, Span, Style};
use ratatui::text;
use ratatui::widgets::{Block, List, ListItem};

pub fn event_logs_stream_view(app: &App) -> List {
    // Draw logs
    let info_style = Style::default().fg(Color::Blue);

    let logs: Vec<ListItem> = app
        .recent_events
        .iter()
        .map(move |item| {
            let level = item.user_did.as_str();
            let ts = item.event_at.format("%Y-%m-%d %H:%M:%S").to_string();

            let content = vec![text::Line::from(vec![
                Span::styled(format!("{ts:<9}"), info_style),
                Span::raw(level),
            ])];
            ListItem::new(content)
        })
        .collect();

    List::new(logs).block(Block::bordered().title("BlueSky Event Logs"))
}
