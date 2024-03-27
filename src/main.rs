mod activity_monitor;
mod arguments;
mod config;
mod discord;
mod plex;

use activity_monitor::ActivityMonitor;
use arguments::parse_args;
use config::Config;
use discord::client::DiscordClient;
use plex::client::PlexClient;
use single_instance::SingleInstance;
use std::panic;

#[tokio::main]
async fn main() {
    parse_args().await;
    let cfg = Config::load();

    let instance = SingleInstance::new("plex-presence").unwrap();
    if !instance.is_single() {
        panic!("program already running");
    }

    ActivityMonitor::new(
        DiscordClient::new(&cfg.discord_application_id),
        PlexClient::new(&cfg.token, &cfg.origin),
        &cfg,
    )
    .start()
    .await;
}
