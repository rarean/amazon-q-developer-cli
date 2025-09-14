use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DelegationConfig {
    /// Keywords that trigger delegation to this agent
    #[serde(default)]
    pub keywords: Vec<String>,
    /// Regex patterns that match tasks for this agent
    #[serde(default)]
    pub task_patterns: Vec<String>,
    /// Whether this agent can be automatically selected for delegation
    #[serde(default)]
    pub auto_delegate: bool,
    /// Priority for agent selection (0-10, higher is more preferred)
    #[serde(default = "default_priority")]
    pub priority: u8,
    /// Level of context inheritance from main conversation
    #[serde(default)]
    pub context_inheritance: ContextInheritanceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ContextInheritanceLevel {
    None,
    Minimal,
    Partial,
    Full,
}

impl Default for ContextInheritanceLevel {
    fn default() -> Self {
        Self::Minimal
    }
}

fn default_priority() -> u8 {
    5
}

impl Default for DelegationConfig {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            task_patterns: Vec::new(),
            auto_delegate: false,
            priority: default_priority(),
            context_inheritance: ContextInheritanceLevel::default(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delegation_config_default() {
        let config = DelegationConfig::default();
        assert_eq!(config.keywords, Vec::<String>::new());
        assert_eq!(config.task_patterns, Vec::<String>::new());
        assert!(!config.auto_delegate);
        assert_eq!(config.priority, 5);
        assert_eq!(config.context_inheritance, ContextInheritanceLevel::Minimal);
    }

    #[test]
    fn test_context_inheritance_level_default() {
        let level = ContextInheritanceLevel::default();
        assert_eq!(level, ContextInheritanceLevel::Minimal);
    }

    #[test]
    fn test_delegation_config_serialization() {
        let config = DelegationConfig {
            keywords: vec!["test".to_string(), "debug".to_string()],
            task_patterns: vec![".*test.*".to_string()],
            auto_delegate: true,
            priority: 8,
            context_inheritance: ContextInheritanceLevel::Full,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: DelegationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_context_inheritance_level_serialization() {
        let levels = vec![
            ContextInheritanceLevel::None,
            ContextInheritanceLevel::Minimal,
            ContextInheritanceLevel::Partial,
            ContextInheritanceLevel::Full,
        ];

        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            let deserialized: ContextInheritanceLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(level, deserialized);
        }
    }
}
