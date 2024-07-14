use crate::Client;
use crate::ClientResult;

pub struct About {
    pub client: Client,
}

impl About {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        About { client }
    }

    /**
     * This function performs a `GET` to the `/about` endpoint.
     *
     * Gets information about the user, the user's Drive, and system capabilities.
     */
    pub async fn get(&self, fields: &str) -> ClientResult<crate::Response<crate::types::About>> {
        let mut query_args: Vec<(String, String)> = Default::default();

        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }

        let query = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/about?{}", query), None);

        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
