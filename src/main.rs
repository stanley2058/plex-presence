mod config;
mod plex;
mod discord;

use std::{env, thread, time, fs, panic};
use std::process::{Command, Stdio, exit};
use std::collections::HashSet;
use ctrlc;
use plex::client::PlexClient;
use discord::rich_presence::DiscordClient;
use config::config::Config;

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
        println!("The lockfile is still present:\n{}", lockfile.to_str().unwrap());
        println!("This means there might already be a running instance.");
        println!("If the program exited violently last time, delete the lockfile manually.");
        println!("=========================================================================");
        panic!("lockfile exist");
    }
    let _ = fs::write(lockfile, "");
}

fn handle_term_signal() {
    let _ = ctrlc::set_handler(move || {
        let lockfile = Config::get_lockfile();
        let _ = fs::remove_file(lockfile);
        exit(0);
    });
}

#[tokio::main]
async fn main() {
    parse_args();
    let cfg = Config::load();
    check_instance();
    handle_term_signal();

    let plex_client = PlexClient::new(&cfg);
    let dc_client = &mut DiscordClient::new(&cfg);
    loop {
        let session_res = plex_client.get_session().await;
        if session_res.is_ok() {
            let session = session_res.unwrap();
            let act = dc_client.plex_session_to_activity(&session);
            dc_client.update_plex_activity(act);
        } else {
            println!("cannot get session, {:#?}", session_res.unwrap_err());
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
