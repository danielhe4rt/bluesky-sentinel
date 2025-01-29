use crate::args::AppSettings;
use crate::models::events_metrics::EventsMetrics;
use crate::models::materialized_views::events_by_type::EventsByType;
use scylla::transport::Node;
use scylla::Metrics;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub const fn new(titles: Vec<String>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

#[derive(Debug, Default)]
pub struct DriverMetrics {
    pub queries_count: u64,
    pub iter_queries_count: u64,
    pub errors_count: u64,
    pub iter_errors_count: u64,
    pub avg_latency: u64,
    pub p99_latency: u64,
    pub p50_latency: u64,
}

impl DriverMetrics {
    pub fn update(&mut self, metrics: Arc<Metrics>) {
        self.queries_count = metrics.get_queries_num();
        self.errors_count = metrics.get_errors_num();
        self.iter_queries_count = metrics.get_queries_iter_num();
        self.iter_errors_count = metrics.get_errors_iter_num();
        self.avg_latency = metrics.get_latency_avg_ms().unwrap_or_default();
        self.p99_latency = metrics.get_latency_percentile_ms(99.9).unwrap_or_default();
        self.p50_latency = metrics.get_latency_percentile_ms(50.0).unwrap_or_default();
    }
}

#[derive(Debug, Default, Clone)]
pub struct ClusterRegion {
    pub name: String,                 // SA-DC, EU-DC, US-DC
    pub coords: (f64, f64),           // Base Coords
    pub nodes: Vec<DeserializedNode>, // Node should have a gap on them while drawing and needs to be symmetric.
    pub region_status: bool,
    pub nodes_count: usize,
    pub nodes_down: usize,
}

impl ClusterRegion {
    /// Checks if the region is fully operating (no nodes down).
    pub fn is_fully_operating(&self) -> bool {
        self.nodes_down == 0
    }

    /// Checks if the region is operating with quorum (more than half of nodes are up).
    pub fn is_quorum(&self) -> bool {
        let quorum = (self.nodes_count / 2) + 1;
        self.nodes_down < (self.nodes_count - quorum)
    }

    /// Checks if the region is operating with a minimum number of nodes (at least one node up).
    pub fn is_operating_with_minimum(&self) -> bool {
        self.nodes_down < self.nodes_count
    }

    /// Checks if the region is completely down (all nodes are down).
    pub fn is_down(&self) -> bool {
        self.nodes_down == self.nodes_count
    }
}


#[derive(Debug, Default, Clone)]
pub struct DeserializedNode {
    pub name: String,
    pub datacenter: String,
    pub coords: (f64, f64),
    pub address: String,
    pub is_running: bool,
}

impl DeserializedNode {
    pub fn transform_nodes(current_nodes: &[Arc<Node>]) -> HashMap<String, ClusterRegion> {
        let mut cluster_regions: HashMap<String, ClusterRegion> = HashMap::new();

        let nodes: Vec<DeserializedNode> = current_nodes
            .iter()
            .map(move |node| {
                let datacenter = node.datacenter.clone().unwrap();
                let name = node.host_id.clone().to_string();
                let address = node.address.to_string();
                let is_running = node.sharder().is_some();
                let coords = match datacenter.as_str() {
                    "SA-DC" => (-23.5505, -46.6333),
                    "EU-DC" => (52.5200, 13.4050),
                    "US-DC" => (37.7749, -122.4194),
                    "datacenter1" => (-23.5505, -46.6333),
                    _ => (0.0, 0.0),
                };

                DeserializedNode {
                    name,
                    datacenter,
                    coords,
                    address,
                    is_running,
                }
            })
            .collect();

        for node in nodes {
            let region = cluster_regions.get_mut(&node.datacenter);
            match region {
                Some(region) => {
                    region.nodes.push(node.clone());
                    region.nodes_count += 1;
                    if !node.is_running {
                        region.nodes_down += 1;
                    }
                }
                None => {
                    let coords = match node.datacenter.as_str() {
                        "SA-DC" => (-23.5505, -46.6333),
                        "EU-DC" => (52.5200, 13.4050),
                        "US-DC" => (37.7749, -122.4194),
                        "datacenter1" => (-23.5505, -46.6333),
                        _ => (0.0, 0.0),
                    };

                    let nodes_down = if node.is_running { 0 } else { 1 };

                    cluster_regions.insert(
                        node.datacenter.clone(),
                        ClusterRegion {
                            name: node.datacenter.clone(),
                            coords,
                            nodes: vec![node],
                            region_status: true,
                            nodes_count: 1,
                            nodes_down,
                        },
                    );
                }
            }
        }

        cluster_regions
    }
}

pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub tabs: TabsState,
    pub listened_events: Vec<EventsMetrics>,
    pub recent_events: Vec<EventsByType>,
    pub cluster_regions: HashMap<String, ClusterRegion>,
    pub enhanced_graphics: bool,
    pub metrics: DriverMetrics,
    pub selected_event: usize,
    pub fps: f64,
}

impl App {
    pub fn new(app_settings: AppSettings) -> Arc<Mutex<Self>> {
        let metrics = DriverMetrics::default();

        let app = App {
            metrics,
            title: app_settings.app_name,
            should_quit: false,
            tabs: TabsState::new(vec!["Events".to_string(), "Connected Nodes".to_string()]),
            listened_events: vec![EventsMetrics {
                event_type: "Event".to_string(),
                created_count: None,
                updated_count: None,
                deleted_count: None,
            }],
            selected_event: 0,
            recent_events: vec![EventsByType::default(); 5],
            cluster_regions: HashMap::new(),
            enhanced_graphics: true,
            fps: 60.0,
        };

        Arc::new(Mutex::new(app))
    }

    pub fn on_up(&mut self) {
        self.selected_event = (self.selected_event + 1) % self.listened_events.len();
    }

    pub fn on_down(&mut self) {
        self.selected_event = if self.selected_event > 0 {
            self.selected_event - 1
        } else {
            self.listened_events.len() - 1
        };
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        //let log = self.recent_events.pop().unwrap();
        //self.recent_events.insert(0, log);
    }
}
