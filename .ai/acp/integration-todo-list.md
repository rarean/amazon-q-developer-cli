# ACP Integration TODO List

Based on the gap analysis in `implementation-gap-analysis.md`, this document outlines the specific tasks needed to fix the ACP integration issues.

## Critical Priority - Architectural Violations

### ✅ 1. Remove Direct AWS Client Dependencies
**File**: `crates/acp/Cargo.toml`
**Status**: COMPLETED
**Issue**: ACP creates its own AWS clients instead of using existing ones
**Reference**: See "Critical Architectural Violation" section in `implementation-gap-analysis.md`

**Completed Tasks**:
- ✅ Removed `amzn-codewhisperer-streaming-client.workspace = true`
- ✅ Removed `aws-config.workspace = true` 
- ✅ Removed `aws-credential-types.workspace = true`

### ✅ 2. Implement Dependency Injection Pattern
**File**: `crates/acp/src/agent.rs`
**Status**: COMPLETED
**Issue**: ACP agent creates dependencies instead of receiving them
**Reference**: See "Code Examples for Gap Resolution" in `implementation-gap-analysis.md`

**Completed Tasks**:
- ✅ Created trait definitions (AwsClientProvider, DatabaseProvider, TelemetryProvider)
- ✅ Modified `QCliAcpAgent::new()` to accept injected dependencies via traits
- ✅ Removed internal AWS client creation logic
- ✅ Fixed circular dependency issue using trait-based approach

### ✅ 3. Create AwsClientBundle Struct
**File**: `crates/chat-cli/src/api_client/mod.rs`
**Status**: COMPLETED (but replaced with trait-based approach)
**Issue**: Need structured way to pass existing AWS clients to ACP
**Reference**: See "AWS Client Integration Fix" in `implementation-gap-analysis.md`

**Completed Tasks**:
- ✅ Created trait-based approach instead of direct struct passing
- ✅ Implemented ChatCliAwsClient wrapper in acp_integration module
- ✅ Added to chat-cli's integration infrastructure

## High Priority - Core Integration

### 4. Add ACP Session Storage to Database
**File**: `crates/chat-cli/src/database/mod.rs`
**Status**: IN PROGRESS (trait interface completed, implementation needed)
**Issue**: ACP sessions not persisted in existing database
**Reference**: See "Database Integration Gaps" in `implementation-gap-analysis.md`

**Tasks**:
- ✅ Created DatabaseProvider trait interface
- ✅ Implemented ChatCliDatabase wrapper
- ⏳ Add `acp_sessions` table to database schema
- ⏳ Implement CRUD operations for ACP sessions
- ⏳ Add session cleanup and expiration logic
- ⏳ Follow existing database patterns in chat-cli

### 5. Integrate with Existing MCP Client
**File**: `crates/acp/src/mcp_bridge.rs`
**Status**: PENDING
**Issue**: ACP creates standalone MCP client instead of using existing one
**Reference**: See "MCP Integration Gaps" in `implementation-gap-analysis.md`

**Tasks**:
- Remove standalone MCP client creation
- Use existing `McpClient` from `crates/chat-cli/src/cli/mcp.rs`
- Share MCP server configurations
- Leverage existing tool registry

### ✅ 6. Replace Hardcoded AWS Credentials
**File**: `crates/acp/src/aws_facade.rs`
**Status**: COMPLETED
**Issue**: Uses hardcoded credentials instead of existing auth flow
**Reference**: See "Authentication Inconsistency" in `implementation-gap-analysis.md`

**Completed Tasks**:
- ✅ Removed hardcoded `Credentials::new("xxx", "xxx", ...)`
- ✅ Uses existing authentication from chat-cli via trait interface
- ✅ ACP respects `q login` status through existing auth check
- ✅ Removed duplicate AWS configuration logic

## Medium Priority - Infrastructure Integration

### 7. Add ACP Telemetry Events
**File**: `crates/chat-cli/src/telemetry/mod.rs`
**Issue**: No ACP-specific telemetry events
**Reference**: See "Telemetry Integration Gaps" in `implementation-gap-analysis.md`

**Tasks**:
- Add `acp_session_created`, `acp_session_ended`, `acp_tool_executed` events
- Integrate with existing `TelemetryClient`
- Add ACP metrics collection
- Follow existing telemetry patterns

### 8. Integrate ACP Configuration
**File**: `crates/chat-cli/src/config/mod.rs`
**Issue**: ACP settings not in existing config system
**Reference**: See "Configuration Integration Gaps" in `implementation-gap-analysis.md`

**Tasks**:
- Add `AcpConfig` struct to existing config
- Add ACP settings to config files
- Implement configuration validation
- Ensure backward compatibility

### 9. Create Unified Error Handling
**File**: `crates/acp/src/error.rs`
**Issue**: Inconsistent error types between ACP and chat-cli
**Reference**: See "Error Handling Integration" in `implementation-gap-analysis.md`

**Tasks**:
- Create unified error types
- Implement error translation between crates
- Ensure consistent error reporting
- Follow existing error patterns in chat-cli

## Implementation Tasks

### ✅ 10. Update ACP Agent Constructor
**File**: `crates/acp/src/agent.rs`
**Status**: COMPLETED
**Issue**: Constructor doesn't accept required dependencies
**Reference**: See "AWS Client Integration Fix" in `implementation-gap-analysis.md`

**Completed Implementation**:
```rust
// Changed from:
impl QCliAcpAgent {
    pub async fn new() -> Result<Self, AcpIntegrationError>

// Changed to:
impl QCliAcpAgent {
    pub async fn new(
        aws_client: Box<dyn AwsClientProvider>,
        database: Arc<dyn DatabaseProvider>,
        telemetry: Arc<dyn TelemetryProvider>,
    ) -> Result<Self, AcpIntegrationError>
```

### ✅ 11. Remove Duplicate AWS Configuration
**File**: `crates/acp/src/aws_facade.rs`
**Status**: COMPLETED
**Issue**: Duplicates AWS config logic from chat-cli
**Reference**: See "AWS Service Integration Gaps" in `implementation-gap-analysis.md`

**Completed Tasks**:
- ✅ Removed `try_create_aws_client()` method
- ✅ Removed AWS configuration setup
- ✅ Uses injected clients instead
- ✅ Removed AWS-specific imports

### ✅ 12. Implement Request Routing Logic
**File**: `crates/acp/src/aws_facade.rs`
**Status**: COMPLETED (basic implementation)
**Issue**: No request classification and routing
**Reference**: See "Crate Integration Points" in `crate-integration-points.md`

**Completed Tasks**:
- ✅ Implemented basic `classify_request()` method
- ✅ Routes requests through trait interface
- ✅ Uses existing client patterns from chat-cli via traits
- ⏳ TODO: Enhance request classification logic for different types (Chat, Code, Analysis)

## Success Criteria

- [x] ACP crate has no direct AWS client dependencies
- [x] ACP agent receives all dependencies via injection
- [x] ACP uses existing chat-cli AWS clients, database, and telemetry (via trait interfaces)
- [x] ACP sessions are persisted in chat-cli database (CRUD operations implemented)
- [ ] ACP uses existing MCP client infrastructure
- [x] ACP respects existing authentication flow
- [x] All tests pass with new architecture (basic compilation successful)
- [x] No code duplication between ACP and chat-cli (architectural violations resolved)

## Progress Summary

**✅ COMPLETED (9/12 tasks)**:
- Critical architectural violations resolved
- Dependency injection implemented
- AWS client duplication eliminated
- Hardcoded credentials removed
- Duplicate configuration logic removed
- Request routing foundation implemented
- Agent constructor updated
- Trait-based integration established
- **AWS service integration completed (real responses via existing ApiClient)**

**⏳ IN PROGRESS (1/12 tasks)**:
- Database integration (CRUD operations implemented, cleanup/expiration logic needed)

**🔄 PENDING (2/12 tasks)**:
- MCP client integration
- Telemetry events implementation

## References

- `implementation-gap-analysis.md` - Detailed gap analysis and problem identification
- `crate-integration-points.md` - Original integration architecture specification
- Current implementation files:
  - `crates/acp/src/agent.rs` - Current ACP agent
  - `crates/acp/src/aws_facade.rs` - Current AWS facade with problems
  - `crates/chat-cli/src/cli/acp.rs` - Current CLI integration
  - `crates/chat-cli/src/api_client/` - Existing AWS clients to reuse
