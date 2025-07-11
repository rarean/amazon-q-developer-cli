# Amazon Q CLI ACP Integration - Implementation Plan (arc42)

## 1. Introduction and Goals

### 1.1 Requirements Overview
Integrate Agent Client Protocol (ACP) support into the Amazon Q CLI to enable communication with ACP-compatible clients (like Zed editor) while leveraging existing AWS service capabilities instead of external APIs.

### 1.2 Quality Goals
- **Compatibility**: Full ACP protocol compliance for seamless client integration
- **Performance**: Low-latency communication leveraging existing AWS streaming clients
- **Maintainability**: Reuse existing crate architecture and patterns
- **Security**: Leverage existing AWS authentication and authorization mechanisms

### 1.3 Stakeholders
- **Amazon Q CLI Users**: Developers using ACP-compatible editors
- **AWS Customers**: Existing Q CLI users wanting ACP integration
- **Development Team**: Maintainers of Amazon Q CLI codebase

## 2. Architecture Constraints

### 2.1 Technical Constraints
- Must integrate with existing Rust codebase
- Leverage existing AWS service clients (CodeWhisperer, Q Developer, Consolas)
- Maintain compatibility with current CLI architecture
- Use existing authentication and telemetry systems

### 2.2 Organizational Constraints
- Follow existing code patterns and conventions
- Maintain MIT/Apache 2.0 licensing
- Use existing CI/CD and testing infrastructure

## 3. System Scope and Context

### 3.1 Business Context
```
[ACP Client] <--ACP--> [Q CLI ACP Agent] <--AWS APIs--> [AWS Services]
                              |
                              v
                        [Existing Q CLI Crates]
```

### 3.2 Technical Context
- **Input**: ACP protocol messages via stdio
- **Processing**: Leverage existing AWS service clients
- **Output**: AWS API responses translated to ACP format
- **Integration**: Reuse chat-cli, mcp_client, and AWS service crates

## 4. Solution Strategy

### 4.1 Architecture Pattern
**Adapter + Facade Pattern**: 
- ACP Agent acts as adapter between ACP protocol and existing AWS clients
- Facade pattern to unify access to multiple AWS service clients

### 4.2 Key Design Decisions
- Extend existing CLI with new `acp` subcommand
- Reuse existing AWS service clients instead of external APIs
- Integrate with existing MCP client for tool execution
- Leverage existing authentication and session management

## 5. Building Block View

### 5.1 Level 1 - System Overview
```
┌─────────────────────────────────────────────────────────┐
│                Amazon Q CLI with ACP                    │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ ACP Agent   │  │   AWS Clients   │  │ MCP Client  │ │
│  │ (New)       │  │   (Existing)    │  │ (Existing)  │ │
│  └─────────────┘  └─────────────────┘  └─────────────┘ │
│  ┌─────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ Auth System │  │   Telemetry     │  │ Database    │ │
│  │ (Existing)  │  │   (Existing)    │  │ (Existing)  │ │
│  └─────────────┘  └─────────────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 5.2 Level 2 - Component Details

#### ACP Agent (`crates/acp/src/agent.rs`)
- **Responsibility**: ACP protocol implementation and AWS service orchestration
- **Interfaces**: ACP protocol handlers, AWS client integration
- **Key Functions**:
  - Session lifecycle management
  - Message translation between ACP and AWS APIs
  - Tool execution coordination

#### AWS Service Facade (`crates/acp/src/aws_facade.rs`)
- **Responsibility**: Unified interface to existing AWS service clients
- **Interfaces**: CodeWhisperer, Q Developer, Consolas clients
- **Key Functions**:
  - Service selection based on request type
  - Response aggregation and formatting
  - Streaming response handling

#### Tool Integration (`crates/acp/src/tools.rs`)
- **Responsibility**: Bridge ACP tool execution with existing MCP client
- **Interfaces**: MCP client, file system operations
- **Key Functions**:
  - Tool discovery and registration
  - Execution delegation to MCP client
  - Result formatting for ACP responses

## 6. Runtime View

### 6.1 ACP Session Initialization
```
ACP Client -> ACP Agent: InitializeRequest
ACP Agent -> Auth System: Validate credentials
Auth System -> ACP Agent: Auth success
ACP Agent -> AWS Clients: Initialize connections
AWS Clients -> ACP Agent: Ready
ACP Agent -> ACP Client: InitializeResponse
```

### 6.2 Prompt Processing Flow
```
ACP Client -> ACP Agent: PromptRequest
ACP Agent -> AWS Facade: Route to appropriate service
AWS Facade -> Q Developer Client: Stream request
Q Developer Client -> AWS Facade: Streaming response
AWS Facade -> Tool Integration: Extract tool calls
Tool Integration -> MCP Client: Execute tools
MCP Client -> Tool Integration: Tool results
Tool Integration -> AWS Facade: Formatted results
AWS Facade -> ACP Agent: Complete response
ACP Agent -> ACP Client: PromptResponse
```

## 7. Deployment View

### 7.1 Integration with Existing CLI
- New `q acp` subcommand added to existing CLI
- Reuses existing authentication and configuration
- Shares database and telemetry infrastructure

### 7.2 Client Integration
- ACP clients connect via stdio to `q acp` command
- No additional installation required beyond Q CLI
- Leverages existing AWS credentials and configuration

## 8. Cross-cutting Concepts

### 8.1 Authentication
- Reuse existing AWS authentication mechanisms
- Leverage existing login/logout commands
- Share session state with main CLI

### 8.2 Error Handling
- Translate AWS service errors to ACP error format
- Maintain existing error logging and telemetry
- Graceful degradation when services unavailable

### 8.3 Performance
- Leverage existing streaming clients for real-time responses
- Reuse connection pools and caching mechanisms
- Efficient message serialization using existing patterns

## 9. Architecture Decisions

### 9.1 ADR-001: Reuse Existing AWS Clients
**Status**: Accepted
**Context**: Need to integrate with AWS services for AI capabilities
**Decision**: Leverage existing CodeWhisperer, Q Developer, and Consolas clients
**Consequences**: Faster development, consistent behavior, reduced maintenance

### 9.2 ADR-002: Extend CLI with ACP Subcommand
**Status**: Accepted
**Context**: Need to integrate ACP without disrupting existing functionality
**Decision**: Add new `acp` subcommand to existing CLI structure
**Consequences**: Clean separation, reuse of infrastructure, familiar patterns

### 9.3 ADR-003: Bridge MCP Client for Tools
**Status**: Accepted
**Context**: Need tool execution capabilities for ACP
**Decision**: Integrate existing MCP client for tool execution
**Consequences**: Reuse existing tool ecosystem, consistent behavior

## 10. Quality Requirements

### 10.1 Performance
- Protocol translation: < 5ms overhead
- Streaming responses: Real-time with existing AWS client performance
- Memory usage: Minimal overhead over existing CLI

### 10.2 Reliability
- Error recovery: Leverage existing AWS client retry mechanisms
- Session management: Reuse existing database and state management
- Availability: Same as existing Q CLI availability

### 10.3 Maintainability
- Code reuse: Maximize use of existing crates and patterns
- Testing: Integrate with existing test infrastructure
- Documentation: Follow existing documentation standards

## 11. Implementation Phases

### 11.1 Phase 1: Core ACP Agent (Week 1-2)
- Implement basic ACP protocol handlers
- Create AWS service facade
- Add `acp` subcommand to CLI

### 11.2 Phase 2: AWS Service Integration (Week 3-4)
- Integrate with existing AWS clients
- Implement streaming response handling
- Add session management

### 11.3 Phase 3: Tool Integration (Week 5-6)
- Bridge with existing MCP client
- Implement tool discovery and execution
- Add file operation support

### 11.4 Phase 4: Testing and Polish (Week 7-8)
- Comprehensive testing with ACP clients
- Performance optimization
- Documentation and examples

## 12. Risk Assessment

### 12.1 Technical Risks
- ACP protocol compatibility issues
- Performance overhead from protocol translation
- Integration complexity with existing crates

### 12.2 Mitigation Strategies
- Early prototype with reference ACP implementation
- Performance benchmarking against existing CLI
- Incremental integration with thorough testing

## 13. Success Criteria

### 13.1 Functional Requirements
- Full ACP protocol compliance
- Seamless integration with Zed editor
- All existing AWS service capabilities available via ACP

### 13.2 Non-Functional Requirements
- Performance within 10% of direct CLI usage
- Zero impact on existing CLI functionality
- Maintainable code following existing patterns
