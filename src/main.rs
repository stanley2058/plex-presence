mod activity_monitor;
mod cleanup;
mod config;
mod discord;
mod plex;

use activity_monitor::activity_monitor::ActivityMonitor;
use cleanup::cleanup::Cleanup;
use config::config::Config;
use discord::client::DiscordClient;
use plex::client::PlexClient;
use std::collections::HashSet;
use std::process::{exit, Command, Stdio};
use std::{env, fs, panic};

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

fn check_instance() {
    let lockfile = Config::get_lockfile();
    if lockfile.exists() {
        println!("=========================== Lockfile Detected ===========================");
        println!(
            "The lockfile is still present:\n{}",
            lockfile.to_str().unwrap()
        );
        println!("This means there might already be a running instance.");
        println!("If the program exited violently last time, delete the lockfile manually.");
        println!("=========================================================================");
        panic!("lockfile exist");
    }
    let _ = fs::write(lockfile, "");
}

#[tokio::main]
async fn main() {
    parse_args();
    let cfg = Config::load();
    check_instance();
    let _c = Cleanup::new();

    ActivityMonitor::new(
        DiscordClient::new(&cfg.discord_application_id),
        PlexClient::new(&cfg.token, &cfg.origin),
        &cfg,
    )
    .start()
    .await;
}
