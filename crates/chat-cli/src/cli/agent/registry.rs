use std::collections::HashMap;
use std::path::PathBuf;

use eyre::Result;
use regex::Regex;

#[cfg(test)]
use super::delegation::ContextInheritanceLevel;
use super::{
    Agent,
    DelegationConfig,
};
use crate::os::Os;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AgentCandidate {
    pub name: String,
    pub score: f32,
    pub reason: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
    keyword_index: HashMap<String, Vec<String>>, // keyword -> agent names
    pattern_index: Vec<(Regex, String)>,         // compiled pattern -> agent name
}

#[allow(dead_code)]
impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            agents: HashMap::new(),
            keyword_index: HashMap::new(),
            pattern_index: Vec::new(),
        })
    }
}

impl AgentRegistry {
    pub fn new() -> Result<Self> {
        Ok(Self {
            agents: HashMap::new(),
            keyword_index: HashMap::new(),
            pattern_index: Vec::new(),
        })
    }

    #[allow(dead_code)]
    pub async fn load_from_directories(os: &Os, paths: &[PathBuf]) -> Result<Self> {
        let mut registry = Self::new()?;

        for path in paths {
            if os.fs.exists(path) {
                registry.load_from_directory(os, path).await?;
            }
        }

        Ok(registry)
    }

    #[allow(dead_code)]
    async fn load_from_directory(&mut self, os: &Os, dir_path: &PathBuf) -> Result<()> {
        let mut entries = os.fs.read_dir(dir_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(agent) = Agent::load(os, &path, &mut None, true, &mut std::io::stderr()).await {
                    self.register_agent(agent);
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn register_agent(&mut self, agent: Agent) {
        let name = agent.name.clone();

        // Index delegation metadata if present
        if let Some(delegation) = &agent.delegation {
            self.index_delegation(&name, delegation);
        }

        self.agents.insert(name, agent);
    }

    #[allow(dead_code)]
    fn index_delegation(&mut self, agent_name: &str, delegation: &DelegationConfig) {
        // Index keywords
        for keyword in &delegation.keywords {
            self.keyword_index
                .entry(keyword.to_lowercase())
                .or_default()
                .push(agent_name.to_string());
        }

        // Index patterns
        for pattern_str in &delegation.task_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                self.pattern_index.push((regex, agent_name.to_string()));
            }
        }
    }

    pub fn find_candidates(&self, input: &str) -> Vec<AgentCandidate> {
        let mut candidates = Vec::new();
        let input_lower = input.to_lowercase();

        // Check keyword matches
        for (keyword, agent_names) in &self.keyword_index {
            if input_lower.contains(keyword) {
                for agent_name in agent_names {
                    if let Some(agent) = self.agents.get(agent_name) {
                        if let Some(delegation) = &agent.delegation {
                            if delegation.auto_delegate {
                                candidates.push(AgentCandidate {
                                    name: agent_name.clone(),
                                    score: delegation.priority as f32,
                                    reason: format!("keyword match: {}", keyword),
                                });
                            }
                        }
                    }
                }
            }
        }

        // Check pattern matches
        for (pattern, agent_name) in &self.pattern_index {
            if pattern.is_match(input) {
                if let Some(agent) = self.agents.get(agent_name) {
                    if let Some(delegation) = &agent.delegation {
                        if delegation.auto_delegate {
                            candidates.push(AgentCandidate {
                                name: agent_name.clone(),
                                score: delegation.priority as f32 + 1.0, // patterns get slight boost
                                reason: "pattern match".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Sort by score descending
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        candidates
    }

    pub fn get_agent(&self, name: &str) -> Option<&Agent> {
        self.agents.get(name)
    }

    pub fn list_agents(&self) -> Vec<&Agent> {
        self.agents.values().collect()
    }

    pub fn list_delegatable_agents(&self) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|agent| agent.delegation.as_ref().is_some_and(|d| d.auto_delegate))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = AgentRegistry::new();
        assert!(registry.is_ok());
    }

    #[test]
    fn test_backward_compatibility_agent_configs() {
        let mut registry = AgentRegistry::new().unwrap();

        // Test that agents without delegation config still work
        let basic_agent = Agent {
            name: "basic-agent".to_string(),
            delegation: None, // No delegation config
            ..Default::default()
        };

        registry.register_agent(basic_agent);

        // Should be able to retrieve the agent
        let retrieved = registry.get_agent("basic-agent");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "basic-agent");

        // Should handle delegation queries gracefully
        let _candidates = registry.find_candidates("test input");
        // Should not crash, may or may not include the basic agent
        // Length is always >= 0 for Vec, so just verify it doesn't crash
    }

    #[test]
    fn test_mixed_agent_configurations() {
        let mut registry = AgentRegistry::new().unwrap();

        // Add agents with different configuration styles
        let modern_agent = Agent {
            name: "modern-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["modern".to_string(), "new".to_string()],
                auto_delegate: true,
                context_inheritance: ContextInheritanceLevel::Full,
                ..Default::default()
            }),
            ..Default::default()
        };

        let simple_agent = Agent {
            name: "simple-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["simple".to_string()],
                auto_delegate: false,
                context_inheritance: ContextInheritanceLevel::None,
                ..Default::default()
            }),
            ..Default::default()
        };

        let no_delegation_agent = Agent {
            name: "no-delegation-agent".to_string(),
            delegation: None,
            ..Default::default()
        };

        registry.register_agent(modern_agent);
        registry.register_agent(simple_agent);
        registry.register_agent(no_delegation_agent);

        // All agents should be listed
        let all_agents = registry.list_agents();
        assert_eq!(all_agents.len(), 3);

        // Should be able to find each agent
        assert!(registry.get_agent("modern-agent").is_some());
        assert!(registry.get_agent("simple-agent").is_some());
        assert!(registry.get_agent("no-delegation-agent").is_some());

        // Delegation queries should work with mixed configurations
        let _candidates = registry.find_candidates("modern new approach");
        // Should handle mixed configurations without errors
        // Length is always >= 0 for Vec, so just verify it doesn't crash
    }

    #[test]
    fn test_register_agent() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "test-agent".to_string(),
            description: Some("Test agent".to_string()),
            delegation: Some(DelegationConfig {
                keywords: vec!["test".to_string()],
                task_patterns: vec!["test task".to_string()],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: false,
                priority: 5,
            }),
            ..Default::default()
        };

        registry.register_agent(agent);

        assert_eq!(registry.list_agents().len(), 1);
        assert!(registry.get_agent("test-agent").is_some());
    }

    #[test]
    fn test_register_multiple_agents() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent1 = Agent {
            name: "agent1".to_string(),
            ..Default::default()
        };
        let agent2 = Agent {
            name: "agent2".to_string(),
            ..Default::default()
        };

        registry.register_agent(agent1);
        registry.register_agent(agent2);

        assert_eq!(registry.list_agents().len(), 2);
        assert!(registry.get_agent("agent1").is_some());
        assert!(registry.get_agent("agent2").is_some());
    }

    #[test]
    fn test_register_duplicate_agent_overwrites() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent1 = Agent {
            name: "duplicate".to_string(),
            description: Some("First version".to_string()),
            ..Default::default()
        };
        let agent2 = Agent {
            name: "duplicate".to_string(),
            description: Some("Second version".to_string()),
            ..Default::default()
        };

        registry.register_agent(agent1);
        registry.register_agent(agent2);

        assert_eq!(registry.list_agents().len(), 1);
        let agent = registry.get_agent("duplicate").unwrap();
        assert_eq!(agent.description, Some("Second version".to_string()));
    }

    #[test]
    fn test_get_agent_existing() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "findme".to_string(),
            description: Some("Test agent".to_string()),
            ..Default::default()
        };
        registry.register_agent(agent);

        let found = registry.get_agent("findme");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "findme");
    }

    #[test]
    fn test_get_agent_nonexistent() {
        let registry = AgentRegistry::new().unwrap();

        let found = registry.get_agent("nonexistent");
        assert!(found.is_none());
    }

    #[test]
    fn test_list_agents_empty() {
        let registry = AgentRegistry::new().unwrap();

        let agents = registry.list_agents();
        assert_eq!(agents.len(), 0);
    }

    #[test]
    fn test_list_agents_multiple() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent1 = Agent {
            name: "agent1".to_string(),
            ..Default::default()
        };
        let agent2 = Agent {
            name: "agent2".to_string(),
            ..Default::default()
        };
        let agent3 = Agent {
            name: "agent3".to_string(),
            ..Default::default()
        };

        registry.register_agent(agent1);
        registry.register_agent(agent2);
        registry.register_agent(agent3);

        let agents = registry.list_agents();
        assert_eq!(agents.len(), 3);
    }

    #[test]
    fn test_list_delegatable_agents() {
        let mut registry = AgentRegistry::new().unwrap();

        let delegatable = Agent {
            name: "delegatable".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["test".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        let non_delegatable = Agent {
            name: "non-delegatable".to_string(),
            delegation: None,
            ..Default::default()
        };

        registry.register_agent(delegatable);
        registry.register_agent(non_delegatable);

        let delegatable_agents = registry.list_delegatable_agents();
        assert_eq!(delegatable_agents.len(), 1);
        assert_eq!(delegatable_agents[0].name, "delegatable");
    }

    #[test]
    fn test_find_candidates_keyword_match() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "keyword-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["rust".to_string(), "code".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("help me with rust programming");
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0].name, "keyword-agent");
    }

    #[test]
    fn test_find_candidates_no_match() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "specific-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["specific".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("unrelated query");
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_find_candidates_pattern_match() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "pattern-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec![],
                task_patterns: vec!["debug.*error".to_string()],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 7,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("debug this error");
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0].name, "pattern-agent");
    }

    #[test]
    fn test_find_candidates_priority_ordering() {
        let mut registry = AgentRegistry::new().unwrap();

        let low_priority = Agent {
            name: "low-priority".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["test".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 3,
            }),
            ..Default::default()
        };
        let high_priority = Agent {
            name: "high-priority".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["test".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 8,
            }),
            ..Default::default()
        };

        registry.register_agent(low_priority);
        registry.register_agent(high_priority);

        let candidates = registry.find_candidates("test query");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].name, "high-priority");
        assert_eq!(candidates[1].name, "low-priority");
    }

    #[test]
    fn test_find_candidates_empty_input() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "test-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["test".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("");
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_registry_with_empty_keywords() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "empty-keywords".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec![],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("any query");
        assert!(candidates.is_empty());
    }

    #[test]
    fn test_registry_case_insensitive_matching() {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "case-agent".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["RUST".to_string()],
                task_patterns: vec![],
                context_inheritance: ContextInheritanceLevel::Minimal,
                auto_delegate: true,
                priority: 5,
            }),
            ..Default::default()
        };
        registry.register_agent(agent);

        let candidates = registry.find_candidates("help with rust code");
        assert!(!candidates.is_empty());
        assert_eq!(candidates[0].name, "case-agent");
    }
}
