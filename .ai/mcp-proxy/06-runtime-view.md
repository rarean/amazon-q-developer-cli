# Runtime View

## Scenario 1: Tool Registration

```mermaid
sequenceDiagram
    participant CLI as Chat CLI
    participant ACP as ACP Agent
    participant Proxy as MCP Proxy
    participant Registry as Tool Registry
    
    CLI->>ACP: Provide BuiltinToolExecutor
    ACP->>Proxy: Create with executor
    Proxy->>Registry: Get capabilities
    Registry->>CLI: Query available tools
    CLI-->>Registry: Return capabilities
    Registry->>Proxy: Register MCP tools
    Proxy->>ACP: Return server address
```

## Scenario 2: Tool Execution

```mermaid
sequenceDiagram
    participant Client as MCP Client
    participant Proxy as MCP Proxy Server
    participant Registry as Tool Registry
    participant Executor as Tool Executor
    participant CLI as Chat CLI Tools
    
    Client->>Proxy: MCP tool call request
    Proxy->>Registry: Route tool call
    Registry->>Executor: Execute tool
    Executor->>CLI: Call builtin implementation
    CLI-->>Executor: Return result
    Executor-->>Registry: Return result
    Registry-->>Proxy: Return MCP response
    Proxy-->>Client: MCP tool result
```

## Scenario 3: Error Handling

```mermaid
sequenceDiagram
    participant Client as MCP Client
    participant Proxy as MCP Proxy
    participant CLI as Chat CLI
    
    Client->>Proxy: Invalid tool request
    Proxy->>CLI: Execute tool
    CLI-->>Proxy: Tool error
    Proxy->>Proxy: Translate error to MCP format
    Proxy-->>Client: MCP error response
```

## Performance Considerations

- **Connection Pooling**: Reuse HTTP connections
- **Async Processing**: Non-blocking tool execution
- **Error Caching**: Cache tool availability checks
- **Minimal Serialization**: Direct parameter passing where possible
