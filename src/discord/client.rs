use crate::plex::activity::PlexActivity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

pub struct DiscordClient {
    ipc_client: DiscordIpcClient,
    current: Option<PlexActivity>,
}

impl DiscordClient {
    pub fn new(discord_application_id: &String) -> Self {
        let mut ipc_client = DiscordIpcClient::new(discord_application_id.as_str()).unwrap();
        let _ = ipc_client.connect().unwrap();

        DiscordClient {
            ipc_client,
            current: None,
        }
    }

    pub fn clear_activity(&mut self) {
        let _ = self.ipc_client.clear_activity();
    }

    pub fn update_activity(&mut self, activity: Option<PlexActivity>) {
        if activity.is_none() && self.current.is_some() {
            let clear_res = self.ipc_client.clear_activity();
            if clear_res.is_err() {
                println!("failed to clear activity");
            }
            self.current = activity;
            return;
        }
        if self.current.is_some() && activity.as_ref().unwrap().playing {
            let current_details = self.current.as_ref().unwrap().details.clone();
            let new_details = activity.as_ref().unwrap().details.clone();
            let current_state = self.current.as_ref().unwrap().state.clone();
            let new_state = activity.as_ref().unwrap().state.clone();
            if new_details == current_details && current_state == new_state {
                return;
            }
        }

        let act = activity.as_ref().unwrap().into_activity();
        let set_result = self.ipc_client.set_activity(act);
        if set_result.is_err() {
            println!("cannot set activity");
        }
        self.current = activity;
    }
}
