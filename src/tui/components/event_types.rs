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
        .map(move |event| {
            let commit_type = event.event_commit_type.as_str();
            let user_did = event.user_did.clone();
            let s = match commit_type {
                "create" => info_style,
                "update" => critical_style,
                "delete" => warning_style,
                _ => info_style,
            };
            
            let content = vec![text::Line::from(vec![
                Span::styled(format!("{commit_type:<9}"), s),
                Span::raw(user_did),
            ])];
            ListItem::new(content)
        }).collect();
    List::new(logs).block(Block::bordered().title("Latest Logs"))
}
