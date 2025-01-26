use crate::jetstream::events::delete::DeleteEventHandler;
use crate::jetstream::events::dto::NewEventDTO;

pub struct CreatePostEvent {}

impl CreatePostEvent {
    pub fn new() -> Self {
        CreatePostEvent {}
    }
}

#[async_trait::async_trait]
impl DeleteEventHandler for CreatePostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        30
    }
}
