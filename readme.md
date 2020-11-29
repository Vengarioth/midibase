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
    "midibase_events": [
        {
            "button": 50,
            "on_down": true,
            "commands": [
                {
                    "command": "SetCurrentScene",
                    "scene": "Scene"
                },
                {
                    "command": "PlaySound",
                    "file": "./sounds/1.wav",
                    "volume": 1.0
                },
                {
                    "command": "SetAudioMute",
                    "audio_source": "Mic/Aux 3",
                    "mute": false
                },
                {
                    "command": "SetSceneItemProperties",
                    "scene_name": "Scene",
                    "item": "BRBPhoto",
                    "options": [
                        {
                            "option_name": "visible",
                            "option_value": "true"
                        }
                    ]
                }
            ]
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

## resources

[OBS-Websocket OBS Plugin Page](https://obsproject.com/forum/resources/obs-websocket-remote-control-obs-studio-from-websockets.466/)
[OBS-Websocket Github](https://github.com/Palakis/obs-websocket)
[OBS-Websocket Protocol Reference](https://github.com/Palakis/obs-websocket/blob/4.x-current/docs/generated/protocol.md)