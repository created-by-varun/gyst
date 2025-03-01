# Gyst Server

This is the server component for the Gyst AI-powered Git assistant. It provides a centralized API that handles communication with the Anthropic API, allowing the Gyst CLI to function without requiring each user to configure their own API key.

## Features

- Centralized API key management
- Endpoints for commit message generation
- Endpoints for Git command suggestions
- Simple deployment and configuration

## Setup

1. Copy the `.env.example` file to `.env`:

   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file and add your Anthropic API key:

   ```
   ANTHROPIC_API_KEY=your_api_key_here
   ```

3. Build the server:

   ```bash
   cargo build --release
   ```

4. Run the server:
   ```bash
   cargo run --release
   ```

## API Endpoints

### Health Check

```
GET /api/health
```

Returns the server status and version.

### Generate Commit Message

```
POST /api/commit
```

Generates a single commit message based on the provided changes and diff.

Request body:

```json
{
  "changes": {
    "added": ["file1.rs", "file2.rs"],
    "modified": ["file3.rs"],
    "deleted": [],
    "renamed": [["old_name.rs", "new_name.rs"]],
    "stats": {
      "files_changed": 4,
      "insertions": 100,
      "deletions": 50
    }
  },
  "diff": "diff --git a/file1.rs b/file1.rs\n..."
}
```

Response:

```json
{
  "message": "feat(component): add new feature"
}
```

### Generate Multiple Commit Suggestions

```
POST /api/commit/suggestions
```

Generates multiple commit message suggestions.

Request body: Same as `/api/commit` with an optional `count` field.

```json
{
  "changes": { ... },
  "diff": "...",
  "count": 3
}
```

Response:

```json
{
  "suggestions": [
    "feat(component): add new feature",
    "feat(api): implement new endpoint",
    "feat(ui): create user interface"
  ]
}
```

### Suggest Git Command

```
POST /api/command
```

Suggests Git commands based on a natural language description.

Request body:

```json
{
  "description": "how do I undo my last commit"
}
```

Response:

```json
{
  "suggestion": "COMMAND: git reset --soft HEAD~1\nEXPLANATION: This command undoes the last commit but keeps the changes staged.\nNOTE: This is safe to use if you haven't pushed the commit to a remote repository."
}
```

## Deployment

For production deployment, consider:

- Using a reverse proxy like Nginx
- Setting up SSL/TLS
- Running as a systemd service
- Using Docker for containerization
