# System Scope and Context

## Business Context

```mermaid
graph TB
    User[Chat CLI User] --> CLI[Chat CLI]
    CLI --> ACP[ACP Agent]
    ACP --> Proxy[MCP Proxy Server]
    Proxy --> Tools[Built-in Tools]
    
    External[External MCP Client] --> Proxy
    
    Tools --> FS[File System]
    Tools --> Terminal[Terminal]
    Tools --> Commands[CLI Commands]
```

## Technical Context

```mermaid
graph TB
    subgraph "Chat CLI Crate"
        BuiltinTools[Built-in Tools]
        Commands[CLI Commands]
        ToolExecutor[Tool Executor Trait]
    end
    
    subgraph "ACP Crate"
        Agent[ACP Agent]
        McpProxy[MCP Proxy Server]
        ProxyInterface[Proxy Interface]
    end
    
    subgraph "External"
        McpClient[MCP Client]
        McpProtocol[MCP Protocol]
    end
    
    ToolExecutor --> ProxyInterface
    ProxyInterface --> McpProxy
    McpProxy --> McpProtocol
    McpProtocol --> McpClient
    
    Agent --> McpProxy
    BuiltinTools --> ToolExecutor
    Commands --> ToolExecutor
```

## Interfaces

| Interface | Partner | Input | Output |
|-----------|---------|-------|--------|
| **Tool Executor** | Chat CLI | Tool name, parameters | Tool result |
| **MCP Server** | External clients | MCP requests | MCP responses |
| **Proxy Interface** | ACP Agent | Tool execution requests | Proxied results |
