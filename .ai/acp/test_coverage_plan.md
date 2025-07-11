# ACP Crate Test Coverage Plan
**Target: Increase coverage from 4.78% to 80%**

## Current Coverage Analysis
- **Current**: 675/14,133 lines (4.78%)
- **Target**: 11,306/14,133 lines (80%)
- **Gap**: 10,631 additional lines needed

## Priority Files (Zero Coverage - Critical)

### 1. session_manager.rs (0/114 lines)
**Priority: HIGH**
- [ ] Test session creation and lifecycle
- [ ] Test session state transitions
- [ ] Test concurrent session handling
- [ ] Test session cleanup and expiration
- [ ] Test error handling for invalid sessions

### 2. session_persistence.rs (0/56 lines)
**Priority: HIGH**
- [ ] Test session save/load operations
- [ ] Test persistence failure scenarios
- [ ] Test data corruption handling
- [ ] Test concurrent access to persistence layer

### 3. parameter_translator.rs (0/14 lines)
**Priority: MEDIUM**
- [ ] Test parameter transformation between formats
- [ ] Test edge cases with malformed parameters
- [ ] Test type conversion validation

### 4. prompt_request_wrapper.rs (0/23 lines)
**Priority: MEDIUM**
- [ ] Test request wrapping functionality
- [ ] Test metadata preservation
- [ ] Test error propagation

## Low Coverage Files (Improvement Needed)

### 5. aws_facade.rs (14/59 lines - 23.7%)
**Priority: HIGH**
- [ ] Test AWS client initialization
- [ ] Test streaming response handling
- [ ] Test error scenarios (network failures, auth issues)
- [ ] Test timeout handling
- [ ] Test connection pooling integration

### 6. stdin_transformer.rs (7/21 lines - 33.3%)
**Priority: MEDIUM**
- [ ] Test input transformation logic
- [ ] Test malformed input handling
- [ ] Test streaming input processing

## Moderate Coverage Files (Enhancement)

### 7. agent.rs (57/101 lines - 56.4%)
**Priority: MEDIUM**
- [ ] Test additional error paths
- [ ] Test edge cases in prompt processing
- [ ] Test session notification edge cases

### 8. file_ops.rs (50/80 lines - 62.5%)
**Priority: LOW**
- [ ] Test file operation error scenarios
- [ ] Test permission handling
- [ ] Test concurrent file access

## Implementation Strategy

### Phase 1: Critical Zero Coverage (Week 1)
1. **session_manager.rs** - Create comprehensive unit tests
2. **session_persistence.rs** - Add persistence layer tests
3. **aws_facade.rs** - Expand AWS integration tests

### Phase 2: Parameter & Request Handling (Week 2)
1. **parameter_translator.rs** - Add transformation tests
2. **prompt_request_wrapper.rs** - Add wrapper functionality tests
3. **stdin_transformer.rs** - Expand input processing tests

### Phase 3: Enhancement & Edge Cases (Week 3)
1. **agent.rs** - Add missing error path tests
2. **file_ops.rs** - Add file operation edge cases
3. Integration tests for cross-module functionality

## Test Implementation Guidelines

### Mock Strategy
- Use existing `mocks.rs` patterns (91.3% coverage)
- Mock AWS clients for `aws_facade.rs` tests
- Mock file system for `file_ops.rs` tests
- Mock persistence layer for `session_manager.rs` tests

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::*;
    
    #[tokio::test]
    async fn test_function_name() {
        // Arrange
        // Act  
        // Assert
    }
}
```

### Coverage Targets by File
- session_manager.rs: 0% → 85% (+97 lines)
- session_persistence.rs: 0% → 85% (+48 lines)
- aws_facade.rs: 24% → 80% (+33 lines)
- parameter_translator.rs: 0% → 90% (+13 lines)
- prompt_request_wrapper.rs: 0% → 90% (+21 lines)
- stdin_transformer.rs: 33% → 80% (+10 lines)

## Success Metrics
- **Overall coverage**: 4.78% → 80%
- **Zero coverage files**: 4 → 0
- **Files >80% coverage**: 2 → 8
- **Test count**: 99 → ~150 tests

## Risk Mitigation
- Focus on business logic over generated code
- Prioritize error paths and edge cases
- Ensure tests are maintainable and fast
- Use integration tests sparingly to avoid flakiness
