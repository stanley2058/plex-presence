use super::media::Session;
use anyhow::Result;
use reqwest::Client;

pub struct PlexClient<'a> {
    client: Client,
    plex_token: &'a String,
    plex_origin: &'a String,
}

impl<'a> PlexClient<'a> {
    pub fn new(plex_token: &'a String, plex_origin: &'a String) -> Self {
        let client = Client::new();
        PlexClient {
            client,
            plex_token,
            plex_origin,
        }
    }

    /// Gets active sessions from Plex
    pub async fn get_session(&self) -> Result<Session> {
        let res = self.get("/status/sessions", None).await?;
        let result: Session = serde_json::from_str(res.as_str())?;
        Ok(result)
    }

    /// Sends a GET request to configured Plex server
    ///
    /// # Arguments
    ///
    /// * `path` - A string gets append to the Plex origin url.
    /// * `query` - A vector of tuples containing additional query parameters.
    async fn get(&self, path: &str, query: Option<&Vec<(String, String)>>) -> Result<String> {
        let mut parameters = format!("?X-Plex-Token={}", self.plex_token);
        if let Some(query) = query {
            for (name, value) in query.iter() {
                parameters.push_str(format!("&{name}={value}").as_str());
            }
        }

        let res = self
            .client
            .get(format!("{}{}{}", self.plex_origin, path, parameters))
            .header("Accept", "application/json")
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }
}
