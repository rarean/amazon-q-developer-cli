#[cfg(test)]
mod integration_tests {
    use crate::cli::agent::registry::AgentRegistry;

    #[test]
    fn test_agent_registry_creation() {
        // Test basic registry creation
        let result = AgentRegistry::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_registry_operations() {
        // Test basic registry operations
        if let Ok(registry) = AgentRegistry::new() {
            let _agent_count = registry.list_agents().len();
            // Just verify the operation completes without error
        }
    }

    #[test]
    fn test_agent_lookup() {
        // Test agent lookup functionality
        if let Ok(registry) = AgentRegistry::new() {
            let nonexistent = registry.get_agent("nonexistent");
            assert!(nonexistent.is_none());
        }
    }
}
