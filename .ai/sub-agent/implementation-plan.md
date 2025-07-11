# Agent Chaining Implementation Plan

## Phase 1: Foundation (Weeks 1-2)

### 1.1 Schema Extension
**Goal**: Extend existing agent JSON schema to support delegation metadata

**Tasks**:
- Add `delegation` field to agent schema in `schemas/agent-v1.json`
- Update `Agent` struct in `crates/chat-cli/src/cli/agent/mod.rs`
- Add delegation-specific types and validation

**Code Changes**:
```rust
// In Agent struct
#[serde(default)]
pub delegation: Option<DelegationConfig>,

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DelegationConfig {
    pub keywords: Vec<String>,
    pub task_patterns: Vec<String>,
    pub auto_delegate: bool,
    pub priority: u8,
    pub context_inheritance: ContextInheritanceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ContextInheritanceLevel {
    None,
    Minimal,
    Partial,
    Full,
}
```

### 1.2 Agent Registry
**Goal**: Create centralized registry for managing available agents

**Tasks**:
- Implement `AgentRegistry` struct
- Add agent discovery and loading mechanisms
- Create delegation metadata indexing

**Code Changes**:
```rust
// New file: crates/chat-cli/src/cli/agent/registry.rs
pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
    delegation_index: HashMap<String, Vec<String>>, // keyword -> agent names
    pattern_index: Vec<(Regex, String)>, // pattern -> agent name
}

impl AgentRegistry {
    pub async fn load_from_directories(os: &Os, paths: &[PathBuf]) -> Result<Self>;
    pub fn find_candidates(&self, input: &str) -> Vec<AgentCandidate>;
    pub fn get_agent(&self, name: &str) -> Option<&Agent>;
}
```

## Phase 2: Core Delegation (Weeks 3-4)

### 2.1 Request Analysis
**Goal**: Implement natural language analysis for agent delegation

**Tasks**:
- Create request analyzer to extract delegation intent
- Implement keyword matching and pattern recognition
- Add explicit agent invocation parsing (e.g., "use code-reviewer agent")

**Code Changes**:
```rust
// New file: crates/chat-cli/src/cli/chat/delegation/analyzer.rs
pub struct RequestAnalyzer {
    registry: Arc<AgentRegistry>,
}

impl RequestAnalyzer {
    pub fn analyze(&self, input: &str) -> DelegationIntent;
    pub fn extract_explicit_agent(&self, input: &str) -> Option<String>;
    pub fn score_agents(&self, input: &str) -> Vec<(String, f32)>;
}

pub enum DelegationIntent {
    ExplicitAgent(String),
    AutoDelegate(Vec<AgentCandidate>),
    NoDelegate,
}
```

### 2.2 Agent Delegator
**Goal**: Core delegation orchestration component

**Tasks**:
- Implement main delegation logic
- Add agent selection algorithms
- Create delegation decision engine

**Code Changes**:
```rust
// New file: crates/chat-cli/src/cli/chat/delegation/delegator.rs
pub struct AgentDelegator {
    registry: Arc<AgentRegistry>,
    analyzer: RequestAnalyzer,
    context_isolator: ContextIsolator,
}

impl AgentDelegator {
    pub async fn delegate(&mut self, 
        input: &str, 
        main_context: &ConversationContext
    ) -> Result<DelegationResult>;
    
    pub fn should_delegate(&self, input: &str) -> bool;
}
```

## Phase 3: Context Isolation (Weeks 5-6)

### 3.1 Context Isolator
**Goal**: Implement isolated execution contexts for sub-agents

**Tasks**:
- Create context isolation mechanisms
- Implement context inheritance levels
- Add context merging strategies

**Code Changes**:
```rust
// New file: crates/chat-cli/src/cli/chat/delegation/context.rs
pub struct ContextIsolator {
    main_context: Arc<ConversationContext>,
    active_sub_contexts: HashMap<String, SubAgentContext>,
}

pub struct SubAgentContext {
    agent_name: String,
    isolated_history: Vec<Message>,
    inherited_context: Vec<ContextItem>,
    tool_results: Vec<ToolResult>,
}

impl ContextIsolator {
    pub fn create_sub_context(&mut self, 
        agent: &Agent, 
        task: &str
    ) -> Result<String>; // returns context_id
    
    pub fn merge_results(&mut self, 
        context_id: &str, 
        results: SubAgentResults
    ) -> Result<()>;
}
```

### 3.2 Context Inheritance
**Goal**: Implement different levels of context sharing

**Tasks**:
- Define inheritance strategies (None, Minimal, Partial, Full)
- Implement context filtering and selection
- Add context size management

**Implementation Strategy**:
- **None**: Empty context, only task description
- **Minimal**: Last 2-3 messages + current task
- **Partial**: Relevant messages based on keyword matching
- **Full**: Complete conversation history (with size limits)

## Phase 4: Integration (Weeks 7-8)

### 4.1 Chat System Integration
**Goal**: Integrate delegation into existing chat flow

**Tasks**:
- Modify main chat loop to support delegation
- Update conversation state management
- Add delegation status indicators

**Code Changes**:
```rust
// In crates/chat-cli/src/cli/chat/mod.rs
impl ChatSession {
    async fn process_user_input(&mut self, input: &str) -> Result<()> {
        // Check for delegation intent
        if let Some(delegation_result) = self.delegator.try_delegate(input).await? {
            self.handle_delegation(delegation_result).await?;
        } else {
            // Existing main agent processing
            self.process_with_main_agent(input).await?;
        }
    }
    
    async fn handle_delegation(&mut self, result: DelegationResult) -> Result<()>;
}
```

### 4.2 Tool Management Integration
**Goal**: Ensure proper tool access for delegated agents

**Tasks**:
- Extend tool manager to support multiple active agents
- Implement tool permission inheritance
- Add tool result aggregation

**Code Changes**:
```rust
// In crates/chat-cli/src/cli/chat/tool_manager.rs
impl ToolManager {
    pub fn create_sub_manager(&self, agent: &Agent) -> Result<ToolManager>;
    pub fn merge_tool_results(&mut self, sub_results: Vec<ToolResult>) -> Result<()>;
}
```

## Phase 5: User Experience (Weeks 9-10)

### 5.1 Slash Commands
**Goal**: Add explicit delegation commands

**Tasks**:
- Implement `/delegate <agent> <task>` command
- Add `/agents list` command to show available agents
- Create `/agents status` to show active delegations

**Code Changes**:
```rust
// In crates/chat-cli/src/cli/chat/cli/mod.rs
#[derive(Debug, Clone)]
pub enum SlashCommand {
    // ... existing commands
    Delegate { agent: String, task: String },
    AgentsList,
    AgentsStatus,
}
```

### 5.2 Visual Indicators
**Goal**: Clear indication of agent delegation status

**Tasks**:
- Add agent name indicators in conversation
- Show delegation status in prompt
- Implement delegation history tracking

**Visual Design**:
```
[main] > Your request
[code-reviewer] > Analyzing code quality...
[main] > Based on the code review, here are the recommendations...
```

## Phase 6: Advanced Features (Weeks 11-12)

### 6.1 Agent Chaining
**Goal**: Support multi-step agent workflows

**Tasks**:
- Implement sequential agent execution
- Add workflow definition capabilities
- Create result passing between agents

**Example Workflow**:
```json
{
  "name": "code-review-workflow",
  "steps": [
    {"agent": "code-analyzer", "task": "analyze code structure"},
    {"agent": "security-reviewer", "task": "check for security issues"},
    {"agent": "performance-optimizer", "task": "suggest optimizations"}
  ]
}
```

### 6.2 Performance Optimization
**Goal**: Optimize delegation performance

**Tasks**:
- Implement agent lazy loading
- Add context caching mechanisms
- Optimize memory usage for multiple contexts

## Implementation Priorities

### High Priority (Must Have)
1. Schema extension and basic delegation
2. Request analysis and agent selection
3. Context isolation (minimal level)
4. Basic integration with chat system

### Medium Priority (Should Have)
1. Advanced context inheritance levels
2. Slash commands for explicit delegation
3. Visual indicators and status display
4. Tool management integration

### Low Priority (Nice to Have)
1. Agent chaining workflows
2. Performance optimizations
3. Advanced delegation rules
4. Delegation analytics and metrics

## Testing Strategy

### Unit Tests
- Agent registry functionality
- Request analysis accuracy
- Context isolation mechanisms
- Delegation decision logic

### Integration Tests
- End-to-end delegation flows
- Tool access and permissions
- Context merging and state management
- Error handling and fallback scenarios

### Performance Tests
- Delegation latency measurements
- Memory usage with multiple contexts
- Concurrent delegation handling
- Large conversation history impact

## Migration Strategy

### Backward Compatibility
- Existing agent configurations work unchanged
- New delegation fields are optional
- Graceful degradation when delegation fails

### Rollout Plan
1. **Alpha**: Internal testing with basic delegation
2. **Beta**: Limited user testing with core features
3. **GA**: Full release with all features and documentation

## Documentation Updates

### User Documentation
- Agent chaining guide and examples
- Delegation configuration reference
- Best practices for agent specialization
- Troubleshooting delegation issues

### Developer Documentation
- Architecture overview and design decisions
- API reference for delegation components
- Extension points for custom delegation logic
- Performance tuning guidelines

## Success Metrics

### Functional Metrics
- Delegation accuracy (>90% correct agent selection)
- Context isolation effectiveness (no cross-contamination)
- Tool execution success rate in delegated contexts

### Performance Metrics
- Delegation overhead (<100ms)
- Memory usage increase (<20% per active sub-agent)
- User satisfaction with delegation experience

### Adoption Metrics
- Number of users creating specialized agents
- Frequency of delegation usage
- Agent configuration complexity trends
