use super::media::Session;
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
    ///
    /// # Panic
    /// Panics if request error, indicating Plex server not reachable.
    pub async fn get_session(&self) -> serde_json::Result<Session> {
        let res = self
            .get(String::from("/status/sessions"), None)
            .await
            .unwrap();
        let result: Session = serde_json::from_str(res.as_str())?;
        Ok(result)
    }

    /// Sends a GET request to configured Plex server
    ///
    /// # Arguments
    ///
    /// * `path` - A string gets append to the Plex origin url.
    /// * `query` - A vector of tuples containing additional query parameters.
    async fn get(
        &self,
        path: String,
        query: Option<&Vec<(String, String)>>,
    ) -> reqwest::Result<String> {
        let mut parameters = format!("?X-Plex-Token={}", self.plex_token);
        if query.is_some() {
            for (name, value) in query.unwrap().iter() {
                parameters.push_str(format!("&{name}={value}").as_str());
            }
        }

        let res = self
            .client
            .get(format!("{}{}{}", self.plex_origin, path, parameters))
            .header("Accept", "application/json")
            .send()
            .await;
        res.unwrap().text().await
    }
}
