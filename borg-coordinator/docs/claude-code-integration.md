# Claude Code Integration Guide

This document explains how the Borg Coordinator integrates with Claude Code subscriptions and CLI.

## Overview

Instead of using the Anthropic API directly, Borg Coordinator spawns Claude Code CLI processes for each instance. This allows you to use your Claude Code subscription to power multiple collaborative AI agents.

## How It Works

### Instance Creation
1. When you create a new instance, the system creates a dedicated workspace directory
2. Each instance gets its own session file with role-specific context
3. The workspace is isolated from other instances

### Task Execution
1. Tasks are converted into prompts with role-specific context
2. A new Claude Code process is spawned for each task
3. The process runs in the instance's dedicated workspace
4. Results are captured and parsed for file changes and test results

### Process Management
- Each task spawns a separate Claude Code process
- Processes have a 5-minute timeout (configurable)
- Failed processes are automatically cleaned up
- Workspace state is preserved between tasks

## Configuration

### Claude Code Path
Set the path to your Claude Code executable:

```env
CLAUDE_CODE_PATH=claude
# Or if not in PATH:
CLAUDE_CODE_PATH=/usr/local/bin/claude
```

### Workspace Configuration
```env
WORKSPACES_DIR=./workspaces  # Default workspace directory
TASK_TIMEOUT=300000         # 5 minutes in milliseconds
```

## Workspace Structure

```
workspaces/
├── instance-uuid-1/
│   ├── session.md          # Role context and task history
│   ├── project-files/      # Generated/modified files
│   └── logs/              # Instance-specific logs
├── instance-uuid-2/
│   ├── session.md
│   └── ...
└── ...
```

## Session Files

Each instance maintains a session file (`session.md`) that contains:

1. **Role Context**: Specialized prompts for the instance role
2. **Task History**: Record of all assigned tasks
3. **Conversation State**: Maintains context between tasks

Example session file structure:
```markdown
# DEVELOPER Instance Session

You are a Developer Claude instance in a collaborative development environment.

Your responsibilities:
- Implement features and functionality
- Write clean, maintainable code
- Follow established coding patterns and conventions
...

---

## Task: 2024-01-15T10:30:00.000Z

Implement user authentication system with JWT tokens

---

## Task: 2024-01-15T11:15:00.000Z

Add unit tests for the authentication system

---
```

## Process Spawning

For each task, the system:

1. **Appends Task**: Adds the new task to the session file
2. **Spawns Process**: Creates a new Claude Code process with:
   - `--workspace` pointing to the instance directory
   - `--session` pointing to the session file
   - Task prompt sent via stdin
3. **Captures Output**: Collects stdout/stderr from the process
4. **Parses Results**: Extracts file changes and test results
5. **Cleans Up**: Terminates the process when complete

## Error Handling

- **Process Timeout**: Tasks that exceed 5 minutes are terminated
- **Spawn Failures**: Logged with detailed error messages
- **Workspace Issues**: Permission errors are caught and reported
- **Claude Code Errors**: Process exit codes and stderr are captured

## Resource Management

- **Process Isolation**: Each task runs in its own process
- **Memory Limits**: Processes are terminated after completion
- **File System**: Each instance has isolated workspace
- **Concurrent Tasks**: Multiple instances can run simultaneously

## Limitations

1. **Process Overhead**: Each task spawns a new process (startup cost)
2. **Session State**: Context is maintained via files, not memory
3. **Subscription Limits**: Subject to your Claude Code subscription limits
4. **CLI Dependency**: Requires Claude Code CLI to be installed and accessible

## Troubleshooting

### Claude Code Not Found
```bash
# Check if Claude Code is installed
which claude
# or
claude --version
```

### Process Spawn Errors
- Verify CLAUDE_CODE_PATH is correct
- Check file permissions on workspace directory
- Ensure Claude Code authentication is working

### Workspace Issues
- Check write permissions on workspace directory
- Verify disk space availability
- Monitor workspace cleanup

### Performance Issues
- Reduce concurrent instances if experiencing slowdowns
- Increase TASK_TIMEOUT for complex tasks
- Monitor system resources during operation

## Best Practices

1. **Workspace Cleanup**: Periodically clean old workspace directories
2. **Resource Monitoring**: Monitor CPU/memory usage during heavy loads
3. **Session Management**: Archive long session files periodically
4. **Error Monitoring**: Watch logs for repeated process failures
5. **Subscription Limits**: Monitor your Claude Code usage to avoid limits

## Future Improvements

Potential enhancements to the Claude Code integration:

1. **Persistent Processes**: Keep Claude Code processes alive between tasks
2. **Session Optimization**: Better session state management
3. **Parallel Tasks**: Allow multiple tasks per instance
4. **Workspace Templates**: Pre-configured workspace environments
5. **Result Caching**: Cache common task results