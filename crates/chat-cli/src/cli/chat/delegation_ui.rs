use crate::cli::agent::DelegationResult;
use crate::cli::chat::delegation_integration::ChatDelegationManager;

/// Visual indicators and UI elements for delegation
#[allow(dead_code)]
pub struct DelegationUI;

#[allow(dead_code)]
impl DelegationUI {
    /// Generate prompt prefix showing active agent
    pub fn get_prompt_prefix(delegation_manager: &ChatDelegationManager) -> String {
        if let Some(agent) = delegation_manager.active_agent() {
            format!("🤖 {} > ", agent)
        } else {
            String::new()
        }
    }

    /// Generate status line for active delegation with enhanced info
    pub fn get_status_line(delegation_manager: &ChatDelegationManager) -> Option<String> {
        delegation_manager.active_agent().map(|agent| {
            format!(
                "🟢 Active Agent: {} | Type '/agents clear' to return to main agent",
                agent
            )
        })
    }

    /// Format delegation result for display with enhanced messaging
    pub fn format_delegation_result(result: &DelegationResult) -> String {
        match result {
            DelegationResult::Delegated { agent, task, .. } => {
                format!("🤖 ✅ Successfully delegated to '{}': {}", agent, task)
            },
            DelegationResult::NoDelegation => "ℹ️  No suitable agent found for delegation".to_string(),
        }
    }

    /// Generate agent switch notification
    pub fn format_agent_switch(from: Option<&str>, to: &str) -> String {
        match from {
            Some(prev) => format!("🔄 Switched from '{}' to '{}'", prev, to),
            None => format!("🤖 Activated agent '{}'", to),
        }
    }

    /// Format delegation failure with user-friendly error message
    pub fn format_delegation_failure(agent: &str, error: &str) -> String {
        format!(
            "❌ Failed to delegate to '{}': {}\n💡 Tip: Try '/agents list' to see available agents",
            agent, error
        )
    }

    /// Format delegation notification
    pub fn format_delegation_notification(agent: &str, task: &str) -> String {
        format!("📋 Delegating task to '{}': {}", agent, task)
    }

    /// Format agent availability status
    pub fn format_agent_status(agent: &str, available: bool, reason: Option<&str>) -> String {
        if available {
            format!("🟢 {} - Available", agent)
        } else {
            match reason {
                Some(r) => format!("🔴 {} - Unavailable ({})", agent, r),
                None => format!("🔴 {} - Unavailable", agent),
            }
        }
    }

    /// Generate delegation help text with enhanced examples
    pub fn get_help_text() -> String {
        r#"🤖 Agent Delegation Help

Available Commands:
  /delegate <agent> [task]  - Delegate to specific agent
  /agents list             - List all available agents  
  /agents active           - Show currently active agent
  /agents clear            - Return to main agent

Examples:
  /delegate code-reviewer review this function
  /delegate data-analyst analyze this dataset
  /agents list

💡 Tips:
- Agents can be invoked naturally: "use code-reviewer agent to help"
- Auto-delegation works with keywords: "please review this code"
- Use '/agents clear' to return to the main conversation agent"#
            .to_string()
    }

    /// Format delegation statistics for display
    pub fn format_delegation_stats(total_delegations: usize, successful: usize, active_contexts: usize) -> String {
        let success_rate = if total_delegations > 0 {
            (successful as f32 / total_delegations as f32 * 100.0) as u32
        } else {
            0
        };

        format!(
            "📊 Delegation Stats: {} total, {}% success rate, {} active contexts",
            total_delegations, success_rate, active_contexts
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_prefix_with_agent() {
        let mut manager = ChatDelegationManager::default();
        manager.set_active_agent(Some("test-agent".to_string()));
        let prefix = DelegationUI::get_prompt_prefix(&manager);
        assert_eq!(prefix, "🤖 test-agent > ");
    }

    #[test]
    fn test_prompt_prefix_without_agent() {
        let manager = ChatDelegationManager::default();
        let prefix = DelegationUI::get_prompt_prefix(&manager);
        assert_eq!(prefix, "");
    }

    #[test]
    fn test_status_line() {
        let mut manager = ChatDelegationManager::default();
        manager.set_active_agent(Some("test-agent".to_string()));
        let status = DelegationUI::get_status_line(&manager);
        assert_eq!(
            status,
            Some("🟢 Active Agent: test-agent | Type '/agents clear' to return to main agent".to_string())
        );
    }
}
