# MCP Proxy Implementation Plan

This directory contains the Arc42 architecture documentation for implementing an MCP Server Proxy in the Rust ACP crate, based on analysis of the Claude Code ACP TypeScript implementation.

## Overview

The MCP Proxy enables ACP agents to expose chat-cli built-in tools through the standard Model Context Protocol (MCP), allowing external systems to use tools like `fs_read`, `fs_write`, `execute_bash`, and CLI commands like `/compact`.

## Architecture Documents

1. **[Introduction and Goals](01-introduction-and-goals.md)** - Requirements and stakeholders
2. **[Architecture Constraints](02-architecture-constraints.md)** - Technical and organizational constraints  
3. **[System Scope and Context](03-system-scope-and-context.md)** - Business and technical context
4. **[Solution Strategy](04-solution-strategy.md)** - Key technology decisions and patterns
5. **[Building Block View](05-building-block-view.md)** - Component structure and interfaces
6. **[Runtime View](06-runtime-view.md)** - Execution scenarios and flows
7. **[Deployment View](07-deployment-view.md)** - Infrastructure and deployment model
8. **[Cross-cutting Concepts](08-concepts.md)** - Domain model and common patterns
9. **[Quality Requirements](09-quality-requirements.md)** - Performance, reliability, security
10. **[Risks and Technical Debt](10-risks-and-technical-debt.md)** - Known issues and future work
11. **[Glossary](11-glossary.md)** - Terms and definitions

## Key Implementation Pattern

Based on the Claude Code ACP analysis, the solution uses:

1. **Internal MCP Server**: Creates an HTTP MCP server within the ACP agent process
2. **Trait-based Interface**: `BuiltinToolExecutor` trait for dependency injection
3. **Conditional Registration**: Only expose tools that are available via capabilities
4. **Tool Proxying**: MCP tools delegate to actual builtin implementations
5. **Error Translation**: Convert between tool errors and MCP protocol errors

## Next Steps

1. Review architecture documents
2. Implement `BuiltinToolExecutor` trait in ACP crate
3. Create MCP proxy server implementation
4. Add tool registry and handlers
5. Integrate with existing ACP agent
6. Add comprehensive tests

## References

- [Arc42 Template](https://arc42.org/)
- [Claude Code ACP Implementation](../claude-code-acp/)
- [Model Context Protocol Specification](https://modelcontextprotocol.io/)
- [ACP Crate Documentation](../../crates/acp/)
