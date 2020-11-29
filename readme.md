# midibase

![ferris using a midi pad](./resources/midibase-small.png)

A command line application to send commands to [obs-websocket](https://obsproject.com/forum/resources/obs-websocket-remote-control-obs-studio-from-websockets.466/) based on midi input. 

Includes a midi input controlled soundboard that can play mp3, ogg and wav files.

## installation

```
cargo install midibase
```

## setup

Create a file called `config.json` and add your commands

```json{
    "commands": [
        {
            "command": "SetCurrentScene",
            "button": 50,
            "scene": "Game"
        },
        {
            "command": "SetAudioMute",
            "button": 50,
            "audio_source": "Mic/Aux 3",
            "mute": false
        },
        {
            "command": "SetCurrentScene",
            "button": 51,
            "scene": "Queue"
        },
        {
            "command": "SetAudioMute",
            "button": 51,
            "audio_source": "Mic/Aux 3",
            "mute": true
        },
        {
            "command": "PlaySound",
            "button": 36,
            "file": "./resources/test-sounds/1.wav"
        },
        {
            "command": "ToggleAudioMute",
            "button": 37,
            "audio_source":"Mic/Aux"
        },
    ]
}
```

Then start obs with the obs-websocket plugin installed and run midibase

```
midibase run
```

## uninstall

To remove midibase just run `cargo uninstall midibase`

## license

[MIT](LICENSE)

## known issues

The application doesn't run in dev mode. Running `cargo run` will error, but `cargo run --release -- run` will work.

## resources

[OBS-Websocket OBS Plugin Page](https://obsproject.com/forum/resources/obs-websocket-remote-control-obs-studio-from-websockets.466/)
[OBS-Websocket Github](https://github.com/Palakis/obs-websocket)
[OBS-Websocket Protocol Reference](https://github.com/Palakis/obs-websocket/blob/4.x-current/docs/generated/protocol.md)