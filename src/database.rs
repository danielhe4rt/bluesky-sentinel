use anyhow::Result;

use scylla::transport::session::{CurrentDeserializationApi, GenericSession};
use scylla::{CachingSession, SessionBuilder};
use std::sync::Arc;
use scylla::transport::ExecutionProfile;
use scylla::load_balancing::DefaultPolicy;


pub async fn create_session() -> Result<GenericSession<CurrentDeserializationApi>> {

    let policy = DefaultPolicy::builder()
        .prefer_datacenter("SA-DC".to_string())
        .build();

    let profile = ExecutionProfile::builder()
        .load_balancing_policy(policy)
        .build();
    
    let session = SessionBuilder::new()
        .known_node("localhost:9042")
        .default_execution_profile_handle(profile.into_handle())
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
