extern crate nix;

use anyhow::Result;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use single_instance::SingleInstance;
use std::collections::HashSet;
use std::env;
use std::process::{exit, Command, Stdio};
use tokio::time;

static SOCKET_NAME: &str = "plex-presence";

#[derive(Clone, Default, Debug)]
pub struct RunArgs {
    program_name: String,
    daemon_mode: bool,
    replace: bool,
}

fn replace_alias(input: &str) -> String {
    match input {
        "replace" => String::from("r"),
        "daemon" => String::from("d"),
        _ => String::from(input),
    }
}

fn parse_run_args() -> RunArgs {
    let mut run_args = RunArgs::default();
    let args: Vec<String> = env::args().collect();
    run_args.program_name = args[0].clone();

    let mut args_set: HashSet<char> = HashSet::from_iter(
        args.iter()
            .skip(1)
            .map(|a| a.replace('-', "").replace("--", ""))
            .map(|a| replace_alias(&a))
            .flat_map(|a| a.chars().collect::<Vec<char>>()),
    );

    if args_set.contains(&'r') {
        args_set.remove(&'r');
        run_args.replace = true
    }

    if args_set.contains(&'d') {
        args_set.remove(&'d');
        run_args.daemon_mode = true;
    }

    run_args
}

fn get_socket_pid() -> Result<i32> {
    let result = Command::new("bash")
        .args([
            "-c",
            "ss -axp src '@plex-presence' | awk -F, '{ print $2 }'",
        ])
        .output()?;
    let pid = String::from_utf8(result.stdout)
        .unwrap()
        .trim()
        .replace("pid=", "")
        .parse()?;
    Ok(pid)
}

pub async fn parse_args() {
    let run_args = parse_run_args();

    if run_args.replace {
        if let Ok(pid) = get_socket_pid() {
            let _ = kill(Pid::from_raw(pid), Signal::SIGTERM);
            time::sleep(time::Duration::from_millis(100)).await;
        }
    }

    if run_args.daemon_mode {
        let _ = Command::new(&run_args.program_name)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        exit(0);
    }

    let instance = SingleInstance::new(SOCKET_NAME).unwrap();
    if !instance.is_single() {
        panic!("program is already running, kill the existing program or use the -r option");
    }
}
