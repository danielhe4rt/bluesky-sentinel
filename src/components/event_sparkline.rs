use crate::app::App;
use ratatui::prelude::{Color, Style};
use ratatui::symbols;
use ratatui::widgets::{Block, Sparkline, SparklineBar};

pub fn event_sparkline_view<'a>(app: &App) -> Sparkline<'a> {
    Sparkline::default()
        .block(Block::new().title("Sparkline:"))
        .style(Style::default().fg(Color::Green))
        .data(vec![SparklineBar::from(10); 300])
        .bar_set(if app.enhanced_graphics {
            symbols::bar::NINE_LEVELS
        } else {
            symbols::bar::THREE_LEVELS
        })
}