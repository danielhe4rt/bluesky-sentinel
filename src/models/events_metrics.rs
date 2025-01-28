use charybdis::macros::charybdis_model;
use charybdis::types::{Counter, Text};

#[charybdis_model(
    table_name = events_metrics,
    partition_keys = [event_type],
    clustering_keys = [],
)]
pub struct EventsMetrics {
    pub event_type: Text,
    pub created_count: Option<Counter>,
    pub updated_count: Option<Counter>,
    pub deleted_count: Option<Counter>,
}

impl Default for EventsMetrics {
    fn default() -> Self {
        Self {
            event_type: Default::default(),
            created_count: Counter(0).into(),
            updated_count: Counter(0).into(),
            deleted_count: Counter(0).into(),
        }
    }
}