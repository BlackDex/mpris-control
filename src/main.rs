#![forbid(unsafe_code, non_ascii_idents)]
#![deny(
    rust_2018_idioms,
    rust_2021_compatibility,
    noop_method_call,
    pointer_structural_match,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    clippy::cast_lossless,
    clippy::clone_on_ref_ptr,
    clippy::equatable_if_let,
    clippy::float_cmp_const,
    clippy::inefficient_to_string,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::manual_assert,
    clippy::manual_instant_elapsed,
    clippy::manual_string_new,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::string_add_assign,
    clippy::string_to_string,
    clippy::unnecessary_join,
    clippy::unnecessary_self_imports,
    clippy::unused_async,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values
)]
use mpris::PlayerFinder;
use pico_args::Arguments;

fn main() {
    // The order of extracting the flags, options and subcommands is important
    // First we check for the help and version flags, and stop on those
    // After that we extract the subcommand/action
    // Then we will try to parse the options
    // And as last option try to extract a fallback action to be compatible with the previous versions using clap
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        show_help();
        std::process::exit(0);
    }

    if args.contains(["-V", "--version"]) {
        println!("mpris-control {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let main_action = args.subcommand().unwrap_or_default();
    let control_all = args.contains(["-a", "--all"]);
    let ignore: Option<String> = args.opt_value_from_str(["-i", "--ignore"]).unwrap_or_default();
    let target: Option<String> = args.opt_value_from_str(["-t", "--target"]).unwrap_or_default();
    let fallback_action = args.opt_free_from_str::<String>().unwrap_or_default();

    let action = match main_action {
        Some(a) => a,
        _ => match fallback_action {
            Some(fa) => fa,
            _ => {
                eprintln!("No action found!\n");
                show_help();
                std::process::exit(1);
            }
        },
    };

    if action == "help" {
        show_help();
        std::process::exit(0);
    }

    match run_action(action, ignore, target, control_all) {
        Ok(msg) => {
            println!("{msg}");
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

fn show_help() {
    println!("mpris-control {}", env!("CARGO_PKG_VERSION"));
    print!("{HELP}");
}

fn run_action(
    action: String,
    ignore: Option<String>,
    target: Option<String>,
    control_all: bool,
) -> Result<String, String> {
    let player_finder = PlayerFinder::new().map_err(|e| format!("Could not connect to D-Bus: {e}"))?;

    let mut players = player_finder.find_all().map_err(|e| format!("Could not find any player: {e}"))?;

    //
    if players.is_empty() {
        return Err("No MPRIS players found".to_string());
    }

    // Check if we need to list all players which we can control, in which case we skip filtering.
    if !control_all {
        // Check if there are ignore or target values, of not, we only want one value, and we are going to be lazy (as does the mpris crate) and pick the first one in the list
        if ignore.is_none() && target.is_none() {
            players.drain(1..);

        // Filter for the ignore values
        } else if ignore.is_some() {
            for ignore_player in ignore.unwrap().split(',') {
                players.retain(|x| x.identity().to_lowercase() != ignore_player.to_lowercase());
            }

        // Filter for the target values
        } else if let Some(target_list) = target {
            players.retain(|x| {
                let mut keep = false;
                for target_player in target_list.split(',') {
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
        return Ok(String::new());
    }

    if !players.is_empty() {
        let mut controlled_players: Vec<String> = Vec::new();
        for player in players {
            controlled_players.push(player.identity().to_string());

            if action == "toggle" {
                player.play_pause().map_err(|e| format!("Could not control player: {e}"))?;
            } else if action == "play" {
                player.play().map_err(|e| format!("Could not control player: {e}"))?;
            } else if action == "pause" {
                player.pause().map_err(|e| format!("Could not control player: {e}"))?;
            } else if action == "next" {
                player.next().map_err(|e| format!("Could not control player: {e}"))?;
            } else if action == "previous" {
                player.previous().map_err(|e| format!("Could not control player: {e}"))?;
            } else if action == "stop" {
                player.stop().map_err(|e| format!("Could not control player: {e}"))?;
            } else {
                return Err(format!("Unrecognized option: {action}"));
            }
        }

        Ok(format!("Action {} run on {}", action, controlled_players.join(" and ")))
    } else {
        Err("No players found".to_string())
    }
}

const HELP: &str = "\
BlackDex (https://github.com/BlackDex/mpris-control/)
Control MPRIS enabled media players

USAGE:
    mpris-control [FLAGS]|<SUBCOMMAND> [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --all                Controll all players instead of only the filtered or first one
    -i, --ignore <IGNORE>    List of players to ignore separated by commas
    -t, --target <TARGET>    List of player to control separated by commas

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    list        List of players to be controlled (use --all to show all active players)
    next        Next song
    pause       Pause
    play        Play
    previous    Previous song
    stop        Stop playback
    toggle      Toggle playback

";
