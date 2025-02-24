# Gyst Branch Management Proposal

Version: 0.2.0

## Overview

Gyst's new branch management features aim to help developers maintain a cleaner repository and better understand branch health. This proposal covers two primary commands: `branch health` and `branch clean`.

## 1. Branch Health Analysis

Command: `gyst branch health`

### Purpose

Analyze and report the health status of branches in the repository to help developers identify potential issues and maintenance needs.

### Features

#### Health Metrics

- **Age Analysis**

  - Days since creation
  - Days since last commit
  - Days since last push
  - Active vs Inactive status (configurable threshold)

- **Activity Metrics**

  - Total number of commits
  - Commit frequency over time
  - Last author
  - Number of contributors

- **Merge Status**
  - Distance from main/master (commits ahead/behind)
  - Merge conflict probability
  - Last successful merge
  - Unreachable commits

#### Command Options

```bash
gyst branch health [options]

Options:
  --all                 Include all branches (local and remote)
  --remote             Only remote branches
  --local              Only local branches
  --days <number>      Consider activity within last N days
  --author <name>      Filter by author
  --format <format>    Output format (text, json, markdown)
```

#### Sample Output

```
Branch Health Report
Last updated: 2024-02-24 10:30 UTC

feature/user-auth
├── Status: 🟡 Needs Attention
├── Age: 15 days
├── Last Activity: 7 days ago
├── Author: John Doe
├── Commits: 23 (3.2/day)
└── Main Distance: 5 ahead, 2 behind

bugfix/login-error
├── Status: 🔴 Stale
├── Age: 45 days
├── Last Activity: 30 days ago
├── Author: Jane Smith
├── Commits: 8 (0.2/day)
└── Main Distance: 8 ahead, 15 behind
```

## 2. Branch Cleanup

Command: `gyst branch clean`

### Purpose

Identify and safely clean up branches that are no longer needed, reducing repository clutter and improving maintenance.

### Features

#### Analysis Criteria

- **Merged Branches**

  - Fully merged into main/master
  - Merged into other active branches
  - Time since merge

- **Stale Branches**

  - No activity threshold (configurable)
  - No recent commits
  - No associated PRs

- **Orphaned Branches**
  - No upstream tracking
  - Unreachable commits
  - Deleted remote branches

#### Command Options

```bash
gyst branch clean [options]

Options:
  --dry-run            Show what would be done
  --interactive, -i    Interactive mode for selection
  --force             Skip confirmation
  --older-than <days> Only consider branches older than N days
  --merged-only       Only fully merged branches
  --prune-remote      Also prune remote references
```

#### Interactive Mode Features

- Checkbox selection for branches
- Preview of changes
- Undo capability
- Batch operations

#### Safety Features

- Protected branch detection
- Recent commit warnings
- Unmerged changes detection
- Backup branch creation (optional)
- Dry run mode

### Configuration

```toml
[branch]
# Health thresholds
stale_days = 30
inactive_days = 7
high_risk_commits = 50

# Protected patterns
protected_patterns = [
  "main",
  "master",
  "develop",
  "release/*"
]

# Cleanup settings
auto_delete_merged = false
backup_before_delete = true
```

## Implementation Plan

### Phase 1: Core Features

1. Implement basic branch information gathering
2. Add health metrics calculation
3. Develop cleanup identification logic

### Phase 2: Safety & Usability

1. Add protection mechanisms
2. Implement interactive mode
3. Add configuration management

### Phase 3: Advanced Features

1. Add merge analysis
2. Implement conflict prediction
3. Add detailed reporting

Would you like to proceed with implementing any specific component of these features?
