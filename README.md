# Gyst - AI-Powered Git Commit Assistant

Gyst is a command-line tool that simplifies git commit workflows by using AI to analyze changes and generate meaningful commit messages. It helps developers maintain consistent commit history and save time while following best practices.

## Features

- **AI-Powered Commit Messages**: Automatically generate meaningful commit messages based on your changes
- **Conventional Commit Format**: All messages follow the conventional commit format
- **Multiple Suggestions**: Get multiple commit message options to choose from
- **Quick Mode**: Fast commit workflow without confirmation prompts
- **Interactive Editing**: Edit generated messages before committing
- **Smart Diff Analysis**: Analyze staged changes for better context
- **Command Help**: Get AI-powered suggestions for Git commands based on what you want to do
- **Cloud-Powered by Default**: Uses our cloud service for AI operations (no API key required)
- **Flexible Configuration**: Option to use direct API access if preferred

## Installation

### Quick Install (macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/created-by-varun/gyst/master/install.sh | bash
```

### Manual Installation

1. Download the latest release for your architecture from the [releases page](https://github.com/created-by-varun/gyst/releases)
2. Make it executable: `chmod +x gyst-darwin-$(uname -m)`
3. Move it to your PATH: `sudo mv gyst-darwin-$(uname -m) /usr/local/bin/gyst`

### Updating

To update gyst to the latest version, you can use either method:

1. Using the install script (recommended):

   ```bash
   curl -fsSL https://raw.githubusercontent.com/created-by-varun/gyst/master/install.sh | bash
   ```

   The script will automatically detect and replace any existing installation.

2. Manual update:

   ```bash
   # Remove existing installation
   sudo rm -f /usr/local/bin/gyst

   # Download and install the latest version
   curl -fsSL https://raw.githubusercontent.com/created-by-varun/gyst/master/install.sh | bash
   ```

To check your current version:

```bash
gyst --version
```

## Configuration

### Server Mode vs. Direct API Mode

Gyst operates in two modes:

1. **Server Mode (Default)**: Uses our cloud service at `https://gyst-cli.vercel.app` to handle AI operations

   - No API key required
   - Faster response times
   - Always up-to-date with the latest models

2. **Direct API Mode**: Connects directly to the Anthropic API
   - Requires your own API key
   - Useful in environments with restricted internet access
   - Gives you control over model selection

By default, Gyst uses Server Mode for the best user experience. You can switch modes using the configuration command:

```bash
# Enable server mode (default)
gyst config --use-server true

# Switch to direct API mode
gyst config --use-server false
```

### Setting Up API Key (Only for Direct API Mode)

If you've disabled server mode, you'll need to set up your AI provider API key:

```bash
gyst config --api-key your-api-key-here
```

The configuration is stored in `~/.gyst/config.toml`:

```toml
[ai]
provider = "anthropic"  # AI provider (currently supports Anthropic)
api_key = "your-api-key" # API key (required only in direct API mode)
model = "claude-3-5-haiku-20241022"  # Model to use

[git]
max_diff_size = 1000  # Maximum diff size in lines

[commit]
max_subject_length = 72  # Maximum length of commit subject line

[server]
use_server = true  # Whether to use server mode (default: true)
```

## Commands

### Generate and Create Commit

```bash
gyst commit [options]
# or use the shorthand
gyst c [options]
```

Analyzes staged changes and generates a commit message using AI.

**Options:**

- `-q, --quick`: Skip confirmation and use the generated message directly
- `-p, --push`: Push changes to the remote repository after committing
- Default behavior: Shows the message and prompts for:
  - `Y` (default): Accept and use the message
  - `n`: Reject and abort commit
  - `e`: Open in editor to modify message

**Example:**

```bash
# Interactive mode (using full command)
gyst commit

# Interactive mode (using shorthand)
gyst c

# Quick mode
gyst commit -q
# or
gyst c -q

# Commit and push
gyst commit -p

# Quick commit and push
gyst commit -q -p
# or
gyst c -qp
```

### Get Multiple Suggestions

```bash
gyst suggest
```

Generates three commit message suggestions for you to choose from. If there are no staged changes, it will offer to stage all changes first.

**Example:**

```bash
# Get 3 suggestions
gyst suggest
```

### Get Git Command Suggestions

```bash
gyst explain "your description here"
```

Get AI-powered suggestions for Git commands based on natural language descriptions of what you want to do.

**Examples:**

```bash
# Find out how to undo the last commit
gyst explain "how do I undo my last commit"

# Learn how to create and switch to a new branch
gyst explain "create and switch to a new branch"

# Get help with resolving merge conflicts
gyst explain "how to resolve merge conflicts"
```

### Branch Health Analysis

```bash
gyst branch health [options]
```

Analyze and report the health status of git branches in your repository. Helps identify stale branches, track activity, and manage branch maintenance.

**Options:**

- `--all`: Include all branches (local and remote)
- `--remote`: Only analyze remote branches
- `--local`: Only analyze local branches (default)
- `--days <number>`: Consider activity within last N days
- `--author <n>`: Filter branches by author
- `--format <format>`: Output format (text, json, markdown)

**Examples:**

```bash
# View health of local branches
gyst branch health

# Include remote branches
gyst branch health --all

# Filter by age (30 days)
gyst branch health --days 30

# Filter by author
gyst branch health --author "John Doe"

# Output in markdown format
gyst branch health --format markdown
```

The command analyzes and reports:

- Branch age and creation date
- Last activity time
- Commit frequency and count
- Author information
- Distance from main branch (commits ahead/behind)
- Overall health status:
  - 🟢 Healthy: Recent activity
  - 🟡 Needs Attention: Inactive for a while
  - 🔴 Stale: No activity for extended period

### Configure Settings

```bash
gyst config [options]
```

Manage Gyst configuration settings.

**Options:**

- `--api-key <key>`: Set the AI service API key (for direct API mode)
- `--use-server <bool>`: Enable or disable server mode (true/false)
- `-s, --show`: Show current configuration (both forms work)

**Example:**

```bash
# Set API key (for direct API mode)
gyst config --api-key your-api-key-here

# Enable server mode (default)
gyst config --use-server true

# Disable server mode and use direct API
gyst config --use-server false

# View current config
gyst config --show
```

### View Diff

```bash
gyst diff
```

Shows a detailed analysis of staged changes including:

- Added files
- Modified files
- Deleted files
- Renamed files
- Change statistics

## Best Practices

1. **Stage Changes**: Always stage your changes using `git add` before using Gyst commands
2. **Review Messages**: While quick mode is convenient, it's recommended to review the AI-generated messages
3. **Server Mode**: Use the default server mode for the best experience without requiring an API key
4. **API Key**: Only set up your API key if you've disabled server mode
5. **Conventional Commits**: Gyst follows the conventional commit format:
   - Format: `type(scope): description`
   - Types: feat, fix, docs, style, refactor, perf, test, chore, ci, build

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](LICENSE)
