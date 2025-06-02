# Autobuild

An automated build and publish tool that monitors Git repository changes and automatically executes build and publish commands.

## Features

- Monitor updates on specified Git branches
- Automatically execute build and publish commands
- Support for DingTalk webhook notifications
- Configurable check interval
- Detailed build logs and time statistics

## Installation

```bash
cargo install autobuild
```

## Usage

1. Create a configuration file `autobuild.json` (optional):

```bash
# Create default configuration file using --init command
autobuild --init
```

Or create the configuration file manually:

```json
{
  "repository": ".",
  "build": "npm run build",
  "publish": "npm run publish",
  "branch": "main",
  "interval": 10,
  "webhook": {
    "url": "https://oapi.dingtalk.com/robot/send?access_token=YOUR_TOKEN",
    "prefix": "Autobuild"
  }
}
```

2. Run the program:

```bash
# Use default configuration
autobuild

# Specify configuration file
autobuild -c path/to/autobuild.json

# Force execute build and publish commands without checking git updates
autobuild -f
# or
autobuild --force
```

## Configuration

- `repository`: Git repository path
- `build`: Build command
- `publish`: Publish command
- `branch`: Branch to monitor
- `interval`: Check interval in seconds
- `webhook`: DingTalk robot configuration
  - `url`: DingTalk robot webhook URL
  - `prefix`: Message prefix

## License

MIT
