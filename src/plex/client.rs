use reqwest::Client;
use crate::config::config::Config;
use super::media::Session;

pub struct PlexClient<'a> {
    config: &'a Config,
    client: Client,
}

impl<'a> PlexClient<'a> {
    pub fn new(config: &'a Config) -> Self {
        let client = Client::new();
        PlexClient {
            config,
            client,
        }
    }
    
    pub async fn get_session(&self) -> serde_json::Result<Session> {
        let res = self.get(
            String::from("/status/sessions"),
            None
        ).await;
        if res.is_err() {
            panic!("cannot not get current session");
        }

        let result: Session = serde_json::from_str(res.unwrap().as_str())?;
        Ok(result)
    }

    async fn get(&self, path: String, query: Option<&Vec<(String, String)>>) -> reqwest::Result<String> {
        let mut parameters = format!("?X-Plex-Token={}", self.config.token);
        if query.is_some() {
            for (name, value) in query.unwrap().iter() {
                parameters.push_str(format!("&{name}={value}").as_str());
            }
        }

        let res = self.client
            .get(format!("{}{}{}", self.config.origin, path, parameters))
            .header("Accept", "application/json")
            .send()
            .await;
        res.unwrap().text().await
    }
}


