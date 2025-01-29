use crate::tui::app::{App, ClusterRegion};
use itertools::Itertools;
use rand::Rng;
use ratatui::prelude::{Color, Span, Style};
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::{Circle, Context, Map, MapResolution};

pub fn draw_map(ctx: &mut Context, app: &App) {
    ctx.draw(&Map {
        color: Color::White,
        resolution: MapResolution::High,
    });
    ctx.layer();

    let current_clusters = app
        .cluster_regions
        .clone()
        .into_iter()
        .sorted_by_key(|(k, v)| k.clone());



    for (_, cluster_region) in current_clusters.enumerate() {
        // Draw the Region Circles
        let current_cluster = cluster_region.1;

        // Draw the Lines
        draw_streaming_flow(ctx, app, &current_cluster);
        draw_datacenters_line(ctx, app, &current_cluster);


        // Draw the Nodes
        draw_nodes(ctx, &current_cluster);
        draw_cluster_circle(ctx, &current_cluster);

        ctx.layer();
    }
}
fn draw_streaming_flow(ctx: &mut Context, app: &App, current_cluster: &ClusterRegion) {
    // Define the number of dots in the stream
    ctx.layer();
    let mut rng = rand::thread_rng(); // Random number generator
    let num_dots = 15;
    let stream_speed = 0.02; // Adjust this for faster or slower movement

    // Time-based animation control
    let time_factor = (2.0 * stream_speed) % 1.0;

    for (_, target_region) in app.cluster_regions.iter() {
        // Skip if the stream loops back to the same cluster
        if current_cluster.coords == target_region.coords {
            continue;
        }

        // Calculate the difference in coordinates
        let dx = target_region.coords.1 - current_cluster.coords.1;
        let dy = target_region.coords.0 - current_cluster.coords.0;

        // Place multiple dots along the stream
        for i in 0..num_dots {
            // Calculate a phase offset for each dot
            let phase_offset = (i as f64) / (num_dots as f64);
            let mut dot_position = (time_factor + phase_offset) % 1.0;

            let random_boost: f64 = rng.gen_range(-0.02..0.03); // Random factor for slight forward or backward movement
            dot_position = (dot_position + random_boost).clamp(0.2, 1.0); // Ensure position stays within bounds

            // Calculate the dot's current position along the stream
            let sparkle_x = current_cluster.coords.1 + dx * dot_position;
            let sparkle_y = current_cluster.coords.0 + dy * dot_position;

            // Draw the dot at the calculated position
            ctx.print(
                sparkle_x,
                sparkle_y,
                Span::styled("•", Style::default().fg(Color::Cyan)), // Customize the color
            );
        }
    }
}

fn draw_nodes(ctx: &mut Context, current_cluster: &ClusterRegion) {
    let gap_coordinates = vec![
        (5.0, 0.0),   // Diagonal top-right
        (-7.0, 7.0),  // Diagonal bottom-right
        (-7.0, -7.0), // Diagonal bottom-left
    ];

    for (node_index, node) in current_cluster.nodes.iter().enumerate() {
        let offset_index = node_index % gap_coordinates.len();
        let (lat_offset, lon_offset) = gap_coordinates[offset_index];

        // Calculate the node's new position with the gap
        let node_lat = current_cluster.coords.0 + lat_offset * 1.1; // Adjust scale as needed
        let node_lon = current_cluster.coords.1 + lon_offset * 1.1;

        let node_style = if node.is_running {
            Style::default().fg(Color::LightCyan)
        } else {
            Style::default().fg(Color::Red)
        };

        ctx.print(node_lon, node_lat, Span::styled(node_index.to_string(), node_style));
    }
}

fn draw_datacenters_line(ctx: &mut Context, app: &App, current_cluster: &ClusterRegion) {
    for s2 in app.cluster_regions.iter().enumerate() {
        let s2 = s2.1 .1;
        ctx.draw(&canvas::Line {
            x1: current_cluster.coords.1,
            y1: current_cluster.coords.0,
            y2: s2.coords.0,
            x2: s2.coords.1,
            color: Color::Yellow,
        });

        // draw floating points which will follow the line track
    }
}

fn draw_cluster_circle(ctx: &mut Context, current_cluster: &ClusterRegion) {

    let cluster_style = if current_cluster.is_fully_operating() {
        Color::Green
    } else if current_cluster.is_quorum() {
        Color::Blue
    } else if current_cluster.is_operating_with_minimum() {
        Color::Yellow
    } else {
        Color::Red
    };


    ctx.draw(&Circle {
        x: current_cluster.coords.1,
        y: current_cluster.coords.0,
        radius: 12.0,
        color: cluster_style,
    });
    
    ctx.print(
        current_cluster.coords.1 - 2.0,
        current_cluster.coords.0 - 3.0,
        Span::styled(current_cluster.name.clone(), cluster_style),
    );
    
}
