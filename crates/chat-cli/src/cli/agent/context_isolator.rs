use std::collections::HashMap;

use eyre::Result;

use super::Agent;
use super::delegation::ContextInheritanceLevel;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SimpleMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
}

#[allow(dead_code)]
impl SimpleMessage {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SubAgentContext {
    pub agent_name: String,
    pub task_description: String,
    pub inherited_messages: Vec<SimpleMessage>,
    pub context_id: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ContextIsolator {
    main_messages: Vec<SimpleMessage>,
    active_sub_contexts: HashMap<String, SubAgentContext>,
    next_context_id: usize,
}

#[allow(dead_code)]
impl ContextIsolator {
    pub fn new() -> Self {
        Self {
            main_messages: Vec::new(),
            active_sub_contexts: HashMap::new(),
            next_context_id: 0,
        }
    }

    pub fn update_main_context(&mut self, messages: Vec<SimpleMessage>) {
        self.main_messages = messages;
    }

    pub fn create_sub_context(&mut self, agent: &Agent, task: &str) -> Result<String> {
        let context_id = format!("sub_ctx_{}", self.next_context_id);
        self.next_context_id += 1;

        let inheritance_level = agent
            .delegation
            .as_ref()
            .map_or(&ContextInheritanceLevel::Minimal, |d| &d.context_inheritance);

        let inherited_messages = self.select_messages_for_inheritance(inheritance_level, task);

        let sub_context = SubAgentContext {
            agent_name: agent.name.clone(),
            task_description: task.to_string(),
            inherited_messages,
            context_id: context_id.clone(),
        };

        self.active_sub_contexts.insert(context_id.clone(), sub_context);
        Ok(context_id)
    }

    fn select_messages_for_inheritance(&self, level: &ContextInheritanceLevel, task: &str) -> Vec<SimpleMessage> {
        match level {
            ContextInheritanceLevel::None => Vec::new(),
            ContextInheritanceLevel::Minimal => {
                // Last 2-3 messages
                self.main_messages.iter().rev().take(3).rev().cloned().collect()
            },
            ContextInheritanceLevel::Partial => {
                // Messages containing keywords from the task
                let task_keywords: Vec<&str> = task
                    .split_whitespace()
                    .filter(|word| word.len() > 3) // Only meaningful words
                    .collect();

                self.main_messages
                    .iter()
                    .filter(|msg| {
                        let content = msg.content().to_lowercase();
                        task_keywords
                            .iter()
                            .any(|keyword| content.contains(&keyword.to_lowercase()))
                    })
                    .cloned()
                    .collect()
            },
            ContextInheritanceLevel::Full => {
                // All messages (with reasonable limit)
                self.main_messages
                    .iter()
                    .rev()
                    .take(50) // Limit to last 50 messages
                    .rev()
                    .cloned()
                    .collect()
            },
        }
    }

    pub fn get_sub_context(&self, context_id: &str) -> Option<&SubAgentContext> {
        self.active_sub_contexts.get(context_id)
    }

    pub fn merge_sub_context_result(&mut self, context_id: &str, result_message: SimpleMessage) -> Result<()> {
        if let Some(_sub_context) = self.active_sub_contexts.remove(context_id) {
            // Add the result to main context
            self.main_messages.push(result_message);
            Ok(())
        } else {
            Err(eyre::eyre!("Sub-context {} not found", context_id))
        }
    }

    pub fn list_active_contexts(&self) -> Vec<&SubAgentContext> {
        self.active_sub_contexts.values().collect()
    }

    pub fn cleanup_context(&mut self, context_id: &str) -> Option<SubAgentContext> {
        self.active_sub_contexts.remove(context_id)
    }

    /// Create an isolated context for a sub-agent with appropriate message inheritance
    pub fn create_isolated_context(&mut self, agent: &Agent, task: &str) -> Result<String> {
        self.create_sub_context(agent, task)
    }

    /// Merge context results back into main conversation
    pub fn merge_context_results(&mut self, context_id: &str, results: Vec<SimpleMessage>) -> Result<()> {
        if let Some(_sub_context) = self.active_sub_contexts.remove(context_id) {
            // Add all result messages to main context
            self.main_messages.extend(results);
            Ok(())
        } else {
            Err(eyre::eyre!("Sub-context {} not found", context_id))
        }
    }

    /// Get context size for management
    pub fn get_context_size(&self) -> usize {
        self.main_messages.len()
    }

    /// Cleanup old contexts based on age or size limits
    pub fn cleanup_old_contexts(&mut self, max_contexts: usize) {
        if self.active_sub_contexts.len() > max_contexts {
            // Remove oldest contexts (simple FIFO based on context_id numbering)
            let mut context_ids: Vec<_> = self.active_sub_contexts.keys().cloned().collect();
            context_ids.sort();

            let to_remove = context_ids.len() - max_contexts;
            for context_id in context_ids.into_iter().take(to_remove) {
                self.active_sub_contexts.remove(&context_id);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::agent::delegation::{
        ContextInheritanceLevel,
        DelegationConfig,
    };

    fn create_test_agent(name: &str, inheritance: ContextInheritanceLevel) -> Agent {
        Agent {
            name: name.to_string(),
            delegation: Some(DelegationConfig {
                context_inheritance: inheritance,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_simple_message_new() {
        let msg = SimpleMessage::new("user", "test content");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "test content");
        assert_eq!(msg.content(), "test content");
    }

    #[test]
    fn test_context_isolator_new() {
        let isolator = ContextIsolator::new();
        assert!(isolator.main_messages.is_empty());
        assert!(isolator.active_sub_contexts.is_empty());
        assert_eq!(isolator.next_context_id, 0);
    }

    #[test]
    fn test_update_main_context() {
        let mut isolator = ContextIsolator::new();
        let messages = vec![
            SimpleMessage::new("user", "hello"),
            SimpleMessage::new("assistant", "hi there"),
        ];

        isolator.update_main_context(messages.clone());
        assert_eq!(isolator.main_messages.len(), 2);
        assert_eq!(isolator.main_messages[0].content(), "hello");
        assert_eq!(isolator.main_messages[1].content(), "hi there");
    }

    #[test]
    fn test_create_sub_context_minimal_inheritance() {
        let mut isolator = ContextIsolator::new();
        let messages = vec![
            SimpleMessage::new("user", "msg1"),
            SimpleMessage::new("assistant", "msg2"),
            SimpleMessage::new("user", "msg3"),
            SimpleMessage::new("assistant", "msg4"),
        ];
        isolator.update_main_context(messages);

        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);
        let context_id = isolator.create_sub_context(&agent, "test task").unwrap();

        assert_eq!(context_id, "sub_ctx_0");
        assert_eq!(isolator.next_context_id, 1);

        let sub_context = isolator.get_sub_context(&context_id).unwrap();
        assert_eq!(sub_context.agent_name, "test_agent");
        assert_eq!(sub_context.task_description, "test task");
        assert_eq!(sub_context.inherited_messages.len(), 3); // Last 3 messages
    }

    #[test]
    fn test_create_sub_context_none_inheritance() {
        let mut isolator = ContextIsolator::new();
        let messages = vec![SimpleMessage::new("user", "test")];
        isolator.update_main_context(messages);

        let agent = create_test_agent("test_agent", ContextInheritanceLevel::None);
        let context_id = isolator.create_sub_context(&agent, "test task").unwrap();

        let sub_context = isolator.get_sub_context(&context_id).unwrap();
        assert!(sub_context.inherited_messages.is_empty());
    }

    #[test]
    fn test_create_sub_context_partial_inheritance() {
        let mut isolator = ContextIsolator::new();
        let messages = vec![
            SimpleMessage::new("user", "hello world"),
            SimpleMessage::new("assistant", "testing code"),
            SimpleMessage::new("user", "debug this"),
        ];
        isolator.update_main_context(messages);

        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Partial);
        let context_id = isolator.create_sub_context(&agent, "debug testing").unwrap();

        let sub_context = isolator.get_sub_context(&context_id).unwrap();
        // Should match messages containing "testing" or "debug"
        assert!(!sub_context.inherited_messages.is_empty());
    }

    #[test]
    fn test_create_sub_context_full_inheritance() {
        let mut isolator = ContextIsolator::new();
        let messages = vec![
            SimpleMessage::new("user", "msg1"),
            SimpleMessage::new("assistant", "msg2"),
        ];
        isolator.update_main_context(messages);

        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Full);
        let context_id = isolator.create_sub_context(&agent, "test task").unwrap();

        let sub_context = isolator.get_sub_context(&context_id).unwrap();
        assert_eq!(sub_context.inherited_messages.len(), 2);
    }

    #[test]
    fn test_merge_sub_context_result() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);
        let context_id = isolator.create_sub_context(&agent, "test task").unwrap();

        let initial_count = isolator.main_messages.len();
        let result = SimpleMessage::new("assistant", "task completed");
        isolator.merge_sub_context_result(&context_id, result).unwrap();

        assert_eq!(isolator.main_messages.len(), initial_count + 1);
        assert_eq!(isolator.main_messages.last().unwrap().content(), "task completed");
        assert!(isolator.get_sub_context(&context_id).is_none());
    }

    #[test]
    fn test_merge_sub_context_result_invalid_id() {
        let mut isolator = ContextIsolator::new();
        let result = SimpleMessage::new("assistant", "test");
        let error = isolator.merge_sub_context_result("invalid_id", result);
        assert!(error.is_err());
    }

    #[test]
    fn test_list_active_contexts() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);

        assert!(isolator.list_active_contexts().is_empty());

        isolator.create_sub_context(&agent, "task1").unwrap();
        isolator.create_sub_context(&agent, "task2").unwrap();

        assert_eq!(isolator.list_active_contexts().len(), 2);
    }

    #[test]
    fn test_cleanup_context() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);
        let context_id = isolator.create_sub_context(&agent, "test task").unwrap();

        assert!(isolator.get_sub_context(&context_id).is_some());
        let cleaned = isolator.cleanup_context(&context_id);
        assert!(cleaned.is_some());
        assert!(isolator.get_sub_context(&context_id).is_none());
    }

    #[test]
    fn test_create_isolated_context() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);

        let context_id = isolator.create_isolated_context(&agent, "test task").unwrap();
        assert!(isolator.get_sub_context(&context_id).is_some());
    }

    #[test]
    fn test_merge_context_results() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);
        let context_id = isolator.create_isolated_context(&agent, "test task").unwrap();

        let initial_count = isolator.main_messages.len();
        let results = vec![
            SimpleMessage::new("assistant", "result 1"),
            SimpleMessage::new("assistant", "result 2"),
        ];

        isolator.merge_context_results(&context_id, results).unwrap();
        assert_eq!(isolator.main_messages.len(), initial_count + 2);
        assert!(isolator.get_sub_context(&context_id).is_none());
    }

    #[test]
    fn test_get_context_size() {
        let mut isolator = ContextIsolator::new();
        assert_eq!(isolator.get_context_size(), 0);

        let messages = vec![SimpleMessage::new("user", "test")];
        isolator.update_main_context(messages);
        assert_eq!(isolator.get_context_size(), 1);
    }

    #[test]
    fn test_cleanup_old_contexts() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);

        // Create 3 contexts
        isolator.create_isolated_context(&agent, "task1").unwrap();
        isolator.create_isolated_context(&agent, "task2").unwrap();
        isolator.create_isolated_context(&agent, "task3").unwrap();

        assert_eq!(isolator.active_sub_contexts.len(), 3);

        // Cleanup to keep only 2
        isolator.cleanup_old_contexts(2);
        assert_eq!(isolator.active_sub_contexts.len(), 2);
    }

    #[test]
    fn test_context_state_management() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Full);

        // Set up main context
        let main_messages = vec![
            SimpleMessage::new("user", "initial message"),
            SimpleMessage::new("assistant", "initial response"),
        ];
        isolator.update_main_context(main_messages);

        // Create sub-context
        let context_id = isolator.create_isolated_context(&agent, "test task").unwrap();
        let sub_context = isolator.get_sub_context(&context_id).unwrap();

        // Verify inheritance
        assert_eq!(sub_context.inherited_messages.len(), 2);
        assert_eq!(sub_context.inherited_messages[0].content(), "initial message");

        // Test merging results
        let results = vec![
            SimpleMessage::new("assistant", "sub-task result 1"),
            SimpleMessage::new("assistant", "sub-task result 2"),
        ];

        let initial_main_count = isolator.main_messages.len();
        isolator.merge_context_results(&context_id, results).unwrap();

        // Verify results merged back
        assert_eq!(isolator.main_messages.len(), initial_main_count + 2);
        assert_eq!(isolator.main_messages.last().unwrap().content(), "sub-task result 2");

        // Verify sub-context cleaned up
        assert!(isolator.get_sub_context(&context_id).is_none());
    }

    #[test]
    fn test_context_inheritance_consistency() {
        let mut isolator = ContextIsolator::new();

        // Set up complex main context
        let main_messages = vec![
            SimpleMessage::new("user", "debug this function"),
            SimpleMessage::new("assistant", "I'll help debug"),
            SimpleMessage::new("user", "also review the code"),
            SimpleMessage::new("assistant", "reviewing now"),
            SimpleMessage::new("user", "test the changes"),
        ];
        isolator.update_main_context(main_messages);

        // Test different inheritance levels
        let test_cases = vec![
            (ContextInheritanceLevel::None, 0),
            (ContextInheritanceLevel::Minimal, 3), // Last 3 messages
            (ContextInheritanceLevel::Full, 5),    // All messages
        ];

        for (level, expected_count) in test_cases {
            let agent = create_test_agent("test_agent", level.clone());
            let context_id = isolator.create_isolated_context(&agent, "debug testing").unwrap();
            let sub_context = isolator.get_sub_context(&context_id).unwrap();

            assert_eq!(
                sub_context.inherited_messages.len(),
                expected_count,
                "Inheritance level {:?} should inherit {} messages",
                level,
                expected_count
            );

            isolator.cleanup_context(&context_id);
        }
    }

    #[test]
    fn test_partial_inheritance_filtering() {
        let mut isolator = ContextIsolator::new();

        // Set up main context with mixed content
        let main_messages = vec![
            SimpleMessage::new("user", "hello world"),
            SimpleMessage::new("assistant", "hi there"),
            SimpleMessage::new("user", "debug this function please"),
            SimpleMessage::new("assistant", "debugging now"),
            SimpleMessage::new("user", "review the code changes"),
            SimpleMessage::new("assistant", "code looks good"),
            SimpleMessage::new("user", "unrelated chat message"),
        ];
        isolator.update_main_context(main_messages);

        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Partial);
        let context_id = isolator.create_isolated_context(&agent, "debug and review").unwrap();
        let sub_context = isolator.get_sub_context(&context_id).unwrap();

        // Should only inherit messages containing "debug" or "review" keywords
        assert!(sub_context.inherited_messages.len() > 0);
        assert!(sub_context.inherited_messages.len() < 7); // Not all messages

        // Verify filtered messages contain relevant keywords
        for msg in &sub_context.inherited_messages {
            let content = msg.content().to_lowercase();
            assert!(
                content.contains("debug") || content.contains("review") || content.contains("code"),
                "Inherited message should contain relevant keywords: {}",
                msg.content()
            );
        }
    }

    #[test]
    fn test_concurrent_context_management() {
        let mut isolator = ContextIsolator::new();
        let agent = create_test_agent("test_agent", ContextInheritanceLevel::Minimal);

        // Create multiple concurrent contexts
        let context1 = isolator.create_isolated_context(&agent, "task1").unwrap();
        let context2 = isolator.create_isolated_context(&agent, "task2").unwrap();
        let context3 = isolator.create_isolated_context(&agent, "task3").unwrap();

        assert_eq!(isolator.active_sub_contexts.len(), 3);

        // Verify each context is independent
        let ctx1 = isolator.get_sub_context(&context1).unwrap();
        let ctx2 = isolator.get_sub_context(&context2).unwrap();
        let ctx3 = isolator.get_sub_context(&context3).unwrap();

        assert_eq!(ctx1.task_description, "task1");
        assert_eq!(ctx2.task_description, "task2");
        assert_eq!(ctx3.task_description, "task3");

        // Merge one context
        let result = SimpleMessage::new("assistant", "task1 completed");
        isolator.merge_sub_context_result(&context1, result).unwrap();

        // Verify only one context was removed
        assert_eq!(isolator.active_sub_contexts.len(), 2);
        assert!(isolator.get_sub_context(&context1).is_none());
        assert!(isolator.get_sub_context(&context2).is_some());
        assert!(isolator.get_sub_context(&context3).is_some());
    }
}
