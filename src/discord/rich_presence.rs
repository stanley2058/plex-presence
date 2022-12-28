use std::time::{UNIX_EPOCH, SystemTime};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{Activity, Assets, Timestamps};
use urlencoding::encode;
use crate::config::config::Config;
use crate::plex::media::Session;

pub struct DiscordClient<'a> {
    config: &'a Config,
    ipc_client: DiscordIpcClient,
    current: Option<DiscordPlexActivity>,
}

#[derive(Clone)]
pub struct DiscordPlexActivity {
    state: String,
    details: String,
    timestamps: Timestamps,
    large_image: String,
}

impl<'a> DiscordClient<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut ipc_client = DiscordIpcClient::new(config.discord_application_id.as_str()).unwrap();
        let connection_result = ipc_client.connect();
        if connection_result.is_err() {
            panic!("cannot connect to discord ipc socket");
        }

        DiscordClient {
            config,
            ipc_client,
            current: None,
        }
    }

    pub fn clear_activity(&mut self) {
        let _ = self.ipc_client.clear_activity();
    }

    pub fn update_activity(&mut self, activity: Activity) {
        let set_result = self.ipc_client.set_activity(activity);
        if set_result.is_err() {
            println!("cannot set activity");
        }
    }

    pub fn update_plex_activity(&mut self, activity: Option<DiscordPlexActivity>) {
        let new = activity.clone();
        if activity.is_none() && self.current.is_some() {
            let clear_res = self.ipc_client.clear_activity();
            if clear_res.is_err() {
                println!("failed to clear activity");
            }
            self.current = new;
            return;
        }
        if self.current.is_some() {
            let current_details = self.current.as_ref().unwrap().details.clone();
            let new_details = new.as_ref().unwrap().details.clone();
            let current_state = self.current.as_ref().unwrap().state.clone();
            let new_state = new.as_ref().unwrap().state.clone();
            if new_details == current_details && current_state == new_state {
                return;
            }
        }
        self.current = new;

        let plex_act = activity.unwrap();
        let act = Activity::new()
            .state(plex_act.state.as_str())
            .details(plex_act.details.as_str())
            .timestamps(plex_act.timestamps.clone())
            .assets(
                Assets::new()
                    .large_image(plex_act.large_image.as_str())
            );
        self.update_activity(act)
    }

    pub fn plex_session_to_activity(&self, session: &Session) -> Option<DiscordPlexActivity> {
        let size = session.media_container.size;
        if size <= 0 { return None; }

        // find the most current player
        let mut sorted = session.media_container.metadata.clone();
        sorted.sort_by(|a, b| (-a.added_at).cmp(&(-b.added_at)));
        let metadata = &sorted[0];

        let player_status = match metadata.player.state.as_str() {
            "playing" => "▶️",
            _ => "⏸️",
        };

        let state = format!("{} {}", player_status, metadata.grandparent_title);
        let details = format!("♫ {}", metadata.title);

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let time_left = (((metadata.duration as u128) - (metadata.view_offset as u128) + now) / 1000) as i64;
        let timestamps = Timestamps::new().end(time_left);

        let large_image = format!("{}/photo/:/transcode?url={}&X-Plex-Token={}&width=80&height=80",
                self.config.origin,
                encode(metadata.thumb.as_str()),
                self.config.token
            );

        Some(DiscordPlexActivity { state, details, timestamps, large_image })
    }
}
