pub mod create;
pub mod delete;
pub mod dto;

use crate::jetstream::events::create::create_event_handler;
use crate::repositories::DatabaseRepository;
use jetstream_oxide::events::commit::{CommitData, CommitEvent, CommitInfo};
use jetstream_oxide::events::EventInfo;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::jetstream::events::delete::delete_event_handler;

enum AppBskyEventRecord {
    Post,
    Like,
    Repost,
}

impl Display for AppBskyEventRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppBskyEventRecord::Post => write!(f, "app.bsky.feed.post"),
            AppBskyEventRecord::Like => write!(f, "app.bsky.feed.like"),
            AppBskyEventRecord::Repost => write!(f, "app.bsky.feed.repost"),
        }
    }

}


pub struct DeleteEventPayload {
    event_info: EventInfo,
    commit_info: CommitInfo,
}

impl DeleteEventPayload {
    fn new(event_info: EventInfo, commit_info: CommitInfo) -> Self {
        Self {
            event_info,
            commit_info,
        }
    }
}

pub struct CreateEventPayload {
    event_info: EventInfo,
    commit_data: CommitData,
}

impl CreateEventPayload {
    fn new(event_info: EventInfo, commit_data: CommitData) -> Self {
        CreateEventPayload {
            event_info,
            commit_data,
        }
    }
}

pub async fn events_handler(
    repository: &Arc<DatabaseRepository>,
    commit: CommitEvent,
    semaphore: Arc<Semaphore>,
) {
    match commit {
        CommitEvent::Create {
            info: user_info,
            commit,
        } => {
            let payload = CreateEventPayload::new(user_info, commit);

            create_event_handler("create".to_string(), repository, payload, semaphore).await;
        }
        CommitEvent::Delete {
            info: user_info,
            commit,
        } => {
            let payload = DeleteEventPayload::new(user_info, commit);
            delete_event_handler(repository, payload, semaphore).await;
        }
        CommitEvent::Update {
            info: user_info,
            commit,
        } => {
            let payload = CreateEventPayload::new(user_info, commit);
            
            create_event_handler("update".to_string(), repository, payload, semaphore).await;
        }
    }
}
