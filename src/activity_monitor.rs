use std::thread;
use std::time::Duration;

use crate::{config::Config, discord::client::DiscordClient, plex::client::PlexClient};

pub struct ActivityMonitor<'a> {
    discord_client: DiscordClient,
    plex_client: PlexClient<'a>,
    config: &'a Config,
}

impl<'a> ActivityMonitor<'a> {
    pub fn new(
        discord_client: DiscordClient,
        plex_client: PlexClient<'a>,
        config: &'a Config,
    ) -> Self {
        ActivityMonitor {
            discord_client,
            plex_client,
            config,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let is_playing = self.fetch_and_update().await;
            if is_playing {
                thread::sleep(Duration::from_secs(1));
            } else {
                thread::sleep(Duration::from_secs(60));
            }
        }
    }

    async fn fetch_and_update(&mut self) -> bool {
        let session_res = self.plex_client.get_session().await;
        let has_session = session_res.is_ok();
        if has_session {
            let session = session_res.unwrap();
            self.discord_client
                .update_activity(session.to_activity(self.config));
        } else {
            // no session alive, clear activity
            self.discord_client.clear_activity();
        }
        has_session
    }
}
