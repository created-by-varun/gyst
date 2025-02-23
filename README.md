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

Before using Gyst, you'll need to set up your AI provider API key:

```bash
gyst config --api-key your-api-key-here
```

The configuration is stored in `~/.gyst/config.toml`:

```toml
[ai]
provider = "anthropic"  # AI provider (currently supports Anthropic)
api_key = "your-api-key"
model = "claude-3-5-haiku-20241022"  # Model to use

[git]
max_diff_size = 1000  # Maximum diff size in lines

[commit]
max_subject_length = 72  # Maximum length of commit subject line
```

## Commands

### Generate and Create Commit

```bash
gyst commit [options]
```

Analyzes staged changes and generates a commit message using AI.

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

### Configure Settings

```bash
gyst config [options]
```

Manage Gyst configuration settings.

**Options:**

- `--api-key <key>`: Set the AI service API key
- `-s, --show`: Show current configuration (both forms work)

**Example:**

```bash
# Set API key
gyst config --api-key your-api-key-here

# View current config (using short form)
gyst config -s

# View current config (using long form)
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
3. **API Key**: Set up your API key using `gyst config --api-key` before using any AI features
4. **Conventional Commits**: Gyst follows the conventional commit format:
   - Format: `type(scope): description`
   - Types: feat, fix, docs, style, refactor, perf, test, chore, ci, build

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](LICENSE)
