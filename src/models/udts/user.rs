use crate::jetstream::leveling::LevelResponse;
use charybdis::macros::charybdis_udt_model;
use charybdis::types::{Float, Int};
use serde::Serialize;

#[derive(Default, Serialize, Clone)]
#[charybdis_udt_model(type_name = bsky_user)]
pub struct User {
    pub level: Int,
    pub experience: Int,
    pub experience_to_next_level: Int,
    pub levels_gained: Int,
    pub progress_percentage: Float,
}
