use crate::models::udts::leveling::Leveling;
use charybdis::macros::charybdis_view_model;
use charybdis::types::{Frozen, Map, Text, Timestamp};

#[charybdis_view_model(
    base_table = events,
    table_name = events_by_type,
    partition_keys = [event_type, bucket_id],
    clustering_keys = [event_at, user_did],
    table_options = r#"
          CLUSTERING ORDER BY (event_at DESC)
    "#
)]
#[derive(Default, Clone)]
pub struct EventsByType {
    pub user_did: Text,
    pub bucket_id: Timestamp,
    pub event_commit_type: Text,
    pub event_type: Text,
    pub event_id: Text,
    pub event_data: Frozen<Map<Text, Text>>,
    pub leveling_state: Leveling,
    pub event_at: Timestamp,
}
