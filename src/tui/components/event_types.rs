use crate::tui::app::App;
use ratatui::prelude::{Color, Span, Style};
use ratatui::text;
use ratatui::widgets::{Block, List, ListItem};

pub fn event_types_view(app: &App) -> List {
    let info_style = Style::default().fg(Color::Blue);
    let warning_style = Style::default().fg(Color::Yellow);
    let error_style = Style::default().fg(Color::Magenta);
    let critical_style = Style::default().fg(Color::Red);
    let logs: Vec<ListItem> = app
        .recent_events
        .iter()
        .map(move |(evt, level)| {
            let level = level.as_str();
            let s = match level {
                "ERROR" => error_style,
                "CRITICAL" => critical_style,
                "WARNING" => warning_style,
                _ => info_style,
            };
            let content = vec![text::Line::from(vec![
                Span::styled(format!("{level:<9}"), s),
                Span::raw(evt),
            ])];
            ListItem::new(content)
        })
        .collect();
    List::new(logs).block(Block::bordered().title("Latest Logs"))
}
