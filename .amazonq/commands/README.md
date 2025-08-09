# Custom Commands
Custom commands allow you to define frequently-used prompts as Markdown files that Amazon Q can execute. Commands are organized by scope (project-specific or personal) and support namespacing through directory structures.

## Syntax
```
/<prefix>:<command-name>
```
Parameters
Parameter	Description
`<prefix>` Command scope (project for project-specific, user for personal)
`<command-name>` Name derived from the Markdown filename (without .md extension)

## Command types

### Project commands
Commands stored in your repository and shared with your team.

Location: `.amazonq/commands/`
Prefix: `/project:`

In the following example, we create the `/project:optimize` command:

```bash
# Create a project command
mkdir -p .amazonq/commands
echo "Analyze this code for performance issues and suggest optimizations:" > .amazonq/commands/optimize.md
```

### Personal commands
Commands available across all your projects.

Location: `~/.amazonq/commands/`
Prefix: `/user:`

In the following example, we create the `/user:security-review` command:

```bash
# Create a personal command
mkdir -p ~/.amazonq/commands
echo "Review this code for security vulnerabilities:" > ~/.amazonq/commands/security-review.md
```

## Features

### Namespacing
Organize commands in subdirectories to create namespaced commands.

Structure: `<prefix>:<namespace>:<command>`

For example, a file at `.amazonq/commands/frontend/component.md` creates the command `/project:frontend:component`

### File format
Command files support:

- Markdown format (.md extension)
- Simple frontmatter for metadata:
  - `description`: Brief description of the command
- Prompt instructions as the main content

Example command file:
```markdown
---
description: Create a git commit
---

# Git Commit Helper

Based on the current repository state, create a single git commit with an appropriate commit message.

Please analyze the staged changes and write a clear, descriptive commit message following conventional commit format.
```

## Security

Commands are automatically validated for potentially dangerous patterns including:
- Destructive file operations (`rm -rf`)
- Privilege escalation (`sudo`)
- Permission changes (`chmod 777`)
- Path traversal (`../../../`)
- Network access (`curl`, `wget`)

Commands containing these patterns will be blocked from execution.

## Best Practices

- Use descriptive command names
- Keep commands focused on a single task
- Include clear instructions in the command content
- Use namespacing to organize related commands
- Document command purpose in frontmatter description
