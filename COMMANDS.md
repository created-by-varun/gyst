# Gyst Commands

Gyst is a command-line tool that helps you write better git commits using AI. Here's a comprehensive list of all available commands and their usage.

## Core Commands

### `gyst commit`

Generate and create a commit with an AI-generated message.

```bash
gyst commit [options]
```

**Options:**

- `-q, --quick`: Skip confirmation and use the generated message directly
- Default behavior: Shows the message and prompts for:
  - `Y` (default): Accept and use the message
  - `n`: Reject and abort commit
  - `e`: Open in editor to modify message

**Example:**

```bash
# Interactive mode
gyst commit

# Quick mode
gyst commit -q
```

### `gyst suggest`

Get multiple commit message suggestions.

```bash
gyst suggest [options]
```

**Options:**

- `-c, --count <number>`: Number of suggestions to generate (default: 3)

**Example:**

```bash
# Get default 3 suggestions
gyst suggest

# Get 5 suggestions
gyst suggest -c 5
```

### `gyst config`

Configure Gyst settings.

```bash
gyst config [options]
```

**Options:**

- `--api-key <key>`: Set the AI service API key
- `-s, --show`: Show current configuration

**Example:**

```bash
# Set API key
gyst config --api-key your-api-key-here

# View current config
gyst config --show
```

### `gyst diff`

Show staged changes with detailed diff.

```bash
gyst diff
```

Shows a detailed analysis of staged changes including:

- Added files
- Modified files
- Deleted files
- Renamed files
- Change statistics

## Configuration

### Global Configuration File

The global configuration file is located at `~/.gyst/config.toml`:

```toml
[ai]
provider = "anthropic"  # AI provider (anthropic)
api_key = "your-api-key"
model = "claude-3-5-haiku-20241022"  # Model to use

[git]
max_diff_size = 1000  # Maximum diff size in lines
protected_branches = ["main", "master"]  # Branches with extra protection

[commit]
max_subject_length = 72  # Maximum length of commit subject line
```

## Best Practices

1. **Staged Changes**: Always stage your changes using `git add` before using Gyst commands.
2. **Review Messages**: While quick mode is convenient, it's recommended to review the AI-generated messages.
3. **API Key**: Set up your API key using `gyst config --api-key` before using any AI features.
4. **Conventional Commits**: Gyst follows the conventional commit format:
   - `type(scope): description`
   - Types: feat, fix, docs, style, refactor, perf, test, chore, ci, build
