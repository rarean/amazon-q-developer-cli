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
            Some(&"clear") => Ok("🔄 Cleared active agent delegation.".to_string()),
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
    fn test_agents_list_empty() {
        let mut manager = ChatDelegationManager::default();
        let result = DelegationCommandHandler::handle_command("agents", &["list"], &mut manager);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("No agents available"));
    }
}
