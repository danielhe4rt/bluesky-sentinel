use anyhow::Result;

use scylla::transport::session::{CurrentDeserializationApi, GenericSession};
use scylla::{CachingSession, SessionBuilder};
use std::sync::Arc;
use scylla::transport::ExecutionProfile;
use scylla::load_balancing::DefaultPolicy;
use crate::args::AppSettings;

pub async fn create_session(app_settings: &AppSettings) -> Result<GenericSession<CurrentDeserializationApi>> {

    let policy = DefaultPolicy::builder()
        .prefer_datacenter(app_settings.prefer_dc.to_string())
        .build();

    let profile = ExecutionProfile::builder()
        .load_balancing_policy(policy)
        .build();

    let session = SessionBuilder::new()
        .known_nodes(&app_settings.known_nodes)
        .default_execution_profile_handle(profile.into_handle())
        .build()
        .await?;



    session.use_keyspace(&app_settings.current_keyspace, true).await?;
    // session.await_schema_agreement().await?;

    Ok(session)
}

pub async fn create_caching_session(app_settings: &AppSettings) -> Result<Arc<CachingSession>> {
    let session = create_session(app_settings).await?;
    let caching_session = Arc::new(CachingSession::from(session, 100));

    Ok(caching_session)
}
