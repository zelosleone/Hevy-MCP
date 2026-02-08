# Hevy MCP Server

MCP server for the [Hevy](https://hevy.com) fitness API, with HTTP transport and session management. Entirely written in Rust for low memory management. Also, proper async support!!!

## Quick start with Docker

```bash
docker pull ghcr.io/zelosleone/hevy-mcp-server:latest
docker run -e HEVY_API_KEY=your-api-key -p 5000:5000 ghcr.io/zelosleone/hevy-mcp-server:latest
```

## Requirements

- Hevy PRO subscription
- Hevy API key from https://hevy.com/settings?developer

## Configuration

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `HEVY_API_KEY` | No | -- | API key from Hevy developer settings. Enables single-user mode. If unset, each client must provide `?apikey=xxx` on url parameter. |
| `HEVY_HTTP_ADDR` | No | `127.0.0.1:3000` (source) / `0.0.0.0:5000` (Docker) | Socket address to bind the HTTP server |
| `HEVY_MCP_PATH` | No | `/` | HTTP route path for the MCP endpoint. A leading `/` is added automatically if missing. |
| `HEVY_SESSION_TIMEOUT_SECS` | No | `3600` | Seconds of inactivity before a session expires |

## Operating modes

### Single-user mode

Set the `HEVY_API_KEY` environment variable. All sessions share this key -- no query parameter needed.

```bash
HEVY_API_KEY=your-api-key ./target/release/hevy-mcp-server
```

### Multi-user mode

Leave `HEVY_API_KEY` unset. Each client must pass its own key as a query parameter during initialization:

```
POST /?apikey=xxx
```

If neither method provides a key, the server responds with an error.

## HTTP transport

The server exposes two endpoints on the configured path (default `/`):

### `POST {path}` -- MCP requests

Send JSON-RPC requests and notifications. The first request must be an `initialize` call. The response includes an `Mcp-Session-Id` header that must be sent on all subsequent requests.

```
POST /
Content-Type: application/json
Mcp-Session-Id: <session-id>
```

### Session lifecycle

- A new session is created on each `initialize` request and a UUID is returned via the `Mcp-Session-Id` response header.
- All non-initialize requests must include the `Mcp-Session-Id` header; requests without it are rejected.
- Sessions are automatically cleaned up after `HEVY_SESSION_TIMEOUT_SECS` seconds of inactivity (default 3600). A background task checks for expired sessions every 60 seconds.
- Sessions can also be deleted explicitly via the `DELETE` endpoint.

## Available tools

### Workouts
- `get_workouts` -- List workouts with pagination. Params: `page` (1-indexed), `page_size` (max 10).
- `get_workouts_count` -- Get the total number of workouts on the account.
- `get_workout_events` -- List workout update/delete events since a timestamp. Params: `page`, `page_size` (max 10), `since` (ISO 8601).
- `get_workout` -- Get a single workout by ID. Returns full details including exercises and sets.
- `create_workout` -- Create a workout. Required: `title`, `start_time`, `end_time` (ISO 8601), `exercises` with `sets`. Optional: `is_private`, `description`.
- `update_workout` -- Replace an existing workout by ID. Same fields as `create_workout` plus `id`.

### Routines
- `get_routines` -- List routines with pagination. Params: `page`, `page_size` (max 10).
- `get_routine` -- Get a single routine by ID, including exercises and set templates.
- `create_routine` -- Create a routine (workout template). Required: `title`, `folder_id`, `exercises` with `sets`. Optional: `notes`.
- `update_routine` -- Replace an existing routine by ID. Same fields as `create_routine` plus `id`.

### Exercise Templates
- `get_exercise_templates` -- List exercise templates from the Hevy library. Params: `page`, `page_size` (max 100).
- `get_exercise_template` -- Get an exercise template by ID, including muscle groups and equipment.
- `create_exercise_template` -- Create a custom exercise template. Required: `title`, `exercise_type`, `equipment_category`, `muscle_group`. Optional: `other_muscles`.

### Routine Folders
- `get_routine_folders` -- List routine folders with pagination. Params: `page`, `page_size` (max 10).
- `get_routine_folder` -- Get a routine folder by ID.
- `create_routine_folder` -- Create a folder to organize routines. Required: `title`.

### Exercise History
- `get_exercise_history` -- Get workout history for an exercise template. Required: `exercise_template_id`. Optional: `start_date`, `end_date` (ISO 8601).

## From source

```bash
cargo build --release
HEVY_API_KEY=your-api-key ./target/release/hevy-mcp-server
```

The server binds to `127.0.0.1:3000` by default. Override with `HEVY_HTTP_ADDR`:

```bash
HEVY_API_KEY=your-api-key HEVY_HTTP_ADDR=0.0.0.0:8080 ./target/release/hevy-mcp-server
```

## License

MIT License - See [LICENSE](LICENSE) for details.

## References

- https://api.hevyapp.com/docs/
