# Delegation Commands

Amazon Q CLI supports delegating tasks to specialized agents for more focused assistance.

## Available Commands

### `/delegate to <agent> [task]`
Delegate a task to a specific agent.

**Examples:**
```
/delegate to code-reviewer review this function
/delegate to debugger find the bug in this code
/delegate to optimizer improve performance
```

### `/delegate list`
List all available agents for delegation.

**Example:**
```
/delegate list
```

## How Delegation Works

1. **Automatic Detection**: Q can automatically detect when to delegate based on your input
2. **Manual Delegation**: Use `/delegate to` commands for explicit delegation
3. **Context Inheritance**: Agents receive relevant conversation context based on their configuration
4. **Seamless Integration**: Results are integrated back into your main conversation

## Agent Context Levels

- **None**: Agent starts with no previous context
- **Minimal**: Agent receives last 2-3 messages
- **Partial**: Agent receives messages relevant to the task
- **Full**: Agent receives complete conversation history (limited to last 50 messages)

## Performance

- Delegation overhead: <100ms
- Context creation: <50ms
- Memory usage increase: <15%

## Troubleshooting

If delegation fails, Q automatically falls back to the main agent to ensure continuity.
