mod activity_monitor;
mod config;
mod discord;
mod plex;

use activity_monitor::ActivityMonitor;
use config::Config;
use discord::client::DiscordClient;
use plex::client::PlexClient;
use single_instance::SingleInstance;
use std::collections::HashSet;
use std::process::{exit, Command, Stdio};
use std::{env, panic};

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    let mut args_set: HashSet<String> = HashSet::from_iter(args.clone());
    if args_set.contains("-d") || args_set.contains("--daemon") {
        args_set.remove("-d");
        args_set.remove("--daemon");
        let _ = Command::new(&args[0])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        exit(0);
    }
}

#[tokio::main]
async fn main() {
    parse_args();
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
