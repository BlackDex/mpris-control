# mpris-control
A small application to control mpris-2 mediaplayers like spotify, vlc, rhythmbox etc..<br>
<br>
I Created this because the spotify-app (snap) didn't seem to work with the default media key's configuration of Ubuntu 18.04.<br>
<br>
This application works with all mpris-2 dbus controlled applications as far as i know.<br>
If you encounter any issues please report them in the issues section.<br>
<br>
**Simple usage:**
```bash
# Toggle play/pause
mpris-control toggle

# Play
mpris-control play

# Pause
mpris-control pause

# Stop
mpris-control stop

# Next track
mpris-control next

# Previous track
mpris-control previous
```

## Notes
Some packages are needed to build some external crates.<br>
```bash
# Ubuntu/Debian
apt install libdbus-1-dev
```
