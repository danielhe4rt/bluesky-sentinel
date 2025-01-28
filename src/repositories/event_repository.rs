use crate::jetstream::events::dto::NewEventDTO;
use crate::jetstream::leveling::LevelResponse;
use crate::models::events::Events;
use crate::models::udts::leveling::Leveling;
use charybdis::operations::Insert;
use charybdis::types::{Counter, Timestamp};

use scylla::CachingSession;
use std::sync::Arc;
use chrono::{Timelike, Utc};
use crate::models::events_metrics::EventsMetrics;

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

    pub async fn increment_event_count(&self, event_type: String, event_commit_type: String) {
        let model = EventsMetrics {
            event_type,
            created_count: None,
            updated_count: None,
            deleted_count: None,
        };

        match event_commit_type.as_str() {
            "create" => {
                model.increment_created_count(1)
                    .execute(&self.session)
                    .await
                    .expect("Failed to increment created count");
            }
            "update" => {
                model.increment_updated_count(1)
                    .execute(&self.session)
                    .await
                    .expect("Failed to increment updated count");
            }
            "delete" => {
                model.increment_deleted_count(1)
                    .execute(&self.session)
                    .await
                    .expect("Failed to increment deleted count");
            }
            _ => {}
        }

    }
}
