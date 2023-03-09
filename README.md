# light-control

A small CLI for philips hue

## Usage

```bash
light-control off
```

```bash
light-control <scene name>
```

You can also specify a different config file:

```bash
light-control -c my-config.json <scene name>
```

## Configuration

Create your configuration in your home directory: `~/light_control_config.json`

```json
{
  "ip": "192.168.1.1",
  "user_id": "abcdefghijklmnopqrstuvwxyz",
  "room_id": 1,
  "scene_config": {
    "tokio": "abcdefghijklmno"
  }
}
```
