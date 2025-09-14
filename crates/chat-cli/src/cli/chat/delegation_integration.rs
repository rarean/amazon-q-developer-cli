use std::sync::Arc;

use eyre::Result;

use crate::cli::agent::{
    AgentDelegator,
    AgentRegistry,
    DelegationResult,
};
use crate::cli::chat::conversation::ConversationState;

/// Integration layer for delegation functionality in the chat loop
pub struct ChatDelegationManager {
    delegator: AgentDelegator,
    registry: AgentRegistry,
    active_agent: Option<String>,
}

impl ChatDelegationManager {
    pub fn new() -> Result<Self> {
        let registry = AgentRegistry::new()?;
        let delegator = AgentDelegator::new(Arc::new(registry.clone()));

        Ok(Self {
            delegator,
            registry,
            active_agent: None,
        })
    }

    /// Process user input for potential delegation with enhanced error handling
    pub fn process_input(&mut self, input: &str, _conversation: &ConversationState) -> Result<DelegationResult> {
        // Use enhanced delegation with fallback
        self.delegator.try_delegate_with_fallback(input)
    }

    /// Force delegation to a specific agent
    #[allow(dead_code)]
    pub fn delegate_to_agent(
        &mut self,
        agent_name: &str,
        input: &str,
        _conversation: &ConversationState,
    ) -> Result<DelegationResult> {
        // For now, just call force_delegate - we'll integrate conversation state later
        self.delegator.force_delegate(agent_name, input)
    }

    /// Get the currently active agent
    pub fn active_agent(&self) -> Option<&str> {
        self.active_agent.as_deref()
    }

    /// Set the active agent
    pub fn set_active_agent(&mut self, agent_name: Option<String>) {
        self.active_agent = agent_name;
    }

    /// List available agents for delegation
    pub fn list_agents(&self) -> Vec<String> {
        self.registry
            .list_agents()
            .iter()
            .map(|agent| agent.name.clone())
            .collect()
    }

    /// Get delegation statistics
    #[allow(dead_code)] // Used in tests
    pub fn get_delegation_stats(&self) -> (usize, usize, usize) {
        let history = self.delegator.get_delegation_history();
        let total = history.len();
        let successful = history.iter().filter(|h| h.success).count();
        let active_contexts = self.delegator.list_active_delegations().len();
        (total, successful, active_contexts)
    }
}

impl Default for ChatDelegationManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            delegator: AgentDelegator::new(Arc::new(AgentRegistry::new().unwrap_or_default())),
            registry: AgentRegistry::default(),
            active_agent: None,
        })
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn create_test_conversation() -> ConversationState {
        // Create a minimal conversation state for testing
        // This is a simplified version just for delegation tests
        use std::collections::HashMap;

        use crate::cli::agent::Agents;
        use crate::cli::chat::conversation::ConversationState;
        use crate::cli::chat::tool_manager::ToolManager;

        // Use tokio runtime for async new method
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let os = crate::os::Os::new().await.unwrap();
            ConversationState::new(
                "test_conversation",
                Agents::default(),
                HashMap::new(),
                ToolManager::default(),
                None,
                &os,
                false,
            )
            .await
        })
    }

    #[test]
    fn test_end_to_end_explicit_delegation() {
        let mut manager = ChatDelegationManager::new().unwrap();
        let conversation = create_test_conversation();

        // Test explicit delegation
        let result = manager.process_input("use code-reviewer agent to review this", &conversation);

        // Should either delegate or return no delegation based on available agents
        match result {
            Ok(DelegationResult::Delegated { agent, .. }) => {
                assert!(!agent.is_empty());
            },
            Ok(DelegationResult::NoDelegation) => {
                // No agents available, which is expected in test environment
            },
            Err(_) => panic!("Should not error on valid input"),
        }
    }

    #[test]
    fn test_end_to_end_auto_delegation() {
        let mut manager = ChatDelegationManager::new().unwrap();
        let conversation = create_test_conversation();

        // Test auto-delegation with keywords
        let result = manager.process_input("please review this code", &conversation);

        // Should either delegate or return no delegation
        match result {
            Ok(DelegationResult::Delegated { .. }) | Ok(DelegationResult::NoDelegation) => {},
            Err(_) => panic!("Should not error on valid input"),
        }
    }

    #[test]
    fn test_agent_management() {
        let mut manager = ChatDelegationManager::new().unwrap();

        // Test agent activation
        assert!(manager.active_agent().is_none());

        manager.set_active_agent(Some("test-agent".to_string()));
        assert_eq!(manager.active_agent(), Some("test-agent"));

        manager.set_active_agent(None);
        assert!(manager.active_agent().is_none());
    }

    #[test]
    fn test_delegation_stats() {
        let manager = ChatDelegationManager::new().unwrap();
        let (total, successful, active) = manager.get_delegation_stats();

        // New manager should have no history
        assert_eq!(total, 0);
        assert_eq!(successful, 0);
        assert_eq!(active, 0);
    }

    #[test]
    fn test_list_agents() {
        let manager = ChatDelegationManager::new().unwrap();
        let _agents = manager.list_agents();

        // Should return a list (may be empty in test environment)
        // Length is always >= 0 for Vec, so just verify it doesn't crash
    }

    #[test]
    fn test_fallback_behavior() {
        let mut manager = ChatDelegationManager::new().unwrap();
        let conversation = create_test_conversation();

        // Test with non-existent agent
        let result = manager.process_input("use nonexistent-agent", &conversation);

        // Should gracefully handle non-existent agents
        match result {
            Ok(DelegationResult::NoDelegation) => {}, // Expected
            Ok(DelegationResult::Delegated { .. }) => panic!("Should not delegate to non-existent agent"),
            Err(_) => {}, // Also acceptable for missing agents
        }
    }
}
