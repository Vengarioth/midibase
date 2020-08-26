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

```json
{
    "commands": [
        {
            "command": "SetCurrentScene",
            "button": 50,
            "scene": "Game"
        },
        {
            "command": "SetCurrentScene",
            "button": 51,
            "scene": "Queue"
        },
        {
            "command": "PlaySound",
            "button": 36,
<<<<<<< HEAD
            "file": "./resources/test-sounds/1.wav",
            "volume": 1.0
=======
            "file": "./resources/test-sounds/1.wav"
>>>>>>> d41269db2af93c4f508f96674a4d8e9d1698f02e
        }
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
