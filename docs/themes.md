# Amazon Q CLI Themes

Amazon Q CLI supports customizable prompt themes that allow you to personalize your chat session experience with git integration, colors, and conditional formatting.

## Enabling Themes

Themes are an experimental feature that must be enabled first:

1. In a chat session, run `/experiment`
2. Select "Themes" from the list to toggle it on
3. Use `/themes` commands to manage themes

## Quick Start

### List Available Themes
```bash
/themes list
```

### Preview a Theme (validates and shows rendered output)
```bash
/themes preview powerline
```

### Switch to a Theme
```bash
/themes switch git-enabled
```

### Show Current Theme
```bash
/themes current
```

### Interactive Theme Selection
```bash
/themes select
```

## Built-in Themes

### minimal (default)
A simple, clean prompt:
```
>
```

### powerline
Enhanced prompt with agent, token usage, and git information in colored segments:
```
 # ${AGENT} > ${TOKEN_USAGE} > ${GIT_BRANCH}
 default > (25.50%) > main
```

### git-enabled
Comprehensive prompt with model, token usage, current directory, and git status:
```
# ${MODEL}:(${TOKEN_USAGE}) ${PWD}:(${GIT_BRANCH})
➜ claude-3-5-sonnet:(25.50%) ~/projects/my-repo:(main) >
```

## Theme Syntax

### Variables

#### Chat Session Variables
- `${AGENT}` - Current agent name (from Q_AGENT environment variable)
- `${MODEL}` - Current model name (from Q_MODEL environment variable)
- `${TOKEN_USAGE}` - Token usage percentage (from Q_TOKEN_USAGE environment variable)
- `${PWD}` - Current working directory (with ~ substitution for home directory)

#### Git Variables
- `${GIT_BRANCH}` - Current git branch name

#### Color Variables
- `${RED}` - Red color
- `${GREEN}` - Green color
- `${YELLOW}` - Yellow color
- `${BLUE}` - Blue color
- `${MAGENTA}` - Magenta color
- `${CYAN}` - Cyan color
- `${BOLD}` - Bold text
- `${RESET}` - Reset formatting

### Conditional Formatting

Use `${VAR:+value_if_set}` syntax to display content only when a variable has a value:

```bash
# Show branch only if in git repository
${GIT_BRANCH:+git:(${GIT_BRANCH}) }

# Show git status indicators only if they exist
${GIT_STAGED:+${GIT_STAGED} }${GIT_MODIFIED:+${GIT_MODIFIED} }

# Combine with colors
${GIT_BRANCH:+${BLUE}git:(${GIT_BRANCH})${RESET} }
```

## Theme Validation

Themes are automatically validated when previewed with `/themes preview`. Validation checks for:
- Balanced braces `${}`
- Valid variable names
- Proper conditional syntax

Invalid themes will show descriptive error messages during preview.

## Best Practices

1. **Keep it readable**: Don't overcrowd your prompt
2. **Use colors wisely**: Too many colors can be distracting
3. **Test in different contexts**: Verify your theme works in various git states
4. **Use conditional formatting**: Only show relevant information
5. **Consider terminal width**: Long prompts can wrap awkwardly

## Troubleshooting

### Themes feature not available
```bash
/experiment  # Enable "Themes" experiment first
```

### Theme not found
```bash
/themes list  # Check available themes (only builtin themes are supported)
```

### Colors not showing
- Ensure your terminal supports ANSI colors
- Check if colors are disabled in terminal settings

### Git status not updating
- Verify you're in a git repository
- Check git repository status with `git status`

## Current Limitations

- Only builtin themes are supported (minimal, powerline, git-enabled)
- Custom theme files are not yet supported
