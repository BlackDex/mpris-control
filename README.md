# mpris-control

[![GitHub Release](https://img.shields.io/github/v/release/BlackDex/mpris-control.svg)](https://github.com/BlackDex/mpris-control/releases/latest)
[![GitHub Issues](https://img.shields.io/github/issues/BlackDex/mpris-control)](https://github.com/BlackDex/mpris-control/issues)
[![Dependency status](https://deps.rs/repo/github/BlackDex/mpris-control/status.svg)](https://deps.rs/repo/github/BlackDex/mpris-control)
[![GPL-3.0 Licensed](https://img.shields.io/github/license/BlackDex/mpris-control.svg)](https://github.com/BlackDex/mpris-control/blob/master/LICENSE)


A small application to control mpris-2 mediaplayers like spotify, vlc, rhythmbox, chromium etc..<br>
<br>
I Created this because the spotify-app (snap) didn't seem to work with the default media key's configuration of Ubuntu 18.04.<br>
<br>
This application works with all mpris-2 dbus controlled applications as far as i know.<br>
If you encounter any issues please report them in the issues section.<br>
<br>
**Simple usage:**
```bash
mpris-control 0.4.3
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

# Examples

# Trigger action for "active" player (First in the list only)
mpris-control play

# List all the players it can find
mpris-control list --all

# Stop all players
mpris-control stop --all

# Ignore specific players: This will ignore both VLC and Chromium, but triggers all others
mpris-control play --ignore "VLC media player","Chromium"

# Target specific players: This will target only Spotify
mpris-control toggle --target Spotify

# OR
mpris-control pause --target Spotify,"VLC media player"

# To check if a filter works, you can use "list" command with the --target or --ignore options
mpris-control list --ignore Spotify

# OR
mpris-control list --target Spotify

```

## Notes
Some packages are needed to build some external crates.<br>
```bash
# Ubuntu/Debian
apt install libdbus-1-dev
```
