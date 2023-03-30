use crate::config::Config;

use super::activity::PlexActivity;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use urlencoding::encode;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub added_at: i64,
    pub r#type: String,
    pub thumb: String,
    pub title: String,
    pub parent_title: String,      // album
    pub grandparent_title: String, // artist
    pub duration: u32,
    pub view_offset: u32, // current played time
    #[serde(rename = "Player")]
    pub player: Player,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaContainer {
    pub size: i32,
    #[serde(rename = "Metadata")]
    pub metadata: Vec<Metadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    #[serde(rename = "MediaContainer")]
    pub media_container: MediaContainer,
}

impl Session {
    pub fn to_activity(&self, config: &Config) -> Option<PlexActivity> {
        let size = self.media_container.size;
        if size <= 0 {
            return None;
        }

        // find the most current player
        let mut reversed = self.media_container.metadata.clone();
        reversed.reverse();
        let metadata = &reversed[0];

        let playing = metadata.player.state.as_str() == "playing";
        let player_status = if playing { "▶️" } else { "⏸️" };

        let state = format!("{} {}", player_status, metadata.grandparent_title);
        let details = format!("🎵 {}", metadata.title);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let large_image = format!(
            "{}/photo/:/transcode?url={}&X-Plex-Token={}&width=80&height=80",
            config.origin,
            encode(metadata.thumb.as_str()),
            config.token
        );

        Some(PlexActivity {
            playing,
            state,
            details,
            create_at: now,
            duration: (metadata.duration - metadata.view_offset) as i64,
            large_image,
        })
    }
}
