# Component Analysis - Claude Code ACP Adapter

## Core Components

### 1. Entry Point (`src/index.ts`)
**Purpose**: Application bootstrap and process management
**Key Features**:
- Redirects console output to stderr (ACP uses stdout)
- Handles unhandled promise rejections
- Keeps process alive for continuous operation
- Minimal, focused responsibility

### 2. ACP Agent (`src/acp-agent.ts`)
**Purpose**: Main adapter implementation bridging ACP and Claude Code SDK
**Key Responsibilities**:
- Session lifecycle management
- Protocol message translation
- Authentication handling
- Terminal management
- Background process coordination

**Key Classes/Types**:
- `Session`: Manages query state and cancellation
- `BackgroundTerminal`: Handles terminal operations
- Agent implementation with ACP protocol handlers

**Critical Methods**:
- `initialize()`: Sets up agent capabilities
- `newSession()`: Creates new conversation sessions
- `prompt()`: Handles user prompts and tool execution
- `readTextFile()` / `writeTextFile()`: File operations

### 3. Tool Handler (`src/tools.ts`)
**Purpose**: Abstracts tool execution and result formatting
**Key Functions**:
- `toolInfoFromToolUse()`: Extracts tool information from Claude responses
- `toolUpdateFromToolResult()`: Formats tool execution results
- `planEntries()`: Manages planning and task breakdown

**Supported Tool Types**:
- Task (thinking/planning)
- NotebookRead/Write (file operations)
- BashRun (command execution)
- EditFile (code modifications)
- CreateFile (file creation)

### 4. MCP Server (`src/mcp-server.ts`)
**Purpose**: Model Context Protocol integration for extensible tools
**Key Features**:
- Dynamic MCP server discovery and management
- Tool registration and execution
- File content caching
- Location tracking for code changes

**Core Functions**:
- `createMcpServer()`: Initializes MCP server with available tools
- `replaceAndCalculateLocation()`: Handles code replacements with location tracking
- Tool execution with proper error handling

### 5. Utilities (`src/utils.ts`)
**Purpose**: Common utilities and type conversions
**Key Utilities**:
- Stream transformations (Node.js to Web API)
- Type guards and assertions
- Pushable stream implementation
- Error handling helpers

## Data Flow Architecture

### Message Flow
```
ACP Client Request
    ↓
ACP Agent (protocol translation)
    ↓
Claude Code SDK (AI processing)
    ↓
Tool Handler (tool execution)
    ↓
MCP Server (if MCP tools used)
    ↓
Tool Results
    ↓
Claude Code SDK (response generation)
    ↓
ACP Agent (protocol translation)
    ↓
ACP Client Response
```

### Session Management
- Sessions maintain state between interactions
- Each session has its own Query instance
- Cancellation support for long-running operations
- Background terminal management for persistent shells

### Tool Execution Pipeline
1. **Tool Detection**: Parse tool use from Claude responses
2. **Tool Info Extraction**: Convert to ACP tool format
3. **Execution**: Run tool via appropriate handler
4. **Result Processing**: Format results for Claude
5. **Location Tracking**: Update file positions for edits

## Integration Points

### External Dependencies
- **@anthropic-ai/claude-code**: Core AI functionality
- **@zed-industries/agent-client-protocol**: ACP protocol implementation
- **@modelcontextprotocol/sdk**: MCP server integration
- **express**: HTTP server for MCP communication
- **diff**: Text difference calculations

### Protocol Compliance
- Full ACP protocol implementation
- Proper error handling and status codes
- Streaming support for real-time responses
- Capability negotiation

### Extensibility
- MCP server plugin architecture
- Tool registration system
- Configurable server discovery
- Custom tool development support

## Performance Considerations

### Optimization Strategies
- File content caching to reduce I/O
- Streaming responses for large outputs
- Efficient diff calculations for code changes
- Background terminal reuse

### Resource Management
- Proper cleanup of sessions and terminals
- Memory-efficient stream handling
- Connection pooling for MCP servers
- Graceful shutdown procedures

## Error Handling Strategy

### Error Categories
1. **Protocol Errors**: ACP message format issues
2. **Authentication Errors**: API key problems
3. **Tool Execution Errors**: Command failures
4. **Network Errors**: Connection issues
5. **File System Errors**: I/O problems

### Recovery Mechanisms
- Graceful degradation when tools fail
- Session isolation to prevent cascading failures
- Retry logic for transient errors
- Clear error reporting to clients
