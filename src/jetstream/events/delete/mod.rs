mod create;
mod like;
mod repost;

use crate::jetstream::events::delete::create::CreatePostEvent;
use crate::jetstream::events::delete::like::LikePostEvent;
use crate::jetstream::events::delete::repost::RepostEvent;
use crate::jetstream::events::dto::NewEventDTO;
use crate::jetstream::events::DeleteEventPayload;
use crate::jetstream::leveling::{calculate_experience, LevelResponse};
use crate::models::character::Character;
use crate::models::character_experience::CharacterExperience;
use crate::repositories::DatabaseRepository;
use charybdis::types::Counter;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::models::events_metrics::EventsMetrics;

#[async_trait::async_trait]
trait DeleteEventHandler {
    async fn handle(
        &mut self,
        repository: &Arc<DatabaseRepository>,
        payload: &NewEventDTO,
    ) -> LevelResponse {
        // find all the data we need
        let character = repository
            .character
            .find_by_partition_key(payload.user_did.clone())
            .await;

        let mut character = match character {
            Some(character) => character,
            None => {
                let response = repository
                    .bsky
                    .get_author_profile(payload.user_did.clone())
                    .await;
                // info!("Creating new character for user {}", payload.user_did);
                match response {
                    Ok(response) => Character::from(response),
                    Err(_) => Character{
                        user_did: payload.user_did.clone(),
                        ..Default::default()
                    },
                }
            }
        };

        let character_experience = repository
            .character
            .find_character_experience_by_partition_key(payload.user_did.clone())
            .await;

        let character_experience = match character_experience {
            Some(character_experience) => character_experience,
            None => {
                let character_experience = CharacterExperience {
                    user_did: payload.user_did.clone(),
                    current_experience: Counter(0),
                };

                repository
                    .character
                    .increment_character_experience(
                        character_experience,
                        character.leveling_state.experience as i64,
                    )
                    .await;

                CharacterExperience {
                    user_did: payload.user_did.clone(),
                    current_experience: Counter(character.leveling_state.experience as i64),
                }
            }
        };

        // calculate the experience
        let current_experience = character_experience.get_experience();
        let action_gained_experience = self.calculate_exp(payload);
        let new_experience = current_experience.saturating_add(action_gained_experience);
        let leveling_response_dto = calculate_experience(current_experience, new_experience);

        repository
            .character
            .update_character(&mut character, leveling_response_dto.clone())
            .await;

        repository
            .event
            .insert_event(payload, leveling_response_dto.clone())
            .await;

        // persist the changes
        repository
            .character
            .increment_character_experience(character_experience, action_gained_experience as i64)
            .await;

        repository
            .event
            .increment_event_count(payload.event_type.clone(), payload.commit_type.clone()).await;

        leveling_response_dto
    }

    fn calculate_exp(&self, payload: &NewEventDTO) -> i32;
}

pub async fn delete_event_handler(
    repository: &Arc<DatabaseRepository>,
    payload: DeleteEventPayload,
    semaphore: Arc<Semaphore>,
) {
    // TODO: review dto usage
    let mut event_payload = NewEventDTO::from(&payload);
    event_payload.commit_type = "delete".to_string();

    let repo = Arc::clone(repository);
    let permit = semaphore.acquire_owned().await.unwrap(); // Acquire a semaphore permit

    tokio::spawn(async move {
        let _ = select_event_handler(&payload.commit_info.collection.as_str())
            .handle(&repo, &event_payload)
            .await;
        // info!("[Created][{}] User {} gained {} experience",event_payload.event_type, event_payload.user_did, response.experience);
        drop(permit); // Release the semaphore permit
    });
}

fn select_event_handler(record: &str) -> Box<dyn DeleteEventHandler + Send + Sync> {
    match record {
        "app.bsky.feed.post" => Box::new(CreatePostEvent::new()),
        "app.bsky.feed.like" => Box::new(LikePostEvent::new()),
        "app.bsky.feed.repost" => Box::new(RepostEvent::new()),
        _ => panic!("Unknown event type"),
    }
}
