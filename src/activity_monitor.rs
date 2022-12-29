pub mod activity_monitor {
    use std::thread;
    use std::time::Duration;

    use crate::{config::config::Config, discord::client::DiscordClient, plex::client::PlexClient};

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
                self.fetch_and_update().await;
                thread::sleep(Duration::from_millis(1000));
            }
        }

        async fn fetch_and_update(&mut self) {
            let session_res = self.plex_client.get_session().await;
            if session_res.is_ok() {
                let session = session_res.unwrap();
                self.discord_client
                    .update_activity(session.into_activity(self.config));
            } else {
                // no session alive, clear activity
                self.discord_client.clear_activity();
            }
        }
    }
}
