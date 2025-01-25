use crate::jetstream::events::create::CreateEventHandler;
use crate::jetstream::events::dto::NewEventDTO;

pub struct LikePostEvent {}

impl LikePostEvent {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl CreateEventHandler for LikePostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        10
    }
}
