# Agent Chaining Gap Implementation Plan
*Arc42 Implementation Architecture*

## 1. Introduction and Goals

### 1.1 Requirements Overview
Complete the agent chaining implementation by addressing critical gaps in the existing foundation. Enable functional sub-agent delegation with minimal user friction and seamless integration.

### 1.2 Quality Goals
| Priority | Quality Goal | Target |
|----------|-------------|---------|
| 1 | **Functional Completeness** | All delegation features working end-to-end |
| 2 | **User Experience** | Intuitive delegation with clear visual feedback |
| 3 | **Performance** | Delegation overhead < 100ms |
| 4 | **Reliability** | Graceful fallback when delegation fails |

### 1.3 Stakeholders
- **End Users**: Developers using Q CLI for specialized tasks
- **Q CLI Maintainers**: Core development team implementing features

## 2. Architecture Constraints

### 2.1 Technical Constraints
- Preserve existing JSON agent configuration format
- Maintain backward compatibility with current agents
- Integrate with existing chat loop architecture
- Reuse current tool management and MCP integration

### 2.2 Implementation Constraints
- Minimal breaking changes to public APIs
- Leverage existing delegation foundation code
- Complete implementation in 4-week timeline

## 3. System Scope and Context

### 3.1 Current State
- ✅ Delegation foundation exists (registry, delegator, analyzer)
- ✅ Agent schema supports delegation configuration
- ❌ Not integrated into main chat loop
- ❌ No user-facing commands or UI

### 3.2 Target State
- ✅ Automatic delegation during conversation
- ✅ Explicit delegation via slash commands
- ✅ Visual indicators for active agents
- ✅ Complete context isolation

## 4. Solution Strategy

### 4.1 Implementation Approach
**Incremental Integration**: Add delegation capabilities to existing chat system without disrupting current functionality.

### 4.2 Key Design Decisions
- **Chat Loop Integration**: Inject delegation check before main agent processing
- **Command Integration**: Add delegation commands to existing SlashCommand enum
- **Context Strategy**: Complete context isolator implementation with inheritance levels
- **UX Strategy**: Minimal visual indicators with clear agent status

## 5. Building Block View

### 5.1 Integration Points
```
┌─────────────────────────────────────────────────────────────┐
│                    Chat Session                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │ Input Parser    │→ │ Delegation      │→ │ Agent Executor  │ │
│  │                 │  │ Manager         │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
│           │                     │                     │        │
│           ▼                     ▼                     ▼        │
│  ┌─────────────────────────────────────────────────────────────┤
│  │         SlashCommand Parser & UI Renderer                   │
│  └─────────────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────────┘
```

## 6. Runtime View

### 6.1 Delegation Flow
```
User Input → Slash Command Check → Delegation Analysis → 
Agent Selection → Context Creation → Task Execution → 
Result Integration → UI Update → Response Display
```

### 6.2 Critical Paths
1. **Automatic Delegation**: Input analysis → agent matching → execution
2. **Explicit Delegation**: Slash command → agent activation → task routing
3. **Context Management**: Isolation → inheritance → result merging

## 7. Implementation Plan

### 7.1 Phase 1: Core Integration (Week 1)
**Goal**: Wire delegation into main chat loop

#### Tasks:
- **Chat Loop Integration**
  - Modify `ChatSession::process_user_input()` to check delegation
  - Add `ChatDelegationManager` to session state
  - Implement delegation decision logic

- **Slash Command Integration**
  - Add `Delegate` and `Agents` variants to `SlashCommand` enum
  - Wire command handlers to existing delegation logic
  - Update command parser and help text

**Code Changes**:
```rust
// In ChatSession
pub struct ChatSession {
    // ... existing fields
    delegation_manager: ChatDelegationManager,
}

// In SlashCommand enum
pub enum SlashCommand {
    // ... existing variants
    /// Delegate task to specific agent
    Delegate { agent: String, task: Option<String> },
    /// Manage agent delegation
    #[command(subcommand)]
    Agents(AgentsSubcommand),
}
```

### 7.2 Phase 2: Context Isolation (Week 2)
**Goal**: Complete context inheritance implementation

#### Tasks:
- **Context Isolator Enhancement**
  - Implement all inheritance levels (None, Minimal, Partial, Full)
  - Add context filtering and selection logic
  - Create context merging strategies

- **Context Integration**
  - Wire context isolator into delegation flow
  - Add context size management
  - Implement context cleanup

**Code Changes**:
```rust
impl ContextIsolator {
    pub fn create_isolated_context(
        &self, 
        agent: &Agent, 
        main_context: &ConversationContext
    ) -> Result<IsolatedContext>;
    
    pub fn merge_context_results(
        &mut self, 
        isolated: IsolatedContext, 
        main_context: &mut ConversationContext
    ) -> Result<()>;
}
```

### 7.3 Phase 3: Automatic Delegation (Week 3)
**Goal**: Enable intelligent task routing

#### Tasks:
- **Request Analysis Enhancement**
  - Improve keyword and pattern matching
  - Add confidence scoring for agent selection
  - Implement delegation decision thresholds

- **Agent Selection Logic**
  - Complete priority-based selection algorithm
  - Add fallback mechanisms for failed delegation
  - Implement delegation history tracking

**Code Changes**:
```rust
impl RequestAnalyzer {
    pub fn analyze_delegation_intent(&self, input: &str) -> DelegationIntent;
    pub fn score_agent_candidates(&self, input: &str) -> Vec<(String, f32)>;
    pub fn should_auto_delegate(&self, input: &str, threshold: f32) -> bool;
}
```

### 7.4 Phase 4: User Experience (Week 4)
**Goal**: Polish delegation UX and documentation

#### Tasks:
- **Visual Indicators**
  - Add agent status to conversation display
  - Implement delegation notifications
  - Create agent switching indicators

- **Error Handling**
  - Add graceful delegation failure handling
  - Implement fallback to main agent
  - Create user-friendly error messages

- **Documentation**
  - Update user documentation with delegation examples
  - Create agent configuration guide
  - Add troubleshooting section

## 8. Detailed Implementation

### 8.1 Chat Loop Integration
**File**: `crates/chat-cli/src/cli/chat/mod.rs`

```rust
impl ChatSession {
    async fn process_user_input(&mut self, input: &str) -> Result<ChatState> {
        // Check for slash commands first
        if let Some(command) = self.parse_slash_command(input)? {
            return command.execute(&mut self.os, self).await;
        }

        // Check for delegation intent
        if let Ok(delegation_result) = self.delegation_manager.process_input(input, &self.conversation) {
            match delegation_result {
                DelegationResult::Delegated { agent, task, context } => {
                    return self.execute_delegated_task(agent, task, context).await;
                }
                DelegationResult::NoDelegation => {
                    // Continue with main agent
                }
            }
        }

        // Existing main agent processing
        self.process_with_main_agent(input).await
    }
}
```

### 8.2 Slash Command Integration
**File**: `crates/chat-cli/src/cli/chat/cli/mod.rs`

```rust
#[derive(Debug, PartialEq, Parser)]
pub enum SlashCommand {
    // ... existing variants
    
    /// Delegate task to specific agent
    Delegate {
        /// Agent name to delegate to
        agent: String,
        /// Optional task description
        task: Option<String>,
    },
    
    /// Manage agent delegation
    #[command(subcommand)]
    Agents(AgentsSubcommand),
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum AgentsSubcommand {
    /// List available agents
    List,
    /// Show active agent
    Active,
    /// Clear active agent
    Clear,
}
```

### 8.3 Context Isolation Implementation
**File**: `crates/chat-cli/src/cli/agent/context_isolator.rs`

```rust
impl ContextIsolator {
    pub fn create_isolated_context(
        &self,
        agent: &Agent,
        main_context: &ConversationContext,
    ) -> Result<IsolatedContext> {
        let inheritance_level = agent.delegation
            .as_ref()
            .map(|d| &d.context_inheritance)
            .unwrap_or(&ContextInheritanceLevel::Minimal);

        let inherited_messages = match inheritance_level {
            ContextInheritanceLevel::None => Vec::new(),
            ContextInheritanceLevel::Minimal => {
                main_context.messages.iter().rev().take(3).cloned().collect()
            }
            ContextInheritanceLevel::Partial => {
                self.filter_relevant_messages(main_context, agent)
            }
            ContextInheritanceLevel::Full => main_context.messages.clone(),
        };

        Ok(IsolatedContext {
            agent_name: agent.name.clone(),
            messages: inherited_messages,
            tools: self.resolve_agent_tools(agent)?,
        })
    }
}
```

## 9. Testing Strategy

### 9.1 Unit Tests
- Delegation decision logic accuracy
- Context isolation and inheritance
- Slash command parsing and execution
- Agent selection algorithms

### 9.2 Integration Tests
- End-to-end delegation flows
- Chat loop integration
- Context merging and state management
- Error handling and fallback scenarios

### 9.3 User Acceptance Tests
- Natural language delegation scenarios
- Explicit agent invocation workflows
- Visual indicator functionality
- Performance under typical usage

## 10. Success Criteria

### 10.1 Functional Requirements
- ✅ Users can delegate tasks via `/delegate <agent>` command
- ✅ Automatic delegation works for configured keywords/patterns
- ✅ Context isolation prevents cross-contamination
- ✅ Visual indicators show active agent status
- ✅ Graceful fallback when delegation fails

### 10.2 Performance Requirements
- Delegation decision time < 50ms
- Context creation overhead < 100ms
- Memory usage increase < 15% per active delegation
- No impact on non-delegation conversations

### 10.3 Quality Requirements
- Zero breaking changes to existing agent configurations
- Backward compatibility with current chat workflows
- Clear error messages for delegation failures
- Comprehensive test coverage (>85%)

## 11. Risk Mitigation

### 11.1 Technical Risks
| Risk | Impact | Mitigation |
|------|--------|------------|
| Chat loop integration complexity | High | Incremental integration with feature flags |
| Context isolation bugs | Medium | Comprehensive unit tests and validation |
| Performance degradation | Medium | Benchmarking and optimization |

### 11.2 Implementation Risks
| Risk | Impact | Mitigation |
|------|--------|------------|
| Timeline pressure | Medium | Prioritize core functionality over polish |
| Breaking changes | High | Extensive backward compatibility testing |
| User confusion | Low | Clear documentation and examples |

## 12. Delivery Timeline

### Week 1: Foundation Integration
- Day 1-2: Chat loop integration
- Day 3-4: Slash command wiring
- Day 5: Integration testing

### Week 2: Context Management
- Day 1-2: Context isolator completion
- Day 3-4: Inheritance level implementation
- Day 5: Context integration testing

### Week 3: Intelligence Layer
- Day 1-2: Request analysis enhancement
- Day 3-4: Automatic delegation logic
- Day 5: End-to-end testing

### Week 4: User Experience
- Day 1-2: Visual indicators and UI
- Day 3-4: Error handling and polish
- Day 5: Documentation and final testing

## 13. Conclusion

This implementation plan addresses the critical gaps in agent chaining functionality while maintaining system stability and user experience. The phased approach ensures incremental progress with testable milestones, enabling early feedback and course correction.

The focus on integration over new development leverages the existing foundation effectively, minimizing risk while delivering complete functionality within the 4-week timeline.
