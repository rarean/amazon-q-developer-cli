# Agent Delegation System

The Amazon Q CLI includes an intelligent agent delegation system that automatically routes tasks to specialized agents based on request analysis and context.

## Quick Start

### Basic Commands

```bash
# List available agents
/agents list

# View active agents
/agents active

# Clear agent history
/agents clear

# Manually delegate a task
/delegate "Deploy my application to AWS"
```

## How It Works

The delegation system analyzes your requests and automatically routes them to the most appropriate specialized agent based on:

- **Request Analysis**: Keywords, patterns, and intent detection
- **Agent Capabilities**: Each agent's specialization and success history
- **Context Inheritance**: Relevant conversation history and file context
- **Confidence Scoring**: Automatic delegation when confidence > 60%

## Agent Types

- **AWS Agent**: Infrastructure, deployments, resource management
- **Code Agent**: Programming, debugging, code review
- **Security Agent**: Security analysis, compliance, vulnerability scanning
- **DevOps Agent**: CI/CD, automation, monitoring

## Configuration Examples

### Basic Agent Configuration

```json
{
  "$schema": "https://raw.githubusercontent.com/aws/amazon-q-developer-cli/main/schemas/agent-v1.1.json",
  "name": "aws-specialist",
  "description": "AWS cloud infrastructure and services expert",
  "prompt": "You are an AWS specialist with deep knowledge of cloud services, infrastructure, and best practices.",
  "delegation": {
    "keywords": ["deploy", "infrastructure", "cloudformation", "s3", "ec2"],
    "autoDelegate": true,
    "priority": 7,
    "contextInheritance": "partial"
  }
}
```

### Advanced Configuration

```json
{
  "$schema": "https://raw.githubusercontent.com/aws/amazon-q-developer-cli/main/schemas/agent-v1.1.json",
  "name": "security-expert",
  "description": "Security analysis and compliance specialist",
  "prompt": "You are a security expert focused on vulnerability assessment, compliance, and security best practices.",
  "delegation": {
    "keywords": ["security", "vulnerability", "compliance", "audit"],
    "taskPatterns": ["scan.*security", "check.*vulnerability", "audit.*code"],
    "autoDelegate": true,
    "priority": 8,
    "contextInheritance": "full"
  }
}
```

## Context Inheritance Levels

### none
No conversation history passed to agent.
```json
"contextInheritance": "none"
```

### minimal
Only current request and immediate context.
```json
"contextInheritance": "minimal"
```

### partial
Filtered context based on task relevance.
```json
"contextInheritance": "partial"
```

### full
Complete conversation history and file context.
```json
"contextInheritance": "full"
```

## Usage Examples

### Automatic Delegation

```bash
# These requests automatically delegate to appropriate agents:
q chat "Deploy my React app to S3"           # → AWS Agent
q chat "Review this Python code for bugs"    # → Code Agent
q chat "Scan for security vulnerabilities"   # → Security Agent
q chat "Set up CI/CD pipeline"              # → DevOps Agent
```

### Manual Delegation

```bash
# Force delegation to specific agent type
/delegate "aws" "Create an S3 bucket with versioning"

# Delegate with custom context
/delegate "security" "Audit this codebase" --context-level Full
```

### Agent Management

```bash
# View delegation history
/agents active

# Check agent performance
/agents list --with-stats

# Reset agent learning
/agents clear
```

## Troubleshooting

**Problem**: "No suitable agent found for request"
**Solution**: 
- Check agent configuration in `~/.amazonq/agents.json`
- Verify agent keywords match your request
- Try manual delegation with `/delegate`

#### Low Confidence Scores
**Problem**: Requests not auto-delegating
**Solution**:
- Lower confidence threshold in agent config
- Add more specific keywords to agent definitions
- Use manual delegation for edge cases

#### Context Too Large
**Problem**: "Context size exceeds limit"
**Solution**:
- Reduce `max_context_size` in agent config
- Use "Partial" or "Minimal" context inheritance
- Clear conversation history with `/clear`

#### Delegation Failures
**Problem**: Agent returns errors or timeouts
**Solution**:
- Check network connectivity
- Verify agent credentials and permissions
- Review agent logs in `~/.amazonq/logs/`
- Use fallback agents in configuration

### Performance Issues

#### Slow Delegation
- Reduce context inheritance level
- Optimize agent selection criteria
- Clear delegation history periodically

#### High Memory Usage
- Limit max context size per agent
- Use context cleanup mechanisms
- Monitor agent resource usage

### Configuration Validation

```bash
# Validate agent configuration
q config validate-agents

# Test agent connectivity
q agents test-connection

# Debug delegation decisions
q chat --debug-delegation "your request here"
```

### Log Analysis

Agent delegation logs are stored in:
- `~/.amazonq/logs/delegation.log` - Delegation decisions
- `~/.amazonq/logs/agents.log` - Agent execution logs
- `~/.amazonq/logs/context.log` - Context isolation logs

## Best Practices

### Agent Configuration
- Use specific keywords for better matching
- Set appropriate confidence thresholds (0.6-0.8)
- Configure fallback agents for reliability
- Limit context size for performance

### Request Formatting
- Be specific about your intent
- Include relevant keywords for auto-delegation
- Use clear, actionable language
- Provide necessary context upfront

### Performance Optimization
- Use "Partial" context inheritance for most cases
- Clear delegation history regularly
- Monitor agent success rates
- Adjust thresholds based on usage patterns


### Configuration File Format

Agents are configured as JSON files in `~/.aws/amazonq/cli-agents/` following the agent-v1.1.json schema:

```json
{
  "$schema": "https://raw.githubusercontent.com/aws/amazon-q-developer-cli/main/schemas/agent-v1.1.json",
  "name": "code-reviewer",
  "description": "Specialized agent for code review and security analysis",
  "prompt": "You are a code review specialist focused on security, performance, and best practices.",
  "delegation": {
    "keywords": ["review", "analyze", "security", "code", "audit"],
    "taskPatterns": ["review.*code", "check.*security", "analyze.*function"],
    "autoDelegate": true,
    "priority": 8,
    "contextInheritance": "partial"
  }
}
```

```json
{
  "$schema": "https://raw.githubusercontent.com/aws/amazon-q-developer-cli/main/schemas/agent-v1.1.json",
  "name": "aws-specialist",
  "description": "AWS cloud infrastructure and services expert",
  "prompt": "You are an AWS specialist with deep knowledge of cloud services, infrastructure, and best practices.",
  "delegation": {
    "keywords": ["aws", "cloud", "infrastructure", "lambda", "s3", "ec2"],
    "taskPatterns": ["configure.*aws", "setup.*lambda", "create.*bucket"],
    "autoDelegate": true,
    "priority": 9,
    "contextInheritance": "full"
  }
}
```

### Configuration Parameters

- **name**: Unique identifier for the agent
- **delegation.keywords**: Keywords that trigger this agent
- **delegation.taskPatterns**: Regex patterns that match tasks for this agent
- **delegation.priority**: Execution priority (0-10, higher = more priority)
- **delegation.contextInheritance**: How much context to share ("none", "minimal", "partial", "full")
- **delegation.autoDelegate**: Whether agent can be automatically selected

## Usage Examples

### Automatic Delegation

The system automatically delegates based on request content:

```bash
# Code review requests → code-reviewer agent
q chat "Review this Python function for security vulnerabilities"

# AWS requests → aws-specialist agent  
q chat "Help me configure an S3 bucket with proper permissions"

# Documentation requests → documentation-writer agent
q chat "Write API documentation for this REST endpoint"
```

### Manual Delegation Commands

```bash
# Force delegation to specific agent
/delegate code-reviewer "Analyze this code for performance issues"

# List all available agents
/agents list

# View currently active delegations
/agents active

# Clear delegation history
/agents clear
```

### Context Inheritance Levels

- **none**: Agent gets no conversation context
- **minimal**: Agent gets current request only
- **partial**: Agent gets filtered relevant context
- **full**: Agent gets complete conversation history

## Troubleshooting

**Problem**: `/delegate unknown-agent "task"` returns "Agent not found"
**Solution**: 
- Check agent name with `/agents list`
- Verify agent configuration in `~/.config/q/agents.toml`
- Restart Q CLI after configuration changes

#### No Automatic Delegation
**Problem**: Requests not automatically delegating to expected agents
**Solution**:
- Check if request contains agent delegation keywords
- Verify delegation confidence threshold (default: 0.6)
- Ensure delegation.autoDelegate is set to true in agent JSON
- Use `/delegate agent-name "request"` for manual delegation

#### Context Issues
**Problem**: Agent missing important context or getting too much context
**Solution**:
- Adjust `context_inheritance` level in agent configuration
- Use "Partial" for balanced context filtering
- Use "Full" for agents needing complete conversation history

#### Performance Issues
**Problem**: Delegation taking too long (>100ms)
**Solution**:
- Reduce number of active agents
- Use "Minimal" context inheritance for faster agents
- Clear delegation history with `/agents clear`

#### Configuration Not Loading
**Problem**: Agent configuration changes not taking effect
**Solution**:
- Verify file path: `~/.aws/amazonq/cli-agents/agent-name.json`
- Check JSON syntax with online validator
- Restart Q CLI application
- Check file permissions (readable by user)
- Ensure delegation.autoDelegate is set to true

### Debug Commands

```bash
# Check agent status
/agents list

# View delegation history
/agents active

# Test specific agent
/delegate agent-name "test request"

# Clear problematic state
/agents clear
```

### Error Messages

- **"No suitable agent found"**: No agent matches request capabilities
- **"Delegation failed"**: Agent execution error, falling back to main agent
- **"Context isolation failed"**: Context filtering error, using minimal context
- **"Agent timeout"**: Agent took too long, falling back to main agent

### Performance Optimization

- Keep agent count under 10 for optimal performance
- Use appropriate context inheritance levels
- Regularly clear delegation history
- Monitor delegation overhead with performance tests
