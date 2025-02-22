use crate::jetstream::leveling::LevelResponse;
use crate::models::character::Character;

use crate::models::character_experience::CharacterExperience;
use crate::models::udts::leveling::Leveling;
use charybdis::operations::{Find, Insert};
use charybdis::scylla::CachingSession;
use charybdis::types::Counter;
use std::sync::Arc;

pub struct CharacterRepository {
    pub session: Arc<CachingSession>,
}

impl CharacterRepository {
    pub fn new(connection: Arc<CachingSession>) -> Self {
        Self {
            session: connection,
        }
    }
    pub async fn find_by_partition_key(&self, user_did: String) -> Option<Character> {
        let character = Character {
            user_did: user_did.clone(),
            ..Default::default()
        };
        
        character.find_by_partition_key()
            .execute(&self.session)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
            .pop()
    }

    pub async fn find_character_experience_by_partition_key(
        &self,
        user_did: String,
    ) -> Option<CharacterExperience> {
        let character = CharacterExperience {
            user_did: user_did.clone(),
            current_experience: Counter(0),
        };
        
        character.find_by_partition_key()
            .execute(&self.session)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap()
            .pop()
            
    }

    pub async fn increment_character_experience(
        &self,
        character_experience: CharacterExperience,
        experience_points: i64,
    ) {
        character_experience
            .increment_current_experience(experience_points)
            .execute(&self.session)
            .await
            .expect("Failed to increment experience");
    }

    pub async fn update_character(&self, character: &mut Character, response: LevelResponse) {
        character.leveling_state = Leveling::from(response);
        character
            .insert()
            .execute(&self.session)
            .await
            .expect("Failed to update character");
    }
}
