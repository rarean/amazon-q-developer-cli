# Agent Chaining Guide

Agent chaining in Amazon Q CLI allows you to delegate tasks to specialized agents automatically or explicitly. This feature enables more focused and expert responses for specific types of tasks.

## Overview

The agent chaining system consists of several key components:

- **Agent Registry**: Manages available agents and their capabilities
- **Request Analyzer**: Determines when and how to delegate requests
- **Context Isolator**: Manages context sharing between agents
- **Agent Delegator**: Orchestrates the delegation process

## Agent Configuration

Agents are configured using JSON files with delegation settings:

```json
{
  "name": "code-reviewer",
  "displayName": "Code Reviewer",
  "description": "Specialized agent for code review and quality analysis",
  "model": "claude-3-5-sonnet-20241022",
  "systemPrompt": "You are a senior code reviewer...",
  "delegation": {
    "keywords": ["review", "code", "quality", "security"],
    "taskPatterns": [
      "review this code",
      "check for bugs",
      "analyze security"
    ],
    "autoDelegate": true,
    "priority": 8,
    "contextInheritance": "Partial"
  },
  "allowedTools": ["fs_read", "fs_write"],
  "trustLevel": "default"
}
```

### Delegation Configuration

- **keywords**: Words that trigger automatic delegation
- **taskPatterns**: Specific phrases that indicate this agent should handle the task
- **autoDelegate**: Whether to automatically delegate matching requests
- **priority**: Higher numbers get priority when multiple agents match (1-10)
- **contextInheritance**: How much context to share with the agent

### Context Inheritance Levels

- **None**: Agent starts with no previous context
- **Minimal**: Only the current request and basic session info
- **Partial**: Recent conversation history (last few exchanges)
- **Full**: Complete conversation history and context

## Usage

### Automatic Delegation

When you ask questions that match agent keywords or patterns, delegation happens automatically:

```
User: "Please review this code for security issues"
→ Automatically delegates to code-reviewer agent
```

### Explicit Delegation

You can explicitly delegate to specific agents:

```
/delegate code-reviewer analyze this function
/delegate data-analyst create a chart from this data
```

### Agent Management Commands

- `/agents list` - Show all available agents
- `/agents active` - Show currently active agent
- `/agents clear` - Clear active agent delegation

## Visual Indicators

When an agent is active, you'll see visual indicators:

- **Prompt Prefix**: `🤖 code-reviewer > ` shows active agent
- **Status Line**: `🟢 Active Agent: code-reviewer`
- **Delegation Messages**: `🤖 Delegated to 'code-reviewer': review this code`

## Creating Custom Agents

1. Create a JSON configuration file in your agents directory
2. Define the agent's capabilities and delegation rules
3. Set appropriate keywords and task patterns
4. Configure context inheritance based on your needs

### Example Agent Types

- **Code Reviewer**: Reviews code quality, security, performance
- **Data Analyst**: Analyzes data, creates visualizations, provides insights
- **DevOps Specialist**: Handles infrastructure, deployment, monitoring
- **Documentation Writer**: Creates and maintains documentation
- **Test Engineer**: Writes and reviews tests, test strategies

## Best Practices

### Agent Design

- **Focused Expertise**: Each agent should have a clear, specific domain
- **Clear Keywords**: Use distinctive keywords that don't overlap too much
- **Appropriate Priority**: Set priorities based on specificity and importance
- **Context Needs**: Choose inheritance level based on how much history the agent needs

### Delegation Strategy

- **Auto-Delegation**: Enable for well-defined, frequent tasks
- **Manual Delegation**: Use for complex or sensitive operations
- **Context Management**: Consider privacy and performance when setting inheritance
- **Tool Permissions**: Grant only necessary tools to each agent

### Performance Considerations

- **Agent Loading**: Agents are loaded on-demand to minimize startup time
- **Context Size**: Higher inheritance levels use more tokens
- **Priority Ordering**: Higher priority agents are checked first
- **Keyword Indexing**: Efficient keyword matching for fast delegation decisions

## Troubleshooting

### Common Issues

1. **Agent Not Found**: Check agent configuration file syntax and location
2. **No Auto-Delegation**: Verify keywords match your input and autoDelegate is true
3. **Wrong Agent Selected**: Adjust priorities or make keywords more specific
4. **Context Issues**: Review contextInheritance setting for the agent

### Debugging

- Use `/agents list` to see available agents
- Check agent configuration files for syntax errors
- Review delegation keywords and patterns
- Test with explicit delegation first

## Integration with Existing Features

Agent chaining works seamlessly with:

- **Tool Permissions**: Each agent can have different tool access
- **Context Management**: Agents respect existing context and knowledge systems
- **Session Management**: Delegation state persists within chat sessions
- **MCP Integration**: Agents can use MCP tools based on their configuration

## Security Considerations

- **Tool Access**: Limit agent tool permissions to minimum required
- **Context Isolation**: Use appropriate inheritance levels to protect sensitive information
- **Agent Validation**: Ensure agent configurations are from trusted sources
- **Audit Trail**: Delegation actions are logged for security review
