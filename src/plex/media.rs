use serde::Deserialize;

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
    pub parent_title: String, // album
    pub grandparent_title: String, // artist
    pub duration: u32,
    pub view_offset: u32, // current played time
    #[serde(rename="Player")]
    pub player: Player,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaContainer {
    pub size: i32,
    #[serde(rename="Metadata")]
    pub metadata: Vec<Metadata>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    #[serde(rename="MediaContainer")]
    pub media_container: MediaContainer,
}
