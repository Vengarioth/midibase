# Midibase Events

An event can be broken down into 3 parts, the midi button, whether it's on press or release, and the list of commands that the event produces.

An event that has no commands nothing might look like this:
```json
{
    "midibase_events": [
        {
            "button":46,
            "on_down":true,
            "commands": []
        }
    ]
}
```
Commands always have the command itself, but the other properties are dependant on the command itself:
[SetCurrentScene](#set-current-scene)

<a name="set-current-scene"></a>
### SetCurrentScene

```json
{
    "midibase_events": [
        {
            "button":46,
            "on_down":true,
            "commands": []
        }
    ]
}
```