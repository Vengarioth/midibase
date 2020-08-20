# midibase

![ferris using a midi pad](./resources/midibase-small.png)

A command line application to send commands to [obs-websocket](https://obsproject.com/forum/resources/obs-websocket-remote-control-obs-studio-from-websockets.466/) based on midi input.

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
