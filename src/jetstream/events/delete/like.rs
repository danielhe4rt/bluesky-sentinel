use crate::jetstream::events::delete::DeleteEventHandler;
use crate::jetstream::events::dto::NewEventDTO;

pub struct LikePostEvent {}

impl LikePostEvent {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait::async_trait]
impl DeleteEventHandler for LikePostEvent {
    fn calculate_exp(&self, _: &NewEventDTO) -> i32 {
        10
    }
}
