//src main.rs
use clap::{App, AppSettings, Arg, SubCommand};
use mpris::PlayerFinder;

fn main() {
    let matches = App::new("mpris-control")
        .version("0.1.0")
        .author("BlackDex")
        .about("Control MPRIS enabled media players")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .subcommand(SubCommand::with_name("play").about("Play"))
        .subcommand(SubCommand::with_name("pause").about("Pause"))
        .subcommand(SubCommand::with_name("next").about("Next song"))
        .subcommand(SubCommand::with_name("previous").about("Previous song"))
        .subcommand(SubCommand::with_name("stop").about("Stop playback"))
        .subcommand(SubCommand::with_name("toggle").about("Toggle playback"))
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
                .help("Target player (may only be one)")
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
    let action = matches.subcommand().0;
    match run_action(action, ignore, target) {
        Ok(msg) => {
            println!("{:?}", msg);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn run_action(action: &str, ignore: &str, target: &str) -> Result<String, String> {
    let player_finder =
        PlayerFinder::new().map_err(|e| format!("Could not connect to D-Bus: {}", e))?;

    let mut players = player_finder
        .find_all()
        .map_err(|e| format!("Could not find any player: {}", e))?;

    for i in ignore.split(',') {
        players.retain(|x| x.identity() != i);
    }
    if target != "" {
        players.retain(|x| x.identity() == target);
    }

    if players.len() > 0 {
        if action == "toggle" {
            players[0]
                .play_pause()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else if action == "play" {
            players[0]
                .play()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else if action == "pause" {
            players[0]
                .pause()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else if action == "next" {
            players[0]
                .next()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else if action == "previous" {
            players[0]
                .previous()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else if action == "stop" {
            players[0]
                .stop()
                .map_err(|e| format!("Could not control player: {}", e))?;
        } else {
            return Err(format!("Unrecognized option: {}", action));
        }

        Ok(format!("Action {} run", action))
    } else {
        return Err(format!("No players found"));
    }
}
