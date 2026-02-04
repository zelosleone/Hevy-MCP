# Hevy MCP Server

MCP server for the Hevy fitness API.

## Quick start with Docker

```bash
docker pull ghcr.io/zelosleone/hevy-mcp-server:latest
docker run -e HEVY_API_KEY=your-api-key -p 5000:5000 ghcr.io/zelosleone/hevy-mcp-server:latest
```

## Requirements

- Hevy PRO subscription
- Hevy API key from https://hevy.com/settings?developer

## Configuration

| Variable | Required | Description |
|----------|----------|-------------|
| `HEVY_API_KEY` | Yes | API key from Hevy developer settings |
| `HEVY_HTTP_ADDR` | No | Bind address, default is 0.0.0.0:5000 in Docker |

## Available tools

### Workouts
- `get_workouts` - List workouts with pagination
- `get_workouts_count` - Get the total number of workouts
- `get_workout_events` - List workout events since a given date
- `get_workout` - Get a single workout by ID
- `create_workout` - Create a new workout
- `update_workout` - Update an existing workout

### Routines
- `get_routines` - List routines with pagination
- `get_routine` - Get a single routine by ID
- `create_routine` - Create a new routine
- `update_routine` - Update an existing routine

### Exercise Templates
- `get_exercise_templates` - List exercise templates with pagination
- `get_exercise_template` - Get a single exercise template by ID
- `create_exercise_template` - Create a new custom exercise template

### Folders
- `get_routine_folders` - List routine folders with pagination
- `create_routine_folder` - Create a new folder
- `get_routine_folder` - Get a routine folder by ID

### Exercise History
- `get_exercise_history` - Get exercise history for a template

## From source

```bash
cargo build --release
HEVY_API_KEY=your-api-key HEVY_HTTP_ADDR=127.0.0.1:5000 ./target/release/hevy-mcp-server
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## References

- https://api.hevyapp.com/docs/
