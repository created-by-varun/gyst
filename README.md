# Gyst - AI-Powered Git Commit Assistant

Version: 0.1.0

## Overview

Gyst is a command-line tool that simplifies git commit workflows by using AI to analyze changes and generate meaningful commit messages. It helps developers maintain consistent commit history and save time while following best practices.

## Installation

```bash
# Using cargo
cargo install gyst

# From source
git clone https://github.com/yourusername/gyst
cd gyst
cargo install --path .
```

## Core Features

### 1. Smart Commit Message Generation

```bash
gyst commit [--quick]
```

- Analyzes staged changes in git repository
- Generates contextual commit messages using AI
- Provides interactive mode for message selection/editing
- Quick mode for faster workflows
- Options:
  - `--quick, -q`: Skip confirmation and use first suggestion
  - `--edit, -e`: Open in editor for manual modifications

### 2. Multiple Suggestions

```bash
gyst suggest [--count <number>]
```

- Generates multiple commit message alternatives
- Allows selection from different options
- Options:
  - `--count, -c`: Number of suggestions (default: 3)
  - `--format, -f`: Output format (default: interactive)

### 3. Diff Analysis

```bash
gyst diff
```

- Shows detailed analysis of staged changes
- Provides statistics about modifications
- Categorizes changes by type:
  - Added files
  - Modified files
  - Deleted files
  - Renamed files
- Statistics include:
  - Number of files changed
  - Lines added/removed
  - File-specific changes

### 4. Configuration Management

```bash
gyst config [options]
```

- Manages tool configuration and API settings
- Options:
  - `--api-key`: Set AI service API key
  - `--show`: Display current configuration
  - `--edit`: Open configuration in editor
  - `--reset`: Reset to default settings

## Configuration

### Location

- Global: `~/.gyst/config.toml`
- Project-specific: `.gyst.toml` in project root

### Configuration Options

```toml
[ai]
provider = "openai"  # or "anthropic"
api_key = "your-api-key"
model = "gpt-4"      # or "claude-3"

[git]
max_diff_size = 1000 # lines
protected_branches = ["main", "master"]

[commit]
template = "conventional"  # or "custom"
max_subject_length = 72
max_body_length = 500
```

## Advanced Features

### 1. Conventional Commits Support

- Automatically formats messages following conventional commits
- Categories:
  - feat: New features
  - fix: Bug fixes
  - docs: Documentation
  - style: Formatting
  - refactor: Code restructuring
  - test: Testing
  - chore: Maintenance

### 2. Smart Analysis

- Detects:
  - Breaking changes
  - API modifications
  - Dependency updates
  - Security-sensitive changes
  - Configuration changes

### 3. Integration Features

- Git hooks compatibility
- CI/CD pipeline integration
- Project-specific configurations

## Error Handling

- Repository validation
- API connectivity issues
- Rate limiting
- Invalid configurations
- Git operation errors

## Best Practices

1. Stage changes before running commands
2. Review suggested messages before accepting
3. Use project-specific configurations for team consistency
4. Keep API keys secure
5. Regular updates for latest features

## Command Examples

```bash
# Generate commit message
gyst commit

# Quick commit without confirmation
gyst commit --quick

# Get 5 message suggestions
gyst suggest --count 5

# Analyze current changes
gyst diff

# Configure API key
gyst config --api-key "your-key-here"

# Show current configuration
gyst config --show
```

## Development Status

- Current version: 0.1.0
- Status: Alpha
- Planned features:
  - Custom templates
  - Team collaboration features
  - Multi-language support
  - IDE integrations

Would you like me to expand on any particular section or add more details to specific features?
