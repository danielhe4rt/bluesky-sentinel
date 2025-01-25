use crate::tui::app::App;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Row, Table};

pub fn driver_metrics_widget(app: &App) -> Table {
    let driver_metrics_table = Table::new(
        vec![Row::new(vec![
            app.metrics.queries_count.to_string(),
            app.metrics.iter_queries_count.to_string(),
            app.metrics.errors_count.to_string(),
            app.metrics.iter_errors_count.to_string(),
            app.metrics.avg_latency.to_string(),
            app.metrics.p99_latency.to_string(),
            app.metrics.p50_latency.to_string(),
        ])],
        [
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ],
    )
    .header(
        Row::new(vec![
            "Queries Reqs",
            "Iter Queries Reqs",
            "Errors",
            "Iter Errors",
            "Avg Latency",
            "P99",
            "P50",
        ])
        .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1),
    )
    .block(Block::bordered().title("Driver Metrics"));
    driver_metrics_table
}
