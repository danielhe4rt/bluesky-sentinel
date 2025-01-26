use crate::jetstream::events::delete::DeleteEventHandler;
use crate::jetstream::events::dto::NewEventDTO;

pub struct RepostEvent {}

impl RepostEvent {
    pub fn new() -> Self {
        RepostEvent {}
    }
}

#[async_trait::async_trait]
impl DeleteEventHandler for RepostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        10
    }
}
