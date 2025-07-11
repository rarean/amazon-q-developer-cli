# Claude Code ACP Adapter - Architecture Documentation (arc42)

## 1. Introduction and Goals

### 1.1 Requirements Overview
The Claude Code ACP Adapter enables integration between Anthropic's Claude Code SDK and ACP (Agent Client Protocol) compatible clients such as Zed editor. It provides a bridge that translates ACP protocol messages to Claude Code SDK calls and vice versa.

### 1.2 Quality Goals
- **Compatibility**: Seamless integration with ACP-compatible clients
- **Reliability**: Stable message translation between protocols
- **Performance**: Low-latency communication for real-time coding assistance
- **Extensibility**: Support for additional tools and capabilities

### 1.3 Stakeholders
- **Developers**: Using Zed or other ACP clients for AI-assisted coding
- **Zed Industries**: Primary client integration
- **Anthropic**: Claude Code SDK provider
- **Open Source Community**: Contributors and maintainers

## 2. Architecture Constraints

### 2.1 Technical Constraints
- Node.js runtime environment
- TypeScript implementation
- ACP protocol compliance
- Claude Code SDK compatibility
- npm package distribution

### 2.2 Organizational Constraints
- Apache 2.0 license
- Open source development model
- Zed Industries maintenance

## 3. System Scope and Context

### 3.1 Business Context
```
[ACP Client (Zed)] <--ACP--> [Claude Code ACP Adapter] <--SDK--> [Claude Code API]
                                        |
                                        v
                                   [MCP Servers]
```

### 3.2 Technical Context
- **Input**: ACP protocol messages from clients
- **Output**: Claude Code SDK API calls
- **Dependencies**: 
  - @anthropic-ai/claude-code
  - @zed-industries/agent-client-protocol
  - @modelcontextprotocol/sdk

## 4. Solution Strategy

### 4.1 Architecture Pattern
**Adapter Pattern**: Translates between ACP protocol and Claude Code SDK interfaces

### 4.2 Key Design Decisions
- Event-driven architecture for real-time communication
- Stateful session management
- Tool abstraction layer for extensibility
- MCP server integration for enhanced capabilities

## 5. Building Block View

### 5.1 Level 1 - System Overview
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

### 5.2 Level 2 - Component Details

#### ACP Agent (`acp-agent.ts`)
- **Responsibility**: Main adapter logic, session management
- **Interfaces**: ACP protocol handlers, Claude Code SDK integration
- **Key Functions**: 
  - Session lifecycle management
  - Message translation
  - Authentication handling

#### Tool Handler (`tools.ts`)
- **Responsibility**: Tool abstraction and execution
- **Interfaces**: Tool use parsing, result formatting
- **Key Functions**:
  - Tool information extraction
  - Location tracking
  - Content formatting

#### MCP Server (`mcp-server.ts`)
- **Responsibility**: Model Context Protocol integration
- **Interfaces**: MCP server management, tool registration
- **Key Functions**:
  - Server lifecycle management
  - Tool discovery and execution
  - Context management

#### Utilities (`utils.ts`)
- **Responsibility**: Common utilities and helpers
- **Interfaces**: Stream handling, type conversions
- **Key Functions**:
  - Stream transformations
  - Type utilities
  - Error handling

## 6. Runtime View

### 6.1 Session Initialization
```
Client -> ACP Agent: InitializeRequest
ACP Agent -> Claude Code: Initialize SDK
Claude Code -> ACP Agent: Ready
ACP Agent -> Client: InitializeResponse
```

### 6.2 Tool Execution Flow
```
Client -> ACP Agent: PromptRequest
ACP Agent -> Claude Code: Query with tools
Claude Code -> Tool Handler: Tool use
Tool Handler -> MCP Server: Execute tool
MCP Server -> Tool Handler: Result
Tool Handler -> Claude Code: Tool result
Claude Code -> ACP Agent: Response
ACP Agent -> Client: PromptResponse
```

## 7. Deployment View

### 7.1 Installation
- npm package: `@zed-industries/claude-code-acp`
- Binary: `claude-code-acp`
- Environment: Node.js runtime

### 7.2 Configuration
- Environment variables: `ANTHROPIC_API_KEY`
- Command line arguments via minimist
- ACP client configuration

## 8. Cross-cutting Concepts

### 8.1 Error Handling
- Structured error responses
- Graceful degradation
- Logging to stderr (stdout reserved for ACP)

### 8.2 Security
- API key management
- Input validation
- Secure communication channels

### 8.3 Performance
- Streaming responses
- Efficient message serialization
- Resource cleanup

## 9. Architecture Decisions

### 9.1 ADR-001: TypeScript Implementation
**Status**: Accepted
**Context**: Need for type safety and maintainability
**Decision**: Use TypeScript for all implementation
**Consequences**: Better developer experience, compile-time error checking

### 9.2 ADR-002: Adapter Pattern
**Status**: Accepted
**Context**: Need to bridge two different protocols
**Decision**: Implement adapter pattern for protocol translation
**Consequences**: Clean separation of concerns, easier testing

### 9.3 ADR-003: MCP Integration
**Status**: Accepted
**Context**: Need for extensible tool ecosystem
**Decision**: Integrate Model Context Protocol for tool management
**Consequences**: Enhanced capabilities, community tool support

## 10. Quality Requirements

### 10.1 Performance
- Response time: < 100ms for protocol translation
- Throughput: Support concurrent sessions
- Memory usage: Efficient resource management

### 10.2 Reliability
- Error recovery: Graceful handling of network issues
- Stability: No memory leaks or resource exhaustion
- Availability: 99.9% uptime for long-running sessions

### 10.3 Usability
- Easy installation via npm
- Clear error messages
- Comprehensive documentation

## 11. Risks and Technical Debts

### 11.1 Technical Risks
- Protocol version compatibility
- API rate limiting
- Network connectivity issues

### 11.2 Technical Debt
- Limited test coverage for integration scenarios
- Hardcoded configuration values
- Missing comprehensive error handling

## 12. Glossary

- **ACP**: Agent Client Protocol - Standard for AI agent communication
- **Claude Code**: Anthropic's coding assistant SDK
- **MCP**: Model Context Protocol - Standard for AI model context management
- **Tool**: Executable function available to the AI agent
- **Session**: Stateful conversation between client and agent
