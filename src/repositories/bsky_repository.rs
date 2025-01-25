use atrium_api::app::bsky::actor::defs::ProfileViewDetailed;
use atrium_api::client::AtpServiceClient;
use atrium_xrpc_client::reqwest::ReqwestClient;
use std::str::FromStr;

pub type BskyClient = AtpServiceClient<ReqwestClient>;

pub struct BskyRepository {
    client: BskyClient,
}

impl BskyRepository {
    pub fn new(uri: String) -> Self {
        let client = AtpServiceClient::new(ReqwestClient::new(uri));
        Self { client }
    }

    pub async fn get_author_profile(&self, author: String) -> anyhow::Result<ProfileViewDetailed> {
        let response = self
            .client
            .service
            .app
            .bsky
            .actor
            .get_profile(
                atrium_api::app::bsky::actor::get_profile::ParametersData {
                    actor: atrium_api::types::string::AtIdentifier::from_str(&author).unwrap(),
                }
                .into(),
            )
            .await;

        match response {
            Ok(response) => Ok(response),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
