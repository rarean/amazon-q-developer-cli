# Agent Chaining Architecture Analysis
*Arc42 Architectural Documentation*

## 1. Introduction and Goals

### 1.1 Requirements Overview
Enable Amazon Q CLI to support agent chaining similar to Claude's sub-agent system, allowing:
- Automatic delegation of tasks to specialized agents
- Natural language invocation of specific agents
- Context isolation between chained agents
- Seamless integration with existing JSON-based agent configuration

### 1.2 Quality Goals
| Priority | Quality Goal | Scenario |
|----------|-------------|----------|
| 1 | **Backward Compatibility** | Existing agent configurations continue to work without modification |
| 2 | **Performance** | Agent chaining adds minimal latency to conversation flow |
| 3 | **Usability** | Users can chain agents through natural language commands |
| 4 | **Maintainability** | Architecture supports future enhancements to agent capabilities |

### 1.3 Stakeholders
- **End Users**: Developers using Q CLI for various tasks
- **Agent Authors**: Users creating specialized agent configurations
- **Q CLI Maintainers**: Core development team

## 2. Architecture Constraints

### 2.1 Technical Constraints
- Must integrate with existing Rust codebase
- Preserve current JSON agent configuration format
- Maintain compatibility with MCP server integration
- Support existing tool management system

### 2.2 Organizational Constraints
- Minimal breaking changes to public APIs
- Leverage existing conversation and context management
- Reuse current agent loading mechanisms

## 3. System Scope and Context

### 3.1 Business Context
```
[User] --natural language--> [Q CLI] --delegates--> [Specialized Agents]
                                |
                                v
                         [Tool Execution]
```

### 3.2 Technical Context
- **Current State**: Single agent per chat session
- **Target State**: Dynamic agent delegation within sessions
- **Integration Points**: Tool Manager, Context Manager, Conversation State

## 4. Solution Strategy

### 4.1 Core Approach
Extend the existing agent system with a **delegation layer** that:
1. Analyzes user requests for agent-specific keywords
2. Maintains an **agent registry** for automatic selection
3. Creates **isolated contexts** for delegated tasks
4. Merges results back into the main conversation

### 4.2 Key Design Decisions
- **Hybrid Model**: Combine explicit invocation with automatic delegation
- **Context Isolation**: Each delegated agent operates in a separate context bubble
- **Result Integration**: Seamlessly merge sub-agent outputs into main conversation
- **Configuration Extension**: Add delegation metadata to existing JSON schema

## 5. Building Block View

### 5.1 Level 1: System Overview
```
┌─────────────────────────────────────────────────────────────┐
│                    Q CLI Chat System                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Main Agent      │  │ Agent Delegator │  │ Sub-Agent Pool  │ │
│  │ (Primary)       │  │                 │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│           │                     │                     │        │
│           └─────────────────────┼─────────────────────┘        │
│                                 │                              │
│  ┌─────────────────────────────────────────────────────────────┤
│  │              Context & Tool Management                      │
│  └─────────────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Level 2: Agent Delegation Components

#### 5.2.1 Agent Delegator
**Responsibility**: Analyze requests and route to appropriate agents
```rust
pub struct AgentDelegator {
    registry: AgentRegistry,
    context_manager: ContextManager,
    delegation_rules: Vec<DelegationRule>,
}
```

#### 5.2.2 Agent Registry
**Responsibility**: Maintain available agents and their capabilities
```rust
pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
    delegation_metadata: HashMap<String, DelegationMetadata>,
}

pub struct DelegationMetadata {
    keywords: Vec<String>,
    task_patterns: Vec<Regex>,
    priority: u8,
    auto_delegate: bool,
}
```

#### 5.2.3 Context Isolator
**Responsibility**: Create isolated execution contexts for sub-agents
```rust
pub struct ContextIsolator {
    main_context: ConversationContext,
    sub_contexts: HashMap<String, SubAgentContext>,
}
```

## 6. Runtime View

### 6.1 Agent Delegation Flow
```
User Input → Request Analysis → Agent Selection → Context Creation → 
Task Execution → Result Integration → Response to User
```

### 6.2 Detailed Sequence
1. **Request Analysis**: Parse user input for delegation keywords
2. **Agent Matching**: Find best-fit agent based on task description
3. **Context Isolation**: Create sub-context with relevant history
4. **Task Execution**: Run delegated agent with isolated context
5. **Result Merging**: Integrate sub-agent output into main conversation
6. **State Update**: Update main conversation state with results

## 7. Deployment View

### 7.1 Configuration Structure
```
.amazonq/cli-agents/
├── main-agent.json          # Primary agent
├── code-reviewer.json       # Specialized sub-agent
├── aws-expert.json         # Specialized sub-agent
└── delegation-rules.json   # Delegation configuration
```

### 7.2 Enhanced Agent Schema
```json
{
  "name": "code-reviewer",
  "description": "Expert code review specialist",
  "delegation": {
    "keywords": ["review", "code quality", "security"],
    "taskPatterns": ["review.*code", "check.*security"],
    "autoDelegate": true,
    "priority": 8
  },
  "tools": ["fs_read", "execute_bash"],
  "contextInheritance": "minimal"
}
```

## 8. Cross-cutting Concepts

### 8.1 Context Management
- **Inheritance Levels**: None, Minimal, Partial, Full
- **Context Boundaries**: Clear separation between main and sub-agent contexts
- **State Synchronization**: Controlled merging of context changes

### 8.2 Error Handling
- **Delegation Failures**: Fallback to main agent
- **Context Conflicts**: Resolution strategies for conflicting states
- **Tool Access**: Inherited vs. restricted tool permissions

### 8.3 Performance Optimization
- **Lazy Loading**: Load sub-agents only when needed
- **Context Caching**: Reuse contexts for repeated delegations
- **Parallel Execution**: Support concurrent sub-agent operations

## 9. Architecture Decisions

### 9.1 ADR-001: Hybrid Delegation Model
**Status**: Proposed
**Decision**: Implement both explicit and automatic agent delegation
**Rationale**: Provides flexibility while maintaining user control

### 9.2 ADR-002: Context Isolation Strategy
**Status**: Proposed  
**Decision**: Use separate context bubbles with controlled inheritance
**Rationale**: Prevents context pollution while enabling necessary information sharing

### 9.3 ADR-003: Configuration Backward Compatibility
**Status**: Proposed
**Decision**: Extend existing JSON schema rather than replace it
**Rationale**: Ensures existing agent configurations continue to work

## 10. Quality Requirements

### 10.1 Performance
- Agent delegation overhead < 100ms
- Context creation time < 50ms
- Memory usage increase < 20% per active sub-agent

### 10.2 Reliability
- Graceful degradation when sub-agents fail
- Main conversation continues if delegation fails
- No data loss during context switching

### 10.3 Usability
- Natural language delegation commands
- Clear indication of active agent
- Transparent result integration

## 11. Risks and Technical Debt

### 11.1 Technical Risks
| Risk | Impact | Mitigation |
|------|--------|------------|
| Context complexity | High | Implement clear isolation boundaries |
| Performance degradation | Medium | Lazy loading and caching strategies |
| Configuration complexity | Medium | Provide sensible defaults and examples |

### 11.2 Technical Debt
- Current single-agent architecture needs refactoring
- Tool permission system may need enhancement
- Context management requires significant changes

## 12. Glossary

- **Agent Delegation**: Process of routing tasks to specialized agents
- **Context Isolation**: Separate execution environment for sub-agents
- **Delegation Metadata**: Configuration defining when to use specific agents
- **Sub-Agent**: Specialized agent invoked for specific tasks
- **Context Inheritance**: Level of information passed from main to sub-agent context
