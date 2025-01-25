use crate::jetstream::events::create::CreateEventHandler;
use crate::jetstream::events::dto::NewEventDTO;

pub struct RepostEvent {}

impl RepostEvent {
    pub fn new() -> Self {
        RepostEvent {}
    }
}

#[async_trait::async_trait]
impl CreateEventHandler for RepostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        10
    }
}
