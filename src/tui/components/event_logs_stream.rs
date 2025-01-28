use std::fmt::format;
use crate::tui::app::App;
use ratatui::prelude::{Color, Span, Style};
use ratatui::text;
use ratatui::widgets::{Block, List, ListItem};
use crate::models::materialized_views::events_by_type::EventsByType;

pub fn event_logs_stream_view(app: &App) -> List {
    // Draw logs
    let info_style = Style::default().fg(Color::Blue);

    let logs: Vec<ListItem> = app
        .recent_events
        .iter()
        .map(move |item| {
            let ts = item.event_at.format("%H:%M:%S");
            let output = format_output(item);

            let content = vec![text::Line::from(vec![
                Span::styled(format!("[ {ts:<9}]"), info_style),
                Span::from(format!("{:.150}", output.trim().trim_ascii())),
            ])];
            ListItem::new(content)
        })
        .collect();

    List::new(logs).block(Block::bordered().title("BlueSky Event Logs"))
}

fn format_output(event: &EventsByType) -> String {
    let event_commit_type = event.event_commit_type.as_str();
    let event_typ = event.event_type.as_str();

    match event_typ{
        "app.bsky.feed.post" => {
            match event_commit_type {
                "create" => {
                    format!("{} posted:  {:<9}", event.user_did, event.event_data.get("text").unwrap_or(&"".to_string()))
                },
                "update" => {
                    format!("{} updated: {:<9}", event.user_did, event.event_id)
                },
                "delete" => {
                    format!("{} deleted: {:<9}", event.user_did, event.event_id)
                },
                _ => "Unknown".to_string(),
            }
        },
        "app.bsky.feed.like" => {
            match event_commit_type {
                "create" => {
                    format!("{} liked:   {:<9}", event.user_did, event.event_id)
                },
                "update" => {
                    format!("{} updated: {:<9}", event.user_did, event.event_id)
                },
                "delete" => {
                    format!("{} deleted: {:<9}", event.user_did, event.event_id)
                },
                _ => "Unknown".to_string(),
            }
        },
        "app.bsky.feed.repost" => {
            match event_commit_type {
                "create" => {
                    format!("{} reposted: {:<9}", event.user_did, event.event_id)
                },
                "update" => {
                    format!("{} updated: {:<9}", event.user_did, event.event_id)
                },
                "delete" => {
                    format!("{} deleted: {:<9}", event.user_did, event.event_id)
                },
                _ => "Unknown".to_string(),
            }
        },
        _ => "Unknown".to_string(),
    }


}