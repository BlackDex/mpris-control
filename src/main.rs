//src main.rs
use std::env;

extern crate mpris;

use mpris::PlayerFinder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ignored_player = "";
    if args.len() == 2 {
    } else if args.len() == 3 {
        ignored_player = if args.len() == 3 { &args[2] } else { "" };
    } else {
        eprintln!(
            "Usage: mpris-control {{toggle|play|pause|next|previous|stop}} [player_to_ignore]"
        );
        std::process::exit(1);
    }
    let action = &args[1];
    action.trim().to_ascii_lowercase();

    if !action.is_empty() {
        match run_action(action, ignored_player) {
            Ok(msg) => {
                println!("{:?}", msg);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}

fn run_action(action: &str, ignored_player: &str) -> Result<String, String> {
    let player_finder =
        PlayerFinder::new().map_err(|e| format!("Could not connect to D-Bus: {}", e))?;

    let mut players = player_finder
        .find_all()
        .map_err(|e| format!("Could not find any player: {}", e))?;

    players.retain(|x| x.identity() != ignored_player);

    if action == "toggle" {
        players[0]
            .checked_play_pause()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "play" {
        players[0]
            .checked_play()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "pause" {
        players[0]
            .checked_pause()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "next" {
        players[0]
            .checked_next()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "previous" {
        players[0]
            .checked_previous()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "stop" {
        players[0]
            .checked_stop()
            .map_err(|e| format!("Could not control player: {}", e))?;
    } else {
        return Err(format!("Unrecognized option: {}", action));
    }

    Ok(format!("Action {} run", action))
}
