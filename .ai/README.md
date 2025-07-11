# Claude Code ACP Adapter - Architecture Documentation

This directory contains comprehensive architecture documentation for the Claude Code ACP Adapter project, following the arc42 methodology.

## Documentation Structure

### 📋 [arc42-architecture.md](./arc42-architecture.md)
Main architecture documentation following arc42 template:
- System overview and goals
- Architecture constraints and decisions
- Building blocks and runtime views
- Quality requirements and risks

### 🔧 [component-analysis.md](./component-analysis.md)
Detailed component breakdown:
- Core component responsibilities
- Data flow architecture
- Integration points and dependencies
- Performance considerations

### 🚀 [deployment-operations.md](./deployment-operations.md)
Deployment and operational guidance:
- Installation methods
- Configuration options
- Monitoring and troubleshooting
- Security and maintenance

### 📊 [technical-specifications.md](./technical-specifications.md)
Technical specifications and standards:
- System requirements
- Protocol compliance
- API specifications
- Performance and security specs

## Quick Start

For developers new to this project:

1. **Start with**: [arc42-architecture.md](./arc42-architecture.md) for system overview
2. **Deep dive**: [component-analysis.md](./component-analysis.md) for implementation details
3. **Deploy**: [deployment-operations.md](./deployment-operations.md) for setup guidance
4. **Reference**: [technical-specifications.md](./technical-specifications.md) for detailed specs

## Architecture Summary

The Claude Code ACP Adapter is a TypeScript-based bridge that enables ACP-compatible clients (like Zed editor) to interact with Anthropic's Claude Code SDK. It implements the Adapter pattern to translate between protocols while providing:

- **Real-time AI assistance** through Claude Code integration
- **Tool execution** via MCP (Model Context Protocol) servers
- **File operations** with location tracking
- **Terminal management** for command execution
- **Session management** for stateful conversations

## Key Components

```
┌─────────────────────────────────────────┐
│        Claude Code ACP Adapter          │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────────┐   │
│  │ ACP Agent   │  │   Tool Handler  │   │
│  └─────────────┘  └─────────────────┘   │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │ MCP Server  │  │   Utilities     │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
```

## Documentation Methodology

This documentation follows the [arc42 template](https://arc42.org/), a proven approach for software architecture documentation that provides:

- **Standardized structure** for consistent documentation
- **Stakeholder-focused** content organization
- **Practical guidance** for implementation and operations
- **Quality-driven** architecture decisions

## Contributing

When updating this documentation:

1. Follow arc42 structure and numbering
2. Keep technical accuracy with code changes
3. Update cross-references between documents
4. Maintain consistency in terminology and formatting

## Generated

This documentation was generated on: 2025-09-08T20:30:23.587-05:00
