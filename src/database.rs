use anyhow::Result;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::sync::Arc;

pub async fn create_session() -> Result<Arc<Session>> {
    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await?;

    Ok(Arc::new(session))
}
