# Amazon Q CLI Themes

Amazon Q CLI supports customizable prompt themes that allow you to personalize your command-line experience with git integration, colors, and conditional formatting.

## Quick Start

### List Available Themes
```bash
q theme list
```

### Preview a Theme
```bash
q theme preview powerline
```

### Switch to a Theme
```bash
q theme switch git-enabled
```

### Validate a Theme
```bash
q theme validate my-custom-theme
```

## Built-in Themes

### minimal
A simple, clean prompt:
```
$ 
```

### powerline
Enhanced prompt with symbols and colors:
```
❯ ⎇ main ●±?↑ ❯ 
```

### git-enabled
Comprehensive git status display:
```
➜ git:(main) ● ± ? ↑ ↓ ✓ $ 
```

## Theme Syntax

### Variables

#### Git Variables
- `${GIT_BRANCH}` - Current git branch name
- `${GIT_CLEAN}` - Clean repository indicator (✓)
- `${GIT_STAGED}` - Staged changes indicator (●)
- `${GIT_MODIFIED}` - Modified files indicator (±)
- `${GIT_UNTRACKED}` - Untracked files indicator (?)
- `${GIT_AHEAD}` - Commits ahead indicator (↑)
- `${GIT_BEHIND}` - Commits behind indicator (↓)

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

## Creating Custom Themes

### Theme File Format
Create a `.theme` file in `~/.aws/amazonq/themes/`:

```bash
# Example: ~/.aws/amazonq/themes/my-theme.theme
${GREEN}${BOLD}➜${RESET} ${GIT_BRANCH:+${CYAN}${GIT_BRANCH}${RESET} }${GIT_STAGED}${GIT_MODIFIED}${GIT_UNTRACKED}$ 
```

### Theme Examples

#### Simple Git Theme
```bash
${GIT_BRANCH:+[${GIT_BRANCH}] }$ 
```

#### Colorful Status Theme
```bash
${BLUE}❯${RESET} ${GIT_BRANCH:+${YELLOW}${GIT_BRANCH}${RESET} }${GIT_CLEAN:+${GREEN}✓${RESET} }${GIT_STAGED:+${GREEN}●${RESET} }${GIT_MODIFIED:+${YELLOW}±${RESET} }${GIT_UNTRACKED:+${RED}?${RESET} }
```

#### Minimalist Theme
```bash
${GIT_BRANCH:+${GIT_BRANCH} }${GIT_STAGED}${GIT_MODIFIED}${GIT_UNTRACKED}> 
```

#### Powerline-style Theme
```bash
${BLUE}${BOLD}❯${RESET} ${GIT_BRANCH:+${CYAN}⎇ ${GIT_BRANCH}${RESET} }${GIT_STAGED}${GIT_MODIFIED}${GIT_UNTRACKED}${GIT_AHEAD}${GIT_BEHIND}${GIT_CLEAN:+ }${YELLOW}❯${RESET} 
```

## Git Status Colors

Git status indicators are automatically colored:
- **Clean repository**: Green ✓
- **Staged changes**: Green ●
- **Modified files**: Yellow ±
- **Untracked files**: Red ?
- **Commits ahead**: Cyan ↑
- **Commits behind**: Magenta ↓

## Theme Validation

Themes are validated for:
- Balanced braces `${}`
- Valid variable names
- Proper conditional syntax

Invalid themes will show descriptive error messages:
```bash
q theme validate broken-theme
# Theme 'broken-theme' is invalid: Unbalanced braces in theme template
```

## Best Practices

1. **Keep it readable**: Don't overcrowd your prompt
2. **Use colors wisely**: Too many colors can be distracting
3. **Test in different contexts**: Verify your theme works in various git states
4. **Use conditional formatting**: Only show relevant information
5. **Consider terminal width**: Long prompts can wrap awkwardly

## Troubleshooting

### Theme not found
```bash
q theme list  # Check available themes
```

### Colors not showing
- Ensure your terminal supports ANSI colors
- Check if colors are disabled in terminal settings

### Git status not updating
- Verify you're in a git repository
- Check git repository status with `git status`

## Advanced Usage

### Dynamic Theme Switching
Themes automatically adapt to your current directory's git status. No manual switching required between git and non-git directories.

### Theme Marketplace
Store and share themes in the `~/.aws/amazonq/themes/` directory. Theme files use the `.theme` extension and contain the prompt template.
