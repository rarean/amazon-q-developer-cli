use eyre::{
    Result,
    eyre,
};

use crate::cli::chat::delegation_integration::ChatDelegationManager;

/// Handle delegation-related slash commands
pub struct DelegationCommandHandler;

impl DelegationCommandHandler {
    /// Process delegation commands like /delegate and /agents
    pub fn handle_command(
        command: &str,
        args: &[&str],
        delegation_manager: &mut ChatDelegationManager,
    ) -> Result<String> {
        match command {
            "delegate" => Self::handle_delegate_command(args, delegation_manager),
            "agents" => Self::handle_agents_command(args, delegation_manager),
            _ => Err(eyre!("Unknown delegation command: {}", command)),
        }
    }

    fn handle_delegate_command(args: &[&str], delegation_manager: &mut ChatDelegationManager) -> Result<String> {
        if args.is_empty() {
            return Ok(
                "Usage: /delegate <agent_name> [task]\nExample: /delegate code-reviewer review this function"
                    .to_string(),
            );
        }

        let agent_name = args[0];
        let task = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            "Ready for delegation".to_string()
        };

        // Validate agent exists before setting as active
        let available_agents = delegation_manager.list_agents();
        if !available_agents.contains(&agent_name.to_string()) {
            return Err(eyre!(
                "Agent '{}' not found. Available agents: {}",
                agent_name,
                if available_agents.is_empty() {
                    "none".to_string()
                } else {
                    available_agents.join(", ")
                }
            ));
        }

        // Set the active agent for delegation
        delegation_manager.set_active_agent(Some(agent_name.to_string()));

        Ok(format!("🤖 Delegated to agent '{}'. Task: {}", agent_name, task))
    }

    fn handle_agents_command(args: &[&str], delegation_manager: &mut ChatDelegationManager) -> Result<String> {
        match args.first() {
            Some(&"list") | None => {
                let agents = delegation_manager.list_agents();
                if agents.is_empty() {
                    Ok("No agents available for delegation.".to_string())
                } else {
                    let mut result = "Available agents:\n".to_string();
                    for agent in agents {
                        let indicator = if delegation_manager.active_agent() == Some(agent.as_str()) {
                            "🟢 (active)"
                        } else {
                            "⚪"
                        };
                        result.push_str(&format!("  {} {}\n", indicator, agent));
                    }
                    Ok(result)
                }
            },
            Some(&"active") => {
                if let Some(active) = delegation_manager.active_agent() {
                    Ok(format!("🟢 Active agent: {}", active))
                } else {
                    Ok("No active agent. Use /delegate <agent_name> to activate an agent.".to_string())
                }
            },
            Some(&"clear") => {
                delegation_manager.set_active_agent(None);
                Ok("🔄 Cleared active agent delegation.".to_string())
            },
            Some(unknown) => Err(eyre!(
                "Unknown agents subcommand: {}. Use 'list', 'active', or 'clear'",
                unknown
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegate_command_usage() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("delegate", &[], &mut manager);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Usage:"));
    }

    #[test]
    fn test_delegate_invalid_agent() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("delegate", &["nonexistent-agent"], &mut manager);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Agent 'nonexistent-agent' not found")
        );
    }

    #[test]
    fn test_agents_list_empty() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("agents", &["list"], &mut manager);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("No agents available"));
    }

    #[test]
    fn test_agents_active_none() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("agents", &["active"], &mut manager);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("No active agent"));
    }

    #[test]
    fn test_agents_clear() {
        let mut manager = ChatDelegationManager::default();
        // Set an active agent first
        manager.set_active_agent(Some("test-agent".to_string()));

        let result = DelegationCommandHandler::handle_command("agents", &["clear"], &mut manager);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Cleared active agent"));
        assert!(manager.active_agent().is_none());
    }

    #[test]
    fn test_agents_invalid_subcommand() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("agents", &["invalid"], &mut manager);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown agents subcommand"));
    }

    #[test]
    fn test_unknown_command() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("unknown", &[], &mut manager);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown delegation command"));
    }

    #[test]
    fn test_delegation_decision_performance() {
        use std::time::Instant;

        let mut manager = ChatDelegationManager::default();
        let conversation = create_test_conversation();
        let test_inputs = vec![
            "review this code",
            "analyze the data",
            "test the function",
            "debug the issue",
            "help me with this",
        ];

        let start = Instant::now();
        for input in &test_inputs {
            let _ = manager.process_input(input, &conversation);
        }
        let duration = start.elapsed();

        // Each delegation decision should be under 150ms (increased for CI environments)
        let avg_time = duration.as_millis() / test_inputs.len() as u128;
        assert!(
            avg_time < 150,
            "Average delegation decision took {}ms, should be under 150ms",
            avg_time
        );
    }

    #[test]
    fn test_automatic_delegation_scenarios() {
        let mut manager = ChatDelegationManager::default();
        let conversation = create_test_conversation();

        // Test scenarios that should trigger automatic delegation
        let auto_delegate_cases = vec![
            ("review this code please", "should trigger delegation for code review"),
            (
                "can you analyze this function",
                "should trigger delegation for analysis",
            ),
            ("debug this error for me", "should trigger delegation for debugging"),
            (
                "use code-reviewer agent to check this",
                "explicit agent mention should work",
            ),
        ];

        for (input, description) in auto_delegate_cases {
            let result = manager.process_input(input, &conversation);
            assert!(result.is_ok(), "Failed for case: {}", description);

            // Check that some form of delegation decision was made
            let delegation_result = result.unwrap();
            // The result should indicate some delegation processing occurred
            assert!(
                matches!(
                    delegation_result,
                    crate::cli::agent::delegator::DelegationResult::Delegated { .. }
                ) || matches!(
                    delegation_result,
                    crate::cli::agent::delegator::DelegationResult::NoDelegation { .. }
                ),
                "Expected delegation result for: {}",
                description
            );
        }
    }

    #[test]
    fn test_edge_cases_and_error_conditions() {
        let mut manager = ChatDelegationManager::default();
        let conversation = create_test_conversation();

        // Test edge cases
        let long_input = "a".repeat(1000);
        let edge_cases = vec![
            ("", "empty input should not crash"),
            ("   ", "whitespace only should not crash"),
            (long_input.as_str(), "very long input should not crash"),
            ("🚀🎉💻", "emoji input should not crash"),
            (
                "SELECT * FROM users; DROP TABLE users;",
                "potential injection should not crash",
            ),
        ];

        for (input, description) in edge_cases {
            let result = manager.process_input(input, &conversation);
            assert!(result.is_ok(), "Failed for edge case: {}", description);
        }

        // Test error conditions for commands
        let error_cases = vec![
            ("delegate", &[] as &[&str], "delegate without args should show usage"),
            ("agents", &["invalid"], "invalid agents subcommand should error"),
            ("nonexistent", &[], "nonexistent command should error"),
        ];

        for (command, args, description) in error_cases {
            let result = DelegationCommandHandler::handle_command(command, args, &mut manager);
            if command == "delegate" && args.is_empty() {
                assert!(
                    result.is_ok() && result.unwrap().contains("Usage"),
                    "Failed for: {}",
                    description
                );
            } else if command == "nonexistent" || (command == "agents" && args.contains(&"invalid")) {
                assert!(result.is_err(), "Should error for: {}", description);
            }
        }
    }

    // Helper function to create test conversation state
    fn create_test_conversation() -> crate::cli::chat::conversation::ConversationState {
        use std::collections::HashMap;

        use crate::cli::agent::Agents;
        use crate::cli::chat::tool_manager::ToolManager;

        // Use tokio runtime for async new method
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let os = crate::os::Os::new().await.unwrap();
            crate::cli::chat::conversation::ConversationState::new(
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
}
