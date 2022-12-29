use discord_rich_presence::activity::{Activity, Assets, Timestamps};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct PlexActivity {
    pub playing: bool,
    pub state: String,
    pub details: String,
    pub create_at: i64,
    pub duration: i64,
    pub large_image: String,
}

impl PlexActivity {
    pub fn into_activity(&self) -> Activity {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let timestamps = Timestamps::new().end(if self.playing {
            (self.duration + self.create_at) / 1000
        } else {
            (self.duration + now) / 1000
        });

        let activity = Activity::new()
            .state(self.state.as_str())
            .details(self.details.as_str())
            .timestamps(timestamps)
            .assets(Assets::new().large_image(self.large_image.as_str()));

        activity
    }
}
