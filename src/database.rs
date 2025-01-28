use anyhow::Result;

use scylla::transport::session::{CurrentDeserializationApi, GenericSession};
use scylla::{CachingSession, SessionBuilder};
use std::sync::Arc;

pub async fn create_session() -> Result<GenericSession<CurrentDeserializationApi>> {
    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await?;

    session.use_keyspace("bsky_rpg", true).await?;
    // session.await_schema_agreement().await?;

    Ok(session)
}

pub async fn create_caching_session() -> Result<Arc<CachingSession>> {
    let session = create_session().await?;
    let caching_session = Arc::new(CachingSession::from(session, 100));

    Ok(caching_session)
}
