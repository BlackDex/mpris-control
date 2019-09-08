//src main.rs
use clap::{App, AppSettings, Arg, SubCommand};
use mpris::PlayerFinder;

fn main() {
    let matches = App::new("mpris-control")
        .version("0.3.0")
        .author("BlackDex (https://github.com/BlackDex/mpris-control/)")
        .about("Control MPRIS enabled media players")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("play").about("Play"))
        .subcommand(SubCommand::with_name("pause").about("Pause"))
        .subcommand(SubCommand::with_name("next").about("Next song"))
        .subcommand(SubCommand::with_name("previous").about("Previous song"))
        .subcommand(SubCommand::with_name("stop").about("Stop playback"))
        .subcommand(SubCommand::with_name("toggle").about("Toggle playback"))
        .subcommand(SubCommand::with_name("list").about("List of players to be controlled (use --all to show all active players)"))
        .arg(
            Arg::with_name("control_all")
                .short("a")
                .long("all")
                .help("Controll all players instead of only the filtered or first one")
                .takes_value(false)
                .multiple(false)
                .conflicts_with("ignore")
                .conflicts_with("target")
                .required(false),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .value_name("IGNORE")
                .help("List of players to ignore separated by commas")
                .takes_value(true)
                .multiple(false)
                .conflicts_with("target")
                .required(false),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET")
                .help("List of player to control separated by commas")
                .takes_value(true)
                .multiple(false)
                .conflicts_with("ignore")
                .required(false),
        )
        .get_matches();

    let ignore = match matches.value_of("ignore") {
        Some(i) => i,
        None => "",
    };

    let target = match matches.value_of("target") {
        Some(i) => i,
        None => "",
    };

    let control_all = matches.is_present("control_all");

    let action = matches.subcommand().0;
    
    match run_action(action, ignore, target, control_all) {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

fn run_action(action: &str, ignore: &str, target: &str, control_all: bool) -> Result<String, String> {
    let player_finder =
        PlayerFinder::new().map_err(|e| format!("Could not connect to D-Bus: {}", e))?;

    let mut players = player_finder
        .find_all()
        .map_err(|e| format!("Could not find any player: {}", e))?;

    // Check if we need to list all players which we can control, in which case we skip filtering.
    if !control_all {
        // Check if there are ignore or target values, of not, we only want one value, and we are going to be lazy (as does the mpris crate) and pick the first one in the list
        if ignore.is_empty() && target.is_empty() {
            players.drain(1..);

        // Filter for the ignore values
        } else if !ignore.is_empty() {
            for ignore_player in ignore.split(',') {
                players.retain(|x| x.identity().to_lowercase() != ignore_player.to_lowercase());
            }

        // Filter for the target values
        } else if !target.is_empty() {
            players.retain(|x| {
                let mut keep = false;
                for target_player in target.split(',') {
                    if x.identity().to_lowercase() == target_player.to_lowercase() {
                        keep = true;
                    }
                }
                keep
            });
        }
    }

    // List all the players which are to be triggered after the ignore or target checks (unless the --all flag is used)
    if action == "list" {
        println!("Players which can be controlled:");
        for player in players {
            println!(" - {:?}", player.identity());
        }
        return Ok("".to_string());
    }

    if !players.is_empty() {
        let mut controlled_players: Vec<String> = Vec::new();
        for player in players {
            controlled_players.push(player.identity().to_string());

            if action == "toggle" {
                player
                .play_pause()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else if action == "play" {
                player
                .play()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else if action == "pause" {
                player
                .pause()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else if action == "next" {
                player
                .next()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else if action == "previous" {
                player
                .previous()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else if action == "stop" {
                player
                .stop()
                .map_err(|e| format!("Could not control player: {}", e))?;
            } else {
                return Err(format!("Unrecognized option: {}", action));
            }
        }

        Ok(format!("Action {} run on {}", action, controlled_players.join(" and ")))
    } else {
        Err("No players found".to_string())
    }
}
