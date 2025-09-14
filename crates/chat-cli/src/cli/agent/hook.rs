use std::fmt::Display;

use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_MAX_OUTPUT_SIZE: usize = 1024 * 10;
const DEFAULT_CACHE_TTL_SECONDS: u64 = 0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, JsonSchema, Hash)]
#[serde(rename_all = "camelCase")]
pub enum HookTrigger {
    /// Triggered during agent spawn
    AgentSpawn,
    /// Triggered per user message submission
    UserPromptSubmit,
    /// Triggered before tool execution
    PreToolUse,
    /// Triggered after tool execution
    PostToolUse,
}

impl Display for HookTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookTrigger::AgentSpawn => write!(f, "agentSpawn"),
            HookTrigger::UserPromptSubmit => write!(f, "userPromptSubmit"),
            HookTrigger::PreToolUse => write!(f, "preToolUse"),
            HookTrigger::PostToolUse => write!(f, "postToolUse"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Hash)]
pub enum Source {
    Agent,
    Session,
}

impl Default for Source {
    fn default() -> Self {
        Self::Agent
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema, Hash)]
pub struct Hook {
    /// The command to run when the hook is triggered
    pub command: String,

    /// Max time the hook can run before it throws a timeout error
    #[serde(default = "Hook::default_timeout_ms")]
    pub timeout_ms: u64,

    /// Max output size of the hook before it is truncated
    #[serde(default = "Hook::default_max_output_size")]
    pub max_output_size: usize,

    /// How long the hook output is cached before it will be executed again
    #[serde(default = "Hook::default_cache_ttl_seconds")]
    pub cache_ttl_seconds: u64,

    /// Optional glob matcher for hook
    /// Currently used for matching tool name of PreToolUse and PostToolUse hook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,

    #[schemars(skip)]
    #[serde(default, skip_serializing)]
    pub source: Source,
}

impl Hook {
    pub fn new(command: String, source: Source) -> Self {
        Self {
            command,
            timeout_ms: Self::default_timeout_ms(),
            max_output_size: Self::default_max_output_size(),
            cache_ttl_seconds: Self::default_cache_ttl_seconds(),
            matcher: None,
            source,
        }
    }

    fn default_timeout_ms() -> u64 {
        DEFAULT_TIMEOUT_MS
    }

    fn default_max_output_size() -> usize {
        DEFAULT_MAX_OUTPUT_SIZE
    }

    fn default_cache_ttl_seconds() -> u64 {
        DEFAULT_CACHE_TTL_SECONDS
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_new() {
        let hook = Hook::new("echo test".to_string(), Source::Agent);
        assert_eq!(hook.command, "echo test");
        assert_eq!(hook.timeout_ms, DEFAULT_TIMEOUT_MS);
        assert_eq!(hook.max_output_size, DEFAULT_MAX_OUTPUT_SIZE);
        assert_eq!(hook.cache_ttl_seconds, DEFAULT_CACHE_TTL_SECONDS);
        assert_eq!(hook.source, Source::Agent);
    }

    #[test]
    fn test_hook_trigger_display() {
        assert_eq!(HookTrigger::AgentSpawn.to_string(), "agentSpawn");
        assert_eq!(HookTrigger::UserPromptSubmit.to_string(), "userPromptSubmit");
    }

    #[test]
    fn test_source_default() {
        let source = Source::default();
        assert_eq!(source, Source::Agent);
    }

    #[test]
    fn test_hook_serialization() {
        let hook = Hook {
            command: "test command".to_string(),
            timeout_ms: 5000,
            max_output_size: 2048,
            cache_ttl_seconds: 300,
            source: Source::Session,
        };

        let json = serde_json::to_string(&hook).unwrap();
        let deserialized: Hook = serde_json::from_str(&json).unwrap();
        assert_eq!(hook.command, deserialized.command);
        assert_eq!(hook.timeout_ms, deserialized.timeout_ms);
        assert_eq!(hook.max_output_size, deserialized.max_output_size);
        assert_eq!(hook.cache_ttl_seconds, deserialized.cache_ttl_seconds);
    }

    #[test]
    fn test_hook_trigger_serialization() {
        let triggers = vec![HookTrigger::AgentSpawn, HookTrigger::UserPromptSubmit];

        for trigger in triggers {
            let json = serde_json::to_string(&trigger).unwrap();
            let deserialized: HookTrigger = serde_json::from_str(&json).unwrap();
            assert_eq!(trigger, deserialized);
        }
    }

    #[test]
    fn test_hooks_hashmap() {
        let mut hooks = HashMap::new();
        let hook = Hook::new("test".to_string(), Source::Agent);
        hooks.insert(HookTrigger::AgentSpawn, hook);

        let hooks_struct = Hooks(hooks);
        let json = serde_json::to_string(&hooks_struct).unwrap();
        let deserialized: Hooks = serde_json::from_str(&json).unwrap();
        assert_eq!(hooks_struct, deserialized);
    }
}
