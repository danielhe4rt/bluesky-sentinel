use scylla::observability::metrics::Metrics;
use std::sync::Arc;
use scylla::cluster::Node as ScyllaNode;
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
    pub fn from_db(metrics: Arc<Metrics>) -> Self {
        Self {
            queries_count: metrics.get_queries_num(),
            errors_count: metrics.get_errors_num(),
            iter_queries_count: metrics.get_queries_iter_num(),
            iter_errors_count: metrics.get_errors_iter_num(),
            avg_latency: metrics.get_latency_avg_ms().unwrap_or_default(),
            p99_latency: metrics.get_latency_percentile_ms(99.9).unwrap_or_default(),
            p50_latency: metrics.get_latency_percentile_ms(50.0).unwrap_or_default(),
        }
    }
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
pub struct Node {
    pub name: String,
    pub datacenter: String,
    pub coords: (f64, f64),
    pub address: String,
    pub status: String,
}

impl Node {
    pub fn transform_nodes(current_nodes: &[Arc<ScyllaNode>]) -> Vec<Node> {
        current_nodes
            .iter()
            .map(move |node| {
                let datacenter = node.datacenter.clone().unwrap();
                let name = node.host_id.clone().to_string();
                let address = node.address.to_string();
                let coords = match datacenter.as_str() {
                    "SA-DC" => (-23.5505, -46.6333),
                    "EU-DC" => (52.5200, 13.4050),
                    _ => (0.0, 0.0),
                };
                let status = match !node.is_down() {
                    true => "Up",
                    false => "Down",
                };

                Node {
                    name,
                    datacenter,
                    coords,
                    address,
                    status: status.to_string(),
                }
            })
            .collect()
    }
}


pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub tabs: TabsState,
    pub coords: Vec<(f64, f64)>,
    pub listened_events: Vec<String>,
    pub recent_events: Vec<(String, String)>,
    pub nodes: Vec<Node>,
    pub enhanced_graphics: bool,
    pub metrics: DriverMetrics,
    pub task_selected: usize,
}

impl App {
    pub fn new(title: String, enhanced_graphics: bool) -> Self {

        let metrics = DriverMetrics::default();

        App {
            metrics,
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Events".to_string(), "Connected Nodes".to_string()]),
            listened_events: vec![
                "app.bsky.users.follow".to_string(),
                "app.bsky.users.repost".to_string(),
            ],
            task_selected: 0,
            recent_events: vec![
                ("Follow".to_string(), "INFO".to_string()),
                ("Repost".to_string(), "INFO".to_string()),
                ("Damn".to_string(), "CRITICAL".to_string()),
            ],
            nodes: vec![Node::default(); 10],
            coords: vec![],
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.task_selected = (self.task_selected + 1) % self.listened_events.len();
    }

    pub fn on_down(&mut self) {
        self.task_selected = if self.task_selected > 0 {
            self.task_selected - 1
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
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        let log = self.recent_events.pop().unwrap();
        self.recent_events.insert(0, log);
    }
}
