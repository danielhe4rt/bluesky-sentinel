use crate::jetstream::events::dto::NewEventDTO;
use crate::jetstream::leveling::LevelResponse;
use crate::models::events::Events;
use crate::models::udts::leveling::Leveling;
use charybdis::operations::Insert;
use charybdis::types::Timestamp;

use scylla::CachingSession;
use std::sync::Arc;
use chrono::{Timelike, Utc};

pub struct EventRepository {
    pub session: Arc<CachingSession>,
}

impl EventRepository {
    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            session: Arc::clone(&connection),
        }
    }

    pub async fn insert_event(&self, payload: &NewEventDTO, level_response: LevelResponse) {
        let current_timestamp = Utc::now()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        let event = Events {
            event_commit_type: payload.commit_type.to_string(),
            bucket_id: current_timestamp,
            user_did: payload.user_did.to_string(),
            event_type: payload.event_type.to_string(),
            event_id: payload.event_id.to_string(),
            event_data: payload.context.clone(),
            leveling_state: Leveling::from(level_response),
            event_at: Utc::now(),
        };

        event
            .insert()
            .execute(&self.session)
            .await
            .expect("Failed to insert event");
    }
}
