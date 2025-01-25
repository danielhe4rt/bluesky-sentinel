use anyhow::Result;

use std::sync::Arc;
use scylla::{Session, SessionBuilder};

pub async fn create_session() -> Result<Arc<Session>> {
    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await?;

    Ok(Arc::new(session))
}
