# Data Structure
## Top level
Create a `json` file with a name like `my_story.json`.
```json
{
    "title": "Adventure Game",
    "author": "TRAIN",
    "start_step": "START",
    "steps" : [
        ...we put blocks here...
        ...separated by commas...
    ]
}
```

## Basic Blocks
### Decision Block
The `opts` bits specify the options, and `goto` points to the next blocks `name`.
```json
{
    "name": "...",
    "type": "decision",
    "text": "...",
    "opts": [
        {
            "text": "...",
            "goto": "..."
        },
        {
            "text": "...",
            "goto": "..."
        },
        {
            "text": "...",
            "goto": "..."
        }
    ]
}
```

### Forwarder Block
```json
{
    "name": "...",
    "type": "forwarder",
    "text": "...",
    "goto": "..."
}
```

### End Block
```json
{
    "name": "...",
    "type": "end",
    "text": "..."
},
```