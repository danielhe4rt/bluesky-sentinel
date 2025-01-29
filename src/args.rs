use paris::Logger;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub app_name: String,
    pub bsky_topics: Vec<String>,
    pub bsky_dids: Option<Vec<String>>,
    pub max_workers: usize,
    pub prefer_dc: String,
    pub known_nodes: Vec<String>,
    pub current_keyspace: String,
}

impl AppSettings {
    pub fn new() -> Self {
        Logger::new();
        env_logger::init();

        dotenvy::from_filename(".env").expect("Failed to load .env file");

        let name = dotenvy::var("APP_NAME").unwrap_or("BlueSky Sentinel".to_string());

        let bsky_topics = dotenvy::var("BSKY_TOPICS")
            .unwrap_or("app.bsky.feed.post".to_string())
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let bsky_dids = dotenvy::var("BSKY_DIDS").unwrap();

        let max_workers = dotenvy::var("MAX_WORKERS")
            .unwrap_or("5".to_string())
            .parse::<usize>()
            .expect("Failed to parse WORKERS");

        let prefer_datacenter = dotenvy::var("DB_PREFER_DC")
            .unwrap_or("datacenter1".to_string())
            .parse::<String>()
            .expect("Failed to parse WORKERS");

        let known_nodes = dotenvy::var("DB_KNOWN_NODES")
            .unwrap_or("localhost:9042".to_string())
            .parse::<String>()
            .expect("Failed to parse Known Nodes")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        
        let current_keyspace = dotenvy::var("DB_KEYSPACE")
            .unwrap_or("bsky_rpg".to_string())
            .parse::<String>()
            .expect("Failed to parse WORKERS");

        let bsky_dids: Option<Vec<String>> = if !bsky_dids.is_empty() {
            let dids = bsky_dids.split(',').map(|s| s.to_string()).collect();

            Some(dids)
        } else {
            None
        };

        Self {
            app_name: name,
            prefer_dc: prefer_datacenter,
            bsky_topics,
            bsky_dids,
            known_nodes,
            max_workers,
            current_keyspace
        }
    }
}
