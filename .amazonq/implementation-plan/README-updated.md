# Amazon Q CLI Custom Commands Implementation Plan (Updated for Consistency)

## Overview

This document outlines the implementation plan for adding custom command functionality to Amazon Q CLI through a new `/commands` slash command. This feature will follow established patterns from the knowledge base feature to ensure consistent user experience and implementation patterns.

## Key Changes for Consistency

### UX Consistency Requirements
- **Settings Integration**: Follow exact knowledge base pattern (`chat.enableCommands`)
- **Command Structure**: Mirror `/knowledge` command patterns
- **Visual Indicators**: Use identical emojis and formatting
- **Error Messages**: Follow exact same templates and tone
- **Help System**: Maintain consistent structure and examples

### Implementation Benefits
- **Reduced Timeline**: 8.5 weeks (down from 9.5) due to pattern reuse
- **Lower Risk**: Established patterns reduce implementation uncertainty
- **Consistent Experience**: Users familiar with knowledge base will understand commands
- **Code Reuse**: Leverage existing settings, file handling, and error patterns

## Project Structure

```
.amazonq/implementation-plan/
├── README-updated.md                   # This file - updated project overview
├── IMPLEMENTATION_SUMMARY-updated.md   # Updated executive summary
├── architecture/
│   ├── 00-system-overview.md          # Original architecture
│   ├── 00-system-overview-updated.md  # Updated for consistency
│   ├── 01-command-structure.md        # Command structure and data models
│   ├── 02-storage-strategy.md         # File storage and organization
│   ├── 03-execution-engine.md         # Command execution architecture
│   └── 04-integration-points.md       # Integration with existing systems
├── design/
│   ├── 00-user-experience.md          # Original UX design
│   ├── 00-user-experience-updated.md  # Updated for consistency
│   ├── 01-consistency-guide.md        # NEW: Mandatory consistency requirements
│   ├── 01-command-format.md           # Command definition format
│   ├── 02-scope-resolution.md         # Global vs local scope handling
│   └── 03-security-model.md           # Security considerations
├── implementation/
│   ├── 00-development-phases.md       # Development phases and milestones
│   ├── 01-task-breakdown.md           # Detailed task breakdown
│   ├── 02-dependencies.md             # Dependencies and prerequisites
│   └── 03-testing-strategy.md         # Testing approach and coverage
├── tasks/
│   ├── phase-1/                       # Phase 1 implementation tasks
│   ├── phase-2/                       # Phase 2 implementation tasks
│   ├── phase-3/                       # Phase 3 implementation tasks
│   └── completed/                     # Completed tasks archive
└── progress/
    ├── status.md                      # Current implementation status
    ├── decisions.md                   # Architectural decisions log
    └── issues.md                      # Known issues and blockers
```

## Quick Start (Updated)

1. **Read Consistency Guide**: Start with `design/01-consistency-guide.md` (MANDATORY)
2. **Review Updated Architecture**: Check `architecture/00-system-overview-updated.md`
3. **Study Knowledge Base**: Analyze existing knowledge base implementation patterns
4. **Review Updated UX**: Check `design/00-user-experience-updated.md`
5. **Implementation**: Follow `IMPLEMENTATION_SUMMARY-updated.md`
6. **Track Progress**: Update `progress/status.md` regularly

## Key Principles (Updated)

- **Pattern Consistency**: Follow knowledge base patterns exactly for UX consistency
- **Incremental Development**: Work can be interrupted and resumed at any point
- **Network Safe**: All changes are atomic and can be safely committed
- **Security Alignment**: Same security model and permission levels as knowledge base
- **Code Reuse**: Leverage existing infrastructure and patterns where possible

## Consistency Requirements (MANDATORY)

### 1. Settings Integration
```rust
// MUST follow exact pattern
pub enum Setting {
    EnabledKnowledge,    // Existing
    EnabledCommands,     // NEW: Same pattern
}

impl Commands {
    pub fn is_enabled(os: &Os) -> bool {
        os.database
            .settings
            .get_bool(Setting::EnabledCommands)
            .unwrap_or(false)
    }
}
```

### 2. Command Structure
```bash
# MUST mirror knowledge base exactly
/knowledge show    ->    /commands show
/knowledge add     ->    /commands add
/knowledge remove  ->    /commands remove
/knowledge update  ->    /commands update
/knowledge clear   ->    /commands clear
/knowledge status  ->    /commands status
```

### 3. Error Messages
```bash
# MUST use exact same template
❌ Commands tool is disabled. Enable it with: q settings chat.enableCommands true
```

### 4. Visual Indicators
```bash
# MUST use identical emojis and formatting
📁 Project Commands (.amazonq/commands/):
🌍 User Commands (~/.amazonq/commands/):
✅ Success states
❌ Error states
⚠️  Warning states
```

## Implementation Phases (Optimized)

### Phase 1: Core Infrastructure (Weeks 1-3) - Reduced Timeline
**Focus**: Establish foundation using knowledge base patterns
- Settings integration (reuse exact pattern)
- Command manager (follow KnowledgeStore pattern)
- File storage (mirror knowledge base structure)
- CLI interface (copy knowledge base command structure)
- Tool registration (same pattern and security level)

### Phase 2: Command Execution (Weeks 4-6) - Reduced Timeline
**Focus**: Implement execution with established security patterns
- Argument processing and template system
- Bash execution (reuse knowledge base security model)
- File references (use existing file abstractions)
- CRUD operations (follow knowledge base error handling)

### Phase 3: Advanced Features (Weeks 7-8.5) - Reduced Timeline
**Focus**: Polish and advanced features
- Enhanced validation and templates
- Usage analytics (follow knowledge base patterns)
- Performance optimization
- Documentation and testing

## Success Criteria (Updated)

### Consistency Validation (MANDATORY)
- [ ] Side-by-side UX testing shows consistent experience
- [ ] Error messages follow exact same templates
- [ ] Settings integration works identically
- [ ] Visual indicators match exactly
- [ ] Help system maintains same structure
- [ ] Tool registration follows same pattern

### Technical Validation
- [ ] >95% test coverage including consistency tests
- [ ] Performance comparable to knowledge base
- [ ] Security audit using same criteria
- [ ] Code review validates pattern adherence

## Risk Mitigation (Updated)

### Reduced Risks Through Consistency
1. **Implementation Risk**: Reduced by following established patterns
2. **UX Risk**: Reduced by maintaining consistency with familiar feature
3. **Integration Risk**: Reduced by reusing existing integration points
4. **Security Risk**: Reduced by following established security model

### New Consistency Risks
1. **Pattern Drift**: Risk of deviating from knowledge base patterns
   - *Mitigation*: Mandatory consistency guide and automated checks
2. **Maintenance Overhead**: Risk of patterns diverging over time
   - *Mitigation*: Shared pattern documentation and regular audits

## Development Guidelines (NEW)

### Before Starting Implementation
1. **Study Knowledge Base**: Complete analysis of existing implementation
2. **Review Consistency Guide**: Understand mandatory requirements
3. **Set Up Validation**: Configure consistency tests and checks
4. **Plan Reviews**: Schedule regular consistency validation reviews

### During Implementation
1. **Pattern Adherence**: Strict adherence to knowledge base patterns
2. **Regular Validation**: Run consistency tests frequently
3. **Code Reviews**: Focus on pattern consistency
4. **User Testing**: Validate consistent experience

### Before Release
1. **Consistency Audit**: Complete side-by-side comparison
2. **User Acceptance**: Test with users familiar with knowledge base
3. **Documentation**: Ensure consistent examples and patterns
4. **Monitoring**: Set up consistency monitoring

## Next Steps

### Immediate Actions (Week 1)
1. **Team Alignment**: Review consistency requirements with team
2. **Knowledge Base Analysis**: Complete study of existing patterns
3. **Environment Setup**: Configure development with consistency checks
4. **Begin Implementation**: Start with settings integration following exact pattern

### Success Dependencies
1. **Pattern Adherence**: Commitment to following knowledge base patterns exactly
2. **Regular Validation**: Consistent testing and review processes
3. **User Feedback**: Regular validation of consistent experience
4. **Documentation**: Maintaining consistent documentation and examples

---

*Last Updated: 2025-07-10*
*Status: Updated for Knowledge Base Consistency*
*Key Changes: Optimized timeline, mandatory consistency requirements, reduced risk profile*
