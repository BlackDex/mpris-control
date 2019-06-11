//src main.rs
use std::env;

extern crate mpris;

use mpris::PlayerFinder;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let action = &args[1];
        action.trim().to_ascii_lowercase();

        if ! action.is_empty() {
            match run_action(action) {
                Ok(msg) => {
                    println!("{:?}", msg);
                },
                Err(err) => {
                    println!("{:?}", err);
                }
            }        
        }
    } else {
        println!("No action has been given!\nPlease use, toggle, play, pause, next or previous");
    }
}

fn run_action(action: &str) -> Result<String, String> {
    let player_finder = PlayerFinder::new()
        .map_err(|e| format!("Could not connect to D-Bus: {}", e))?;

    let player = player_finder.find_active()
        .map_err(|e| format!("Could not find any player: {}", e))?;

    if action == "toggle" {
       player.checked_play_pause().map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "play" {
        player.checked_play().map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "pause" {
        player.checked_pause().map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "next" {
        player.checked_next().map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "previous" {
        player.checked_previous().map_err(|e| format!("Could not control player: {}", e))?;
    } else if action == "stop" {
        player.checked_stop().map_err(|e| format!("Could not control player: {}", e))?;
    }

    Ok(format!("Action {} run", action))
}
