use anyhow::Result;
use once_cell::sync::OnceCell;
use openapi::tower::client::{ApiClient, Configuration, Url};
use std::time::Duration;

static REST_SERVER: OnceCell<ApiClient> = OnceCell::new();

/// REST client
pub struct RestClient {}

impl RestClient {
    /// Initialise the URL of the REST server.
    pub fn init(mut url: Url) -> Result<()> {
        // TODO: Support HTTPS Certificates
        if url.port().is_none() {
            url.set_port(Some(30011))
                .map_err(|_| anyhow::anyhow!("Failed to set REST client port"))?;
        }
        let cfg =
            Configuration::new(url, Duration::from_secs(5), None, None, true).map_err(|error| {
                anyhow::anyhow!(
                    "Failed to create openapi configuration, Error: '{:?}'",
                    error
                )
            })?;
        REST_SERVER.get_or_init(|| ApiClient::new(cfg));
        Ok(())
    }

    /// Get an ApiClient to use for REST calls.
    pub(crate) fn client() -> &'static ApiClient {
        REST_SERVER.get().unwrap()
    }
}
