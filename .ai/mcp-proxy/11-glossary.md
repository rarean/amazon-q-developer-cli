# Glossary

## Terms and Definitions

| Term | Definition |
|------|------------|
| **ACP** | Agent Client Protocol - Protocol for communication between agents and clients |
| **MCP** | Model Context Protocol - Standard protocol for tool communication |
| **Builtin Tools** | Native tools implemented in chat CLI (fs_read, fs_write, execute_bash, etc.) |
| **Tool Executor** | Interface for executing builtin tools and commands |
| **MCP Proxy** | Server that translates between MCP protocol and builtin tools |
| **Tool Registry** | Component that manages available tools and routes calls |

## Acronyms

| Acronym | Full Form |
|---------|-----------|
| **API** | Application Programming Interface |
| **CLI** | Command Line Interface |
| **HTTP** | Hypertext Transfer Protocol |
| **JSON** | JavaScript Object Notation |
| **RPC** | Remote Procedure Call |
| **SDK** | Software Development Kit |
| **URI** | Uniform Resource Identifier |

## Technical Concepts

| Concept | Description |
|---------|-------------|
| **Dependency Injection** | Pattern where dependencies are provided externally rather than created internally |
| **Proxy Pattern** | Structural pattern that provides a placeholder/surrogate for another object |
| **Protocol Translation** | Converting between different communication protocols |
| **Trait-based Interface** | Rust pattern using traits to define behavior contracts |
| **Async/Await** | Asynchronous programming pattern for non-blocking operations |

## Domain-Specific Terms

| Term | Context | Definition |
|------|---------|------------|
| **Session** | ACP | Isolated conversation context with state |
| **Tool Call** | MCP | Request to execute a specific tool with parameters |
| **Capability** | System | Available functionality that can be exposed |
| **Handler** | Implementation | Function that processes specific tool requests |
| **Transport** | Network | Mechanism for message delivery (HTTP, WebSocket, etc.) |
