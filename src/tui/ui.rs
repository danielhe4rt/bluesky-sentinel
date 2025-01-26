use crate::tui::app::App;
use crate::tui::components::driver_metrics::driver_metrics_widget;
use crate::tui::components::event_logs_stream::event_logs_stream_view;
use crate::tui::components::event_sparkline::event_sparkline_view;
use crate::tui::components::event_types::event_types_view;
use crate::tui::components::select_events::listening_events_view;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{
        canvas::{self, Canvas, Circle, Map, MapResolution},
        Block, Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title.clone()))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(frame, app, chunks[1]),
        1 => draw_world_map_tab(frame, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ])
    .split(area);
    draw_gauges(frame, app, chunks[0]);
    draw_charts(frame, app, chunks[1]);
    draw_text(frame, chunks[2]);
}

fn draw_gauges(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Percentage(100)])
        .margin(1)
        .split(area);
    let block = Block::bordered().title("Graphs");
    frame.render_widget(block, area);

    let sparkline = event_sparkline_view(app);
    frame.render_widget(sparkline, chunks[0]);
}

#[allow(clippy::too_many_lines)]
fn draw_charts(frame: &mut Frame, app: &mut App, area: Rect) {
    let constraints = vec![Constraint::Percentage(20), Constraint::Percentage(80)];
    let chunks = Layout::horizontal(constraints).split(area);
    let sidebar_view = chunks[0];
    let logs_view = chunks[1];
    {
        let events_view = Layout::vertical([Constraint::Percentage(100)]).split(sidebar_view);
        let events_view = events_view[0];
        {
            let chunks = Layout::horizontal([Constraint::Percentage(100)]).split(events_view);
            let event_types = chunks[0];

            let current_events = listening_events_view(app);
            frame.render_widget(current_events, event_types);
        }
    }

    let logs = event_logs_stream_view(app);
    frame.render_widget(logs, logs_view);
}

fn draw_text(frame: &mut Frame, area: Rect) {
    let text = vec![
        text::Line::from(vec![
            Span::from("This is a "),
            Span::styled("TUI", Style::default().fg(Color::Red)),
            Span::raw(" + "),
            Span::styled("ScyllaDB", Style::default().fg(Color::Cyan)),
            Span::raw(" + "),
            Span::styled("BlueSky", Style::default().fg(Color::Blue)),
            Span::raw(" demo."),
        ]),
        text::Line::from(vec![
            Span::raw(
                "This project was done during my livecoding sessions, so don't forget to follow: ",
            ),
            Span::styled(
                "https://twitch.tv/danielhe4rt",
                Style::default()
                    .add_modifier(Modifier::UNDERLINED)
                    .fg(Color::Magenta),
            ),
        ]),
        text::Line::from(vec![
            Span::raw("You can find the source available at: "),
            Span::styled(
                "https://github.com/danielhe4rt/bluesky-sentinel",
                Style::default()
                    .add_modifier(Modifier::UNDERLINED)
                    .fg(Color::Blue),
            ),
        ]),
    ];
    let block = Block::bordered().title(Span::styled(
        "About the Project",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_world_map_tab(frame: &mut Frame, app: &App, area: Rect) {
    let vertical_chunk =
        Layout::vertical([Constraint::Min(5), Constraint::Percentage(100)]).split(area);

    let chunks = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(vertical_chunk[1]);
    let up_style = Style::default().fg(Color::Green);
    let failure_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT);

    let rows = app.nodes.iter().map(|s| {
        let style = if s.status == "Up" {
            up_style
        } else {
            failure_style
        };
        Row::new(vec![
            s.name.as_str(),
            s.datacenter.as_str(),
            s.address.as_str(),
            s.status.as_str(),
        ])
        .style(style)
    });

    let driver_metrics_table = driver_metrics_widget(app);

    frame.render_widget(driver_metrics_table, vertical_chunk[0]);

    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .header(
        Row::new(vec!["Node", "Location", "Address", "Status"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
    )
    .block(Block::bordered().title("Connected Nodes"));
    frame.render_widget(table, chunks[0]);

    let map = Canvas::default()
        .block(Block::bordered().title("World"))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();

            ctx.draw(&Circle {
                x: app.nodes[0].coords.1,
                y: app.nodes[0].coords.0,
                radius: 10.0,
                color: Color::Green,
            });

            ctx.draw(&Circle {
                x: app.nodes[2].coords.1,
                y: app.nodes[2].coords.0,
                radius: 10.0,
                color: Color::Green,
            });

            for (i, s1) in app.nodes.iter().enumerate() {
                for s2 in &app.nodes[i + 1..] {
                    ctx.draw(&canvas::Line {
                        x1: s1.coords.1,
                        y1: s1.coords.0,
                        y2: s2.coords.0,
                        x2: s2.coords.1,
                        color: Color::Yellow,
                    });
                }
            }
            for server in &app.nodes {
                let color = if server.status == "Up" {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.print(
                    server.coords.1,
                    server.coords.0,
                    Span::styled("X", Style::default().fg(color)),
                );
            }
        })
        .marker(if app.enhanced_graphics {
            symbols::Marker::Braille
        } else {
            symbols::Marker::Dot
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    frame.render_widget(map, chunks[1]);
}
