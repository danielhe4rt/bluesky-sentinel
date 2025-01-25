use anyhow::Result;

use std::sync::Arc;
use scylla::{CachingSession, Session, SessionBuilder};
use scylla::transport::session::{CurrentDeserializationApi, GenericSession};

pub async fn create_session() -> Result<GenericSession<CurrentDeserializationApi>> {
    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .build()
        .await?;

    session.use_keyspace("bsky_rpg", true).await?;
    
    Ok(session)
}

pub async fn create_caching_session() -> Result<Arc<CachingSession>> {
    let session = create_session().await?;
    let caching_session = Arc::new(CachingSession::from(session, 100));

    Ok(caching_session)
}