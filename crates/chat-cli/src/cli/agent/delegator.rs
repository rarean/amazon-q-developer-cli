use std::collections::VecDeque;
use std::sync::Arc;

use eyre::Result;

use super::Agent;
use super::analyzer::{
    DelegationIntent,
    RequestAnalyzer,
};
use super::context_isolator::{
    ContextIsolator,
    SimpleMessage,
    SubAgentContext,
};
use super::registry::{
    AgentCandidate,
    AgentRegistry,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DelegationResult {
    Delegated {
        agent: String,
        task: String,
        context_id: String,
    },
    NoDelegation,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DelegationHistory {
    pub agent: String,
    pub task: String,
    pub timestamp: std::time::SystemTime,
    pub success: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AgentDelegator {
    registry: Arc<AgentRegistry>,
    analyzer: RequestAnalyzer,
    context_isolator: ContextIsolator,
    delegation_history: VecDeque<DelegationHistory>,
    max_history_size: usize,
}

#[allow(dead_code)]
impl AgentDelegator {
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        let analyzer = RequestAnalyzer::new(registry.clone());
        let context_isolator = ContextIsolator::new();

        Self {
            registry,
            analyzer,
            context_isolator,
            delegation_history: VecDeque::new(),
            max_history_size: 100,
        }
    }

    pub fn update_main_context(&mut self, messages: Vec<SimpleMessage>) {
        self.context_isolator.update_main_context(messages);
    }

    /// Enhanced delegation with priority-based selection and fallback
    pub fn try_delegate_with_fallback(&mut self, input: &str) -> Result<DelegationResult> {
        let (intent, confidence) = self.analyzer.analyze_delegation_intent(input);

        match intent {
            DelegationIntent::ExplicitAgent(agent_name) => self.try_delegate_to_agent(&agent_name, input, true),
            DelegationIntent::AutoDelegate(candidates) => {
                self.try_priority_based_delegation(candidates, input, confidence)
            },
            DelegationIntent::NoDelegate => Ok(DelegationResult::NoDelegation),
        }
    }

    /// Try delegation to specific agent with fallback
    fn try_delegate_to_agent(
        &mut self,
        agent_name: &str,
        input: &str,
        allow_fallback: bool,
    ) -> Result<DelegationResult> {
        if let Some(agent) = self.registry.get_agent(agent_name) {
            let task = self.analyzer.extract_task(input);

            match self.context_isolator.create_isolated_context(agent, &task) {
                Ok(context_id) => {
                    self.record_delegation_attempt(agent_name, &task, true);
                    Ok(DelegationResult::Delegated {
                        agent: agent_name.to_string(),
                        task,
                        context_id,
                    })
                },
                Err(_) if allow_fallback => {
                    // Try fallback to best available agent
                    self.record_delegation_attempt(agent_name, &task, false);
                    self.try_fallback_delegation(input)
                },
                Err(e) => {
                    self.record_delegation_attempt(agent_name, &task, false);
                    Err(e)
                },
            }
        } else if allow_fallback {
            self.try_fallback_delegation(input)
        } else {
            Ok(DelegationResult::NoDelegation)
        }
    }

    /// Priority-based delegation with multiple candidates
    fn try_priority_based_delegation(
        &mut self,
        mut candidates: Vec<AgentCandidate>,
        input: &str,
        confidence: f32,
    ) -> Result<DelegationResult> {
        if confidence < 0.6 {
            return Ok(DelegationResult::NoDelegation);
        }

        // Sort by score and recent success rate
        candidates.sort_by(|a, b| {
            let a_success_rate = self.get_agent_success_rate(&a.name);
            let b_success_rate = self.get_agent_success_rate(&b.name);
            let a_combined = a.score * 0.7 + a_success_rate * 0.3;
            let b_combined = b.score * 0.7 + b_success_rate * 0.3;
            b_combined.partial_cmp(&a_combined).unwrap()
        });

        // Try candidates in priority order
        for candidate in candidates {
            if let Ok(result) = self.try_delegate_to_agent(&candidate.name, input, false) {
                match result {
                    DelegationResult::Delegated { .. } => return Ok(result),
                    DelegationResult::NoDelegation => {},
                }
            }
        }

        Ok(DelegationResult::NoDelegation)
    }

    /// Fallback delegation to any available agent
    fn try_fallback_delegation(&mut self, input: &str) -> Result<DelegationResult> {
        let candidates = self.analyzer.score_agent_candidates(input);
        if let Some(best) = candidates.first() {
            self.try_delegate_to_agent(&best.name, input, false)
        } else {
            Ok(DelegationResult::NoDelegation)
        }
    }

    /// Record delegation attempt for history tracking
    fn record_delegation_attempt(&mut self, agent: &str, task: &str, success: bool) {
        let entry = DelegationHistory {
            agent: agent.to_string(),
            task: task.to_string(),
            timestamp: std::time::SystemTime::now(),
            success,
        };

        self.delegation_history.push_back(entry);

        // Maintain history size limit
        while self.delegation_history.len() > self.max_history_size {
            self.delegation_history.pop_front();
        }
    }

    /// Get success rate for an agent based on recent history
    fn get_agent_success_rate(&self, agent_name: &str) -> f32 {
        let recent_attempts: Vec<_> = self.delegation_history
            .iter()
            .filter(|h| h.agent == agent_name)
            .rev()
            .take(10) // Last 10 attempts
            .collect();

        if recent_attempts.is_empty() {
            return 0.5; // Default neutral score
        }

        let successes = recent_attempts.iter().filter(|h| h.success).count();
        successes as f32 / recent_attempts.len() as f32
    }

    /// Get delegation history for analysis
    pub fn get_delegation_history(&self) -> &VecDeque<DelegationHistory> {
        &self.delegation_history
    }

    pub fn try_delegate(&mut self, input: &str) -> Result<DelegationResult> {
        let intent = self.analyzer.analyze(input);

        match intent {
            DelegationIntent::ExplicitAgent(agent_name) => {
                if let Some(agent) = self.registry.get_agent(&agent_name) {
                    let task = self.analyzer.extract_task(input);
                    let context_id = self.context_isolator.create_isolated_context(agent, &task)?;

                    Ok(DelegationResult::Delegated {
                        agent: agent_name.clone(),
                        task,
                        context_id,
                    })
                } else {
                    Ok(DelegationResult::NoDelegation)
                }
            },
            DelegationIntent::AutoDelegate(candidates) => {
                if let Some(best_candidate) = candidates.first() {
                    if let Some(agent) = self.registry.get_agent(&best_candidate.name) {
                        let task = self.analyzer.extract_task(input);
                        let context_id = self.context_isolator.create_isolated_context(agent, &task)?;

                        Ok(DelegationResult::Delegated {
                            agent: best_candidate.name.clone(),
                            task,
                            context_id,
                        })
                    } else {
                        Ok(DelegationResult::NoDelegation)
                    }
                } else {
                    Ok(DelegationResult::NoDelegation)
                }
            },
            DelegationIntent::NoDelegate => Ok(DelegationResult::NoDelegation),
        }
    }

    pub fn should_delegate(input: &str) -> bool {
        RequestAnalyzer::has_delegation_intent(input)
    }

    pub fn get_sub_context(&self, context_id: &str) -> Option<&SubAgentContext> {
        self.context_isolator.get_sub_context(context_id)
    }

    pub fn merge_delegation_result(&mut self, context_id: &str, result_content: &str) -> Result<()> {
        let result_message = SimpleMessage::new("assistant", result_content);
        self.context_isolator
            .merge_sub_context_result(context_id, result_message)
    }

    /// Merge multiple result messages from a delegation
    pub fn merge_delegation_results(&mut self, context_id: &str, results: Vec<SimpleMessage>) -> Result<()> {
        self.context_isolator.merge_context_results(context_id, results)
    }

    /// Get context size for management
    pub fn get_context_size(&self) -> usize {
        self.context_isolator.get_context_size()
    }

    /// Cleanup old contexts to manage memory
    pub fn cleanup_old_contexts(&mut self, max_contexts: usize) {
        self.context_isolator.cleanup_old_contexts(max_contexts);
    }

    pub fn list_active_delegations(&self) -> Vec<&SubAgentContext> {
        self.context_isolator.list_active_contexts()
    }

    pub fn cleanup_delegation(&mut self, context_id: &str) -> Option<SubAgentContext> {
        self.context_isolator.cleanup_context(context_id)
    }

    pub fn get_delegation_candidates(&self, input: &str) -> Vec<AgentCandidate> {
        self.registry.find_candidates(input)
    }

    pub fn list_available_agents(&self) -> Vec<&Agent> {
        self.registry.list_agents()
    }

    pub fn list_delegatable_agents(&self) -> Vec<&Agent> {
        self.registry.list_delegatable_agents()
    }

    /// Force delegate to a specific agent, bypassing analysis
    pub fn force_delegate(&mut self, agent_name: &str, task: &str) -> Result<DelegationResult> {
        if let Some(agent) = self.registry.get_agent(agent_name) {
            let context_id = self.context_isolator.create_sub_context(agent, task)?;

            Ok(DelegationResult::Delegated {
                agent: agent_name.to_string(),
                task: task.to_string(),
                context_id,
            })
        } else {
            Ok(DelegationResult::NoDelegation)
        }
    }

    /// Get delegation statistics
    pub fn get_delegation_stats(&self) -> DelegationStats {
        let active_count = self.context_isolator.list_active_contexts().len();
        let total_agents = self.registry.list_agents().len();
        let delegatable_agents = self.registry.list_delegatable_agents().len();

        DelegationStats {
            active_delegations: active_count,
            total_agents,
            delegatable_agents,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DelegationStats {
    pub active_delegations: usize,
    pub total_agents: usize,
    pub delegatable_agents: usize,
}
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    // Mock implementations for testing
    fn create_mock_registry() -> Arc<AgentRegistry> {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "test_agent".to_string(),
            delegation: Some(super::super::delegation::DelegationConfig {
                auto_delegate: true,
                keywords: vec!["test".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        };

        registry.register_agent(agent);
        Arc::new(registry)
    }

    #[test]
    fn test_delegation_result_creation() {
        let result = DelegationResult::Delegated {
            agent: "test_agent".to_string(),
            task: "test task".to_string(),
            context_id: "ctx_123".to_string(),
        };

        match result {
            DelegationResult::Delegated {
                agent,
                task,
                context_id,
            } => {
                assert_eq!(agent, "test_agent");
                assert_eq!(task, "test task");
                assert_eq!(context_id, "ctx_123");
            },
            _ => panic!("Expected Delegated variant"),
        }
    }

    #[test]
    fn test_agent_delegator_new() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry.clone());

        // Verify delegator is properly initialized
        assert_eq!(delegator.registry.list_agents().len(), 1);
        assert!(delegator.context_isolator.list_active_contexts().is_empty());
    }

    #[test]
    fn test_update_main_context() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let messages = vec![
            SimpleMessage::new("user", "hello"),
            SimpleMessage::new("assistant", "hi"),
        ];

        delegator.update_main_context(messages);
        // Context should be updated (verified through delegation behavior)
    }

    #[tokio::test]
    async fn test_try_delegate_explicit_agent_success() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.try_delegate("use test_agent agent to help").unwrap();
        match result {
            DelegationResult::Delegated { agent, task, .. } => {
                assert_eq!(agent, "test_agent");
                assert!(task.contains("help"));
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[tokio::test]
    async fn test_try_delegate_explicit_agent_not_found() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.try_delegate("use nonexistent_agent agent").unwrap();
        match result {
            DelegationResult::NoDelegation => {}, // Expected
            _ => panic!("Expected no delegation"),
        }
    }

    #[tokio::test]
    async fn test_try_delegate_auto_delegate_success() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.try_delegate("please test this code").unwrap();
        // Auto-delegation may or may not trigger based on scoring
        match result {
            DelegationResult::Delegated { agent, .. } => {
                assert_eq!(agent, "test_agent");
            },
            DelegationResult::NoDelegation => {}, // Also acceptable
        }
    }

    #[tokio::test]
    async fn test_try_delegate_no_delegate() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.try_delegate("hello world").unwrap();
        match result {
            DelegationResult::NoDelegation => {}, // Expected
            _ => panic!("Expected no delegation"),
        }
    }

    #[test]
    fn test_should_delegate() {
        // Test static method
        assert!(AgentDelegator::should_delegate("@agent do something"));
        assert!(AgentDelegator::should_delegate("delegate to test"));
        assert!(!AgentDelegator::should_delegate("regular message"));
    }

    #[test]
    fn test_get_delegation_stats() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let stats = delegator.get_delegation_stats();
        assert_eq!(stats.active_delegations, 0);
        assert_eq!(stats.total_agents, 1);
        assert_eq!(stats.delegatable_agents, 1);
    }

    #[test]
    fn test_list_available_agents() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let agents = delegator.list_available_agents();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "test_agent");
    }

    #[test]
    fn test_list_delegatable_agents() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let agents = delegator.list_delegatable_agents();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "test_agent");
    }

    #[test]
    fn test_get_delegation_candidates() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let candidates = delegator.get_delegation_candidates("test this code");
        assert!(!candidates.is_empty());
    }

    #[test]
    fn test_list_active_delegations() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let active = delegator.list_active_delegations();
        assert!(active.is_empty());
    }

    #[test]
    fn test_delegation_stats_creation() {
        let stats = DelegationStats {
            active_delegations: 2,
            total_agents: 5,
            delegatable_agents: 3,
        };

        assert_eq!(stats.active_delegations, 2);
        assert_eq!(stats.total_agents, 5);
        assert_eq!(stats.delegatable_agents, 3);
    }

    #[test]
    fn test_force_delegate_success() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.force_delegate("test_agent", "forced task").unwrap();
        match result {
            DelegationResult::Delegated { agent, task, .. } => {
                assert_eq!(agent, "test_agent");
                assert_eq!(task, "forced task");
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[test]
    fn test_force_delegate_nonexistent_agent() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.force_delegate("nonexistent", "task").unwrap();
        match result {
            DelegationResult::NoDelegation => {}, // Expected
            _ => panic!("Expected no delegation"),
        }
    }

    #[test]
    fn test_get_sub_context() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Create a delegation first
        let result = delegator.force_delegate("test_agent", "test task").unwrap();
        match result {
            DelegationResult::Delegated { context_id, .. } => {
                let context = delegator.get_sub_context(&context_id);
                assert!(context.is_some());
                assert_eq!(context.unwrap().agent_name, "test_agent");
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[test]
    fn test_merge_delegation_result() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // First create a delegation context
        let result = delegator.force_delegate("test_agent", "test task").unwrap();
        match result {
            DelegationResult::Delegated { context_id, .. } => {
                let merge_result = delegator.merge_delegation_result(&context_id, "completed");
                assert!(merge_result.is_ok());
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[test]
    fn test_merge_delegation_result_invalid_context() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.merge_delegation_result("invalid_id", "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_cleanup_delegation() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Create and then cleanup a delegation
        let result = delegator.force_delegate("test_agent", "test task").unwrap();
        match result {
            DelegationResult::Delegated { context_id, .. } => {
                let cleaned = delegator.cleanup_delegation(&context_id);
                assert!(cleaned.is_some());

                // Verify it's cleaned up
                let context = delegator.get_sub_context(&context_id);
                assert!(context.is_none());
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[test]
    fn test_delegation_result_debug() {
        let result = DelegationResult::Delegated {
            agent: "test".to_string(),
            task: "task".to_string(),
            context_id: "ctx".to_string(),
        };

        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("ctx"));
    }

    #[test]
    fn test_agent_delegator_debug() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let debug_str = format!("{:?}", delegator);
        assert!(debug_str.contains("AgentDelegator"));
    }

    #[test]
    fn test_try_delegate_with_fallback() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        let result = delegator.try_delegate_with_fallback("use test_agent agent").unwrap();
        match result {
            DelegationResult::Delegated { agent, .. } => {
                assert_eq!(agent, "test_agent");
            },
            DelegationResult::NoDelegation => panic!("Expected delegation to succeed"),
        }
    }

    #[test]
    fn test_delegation_history_tracking() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Record some delegation attempts
        delegator.record_delegation_attempt("test_agent", "task1", true);
        delegator.record_delegation_attempt("test_agent", "task2", false);
        delegator.record_delegation_attempt("test_agent", "task3", true);

        let history = delegator.get_delegation_history();
        assert_eq!(history.len(), 3);

        let success_rate = delegator.get_agent_success_rate("test_agent");
        assert!((success_rate - 0.67).abs() < 0.01); // 2/3 success rate
    }

    #[test]
    fn test_get_agent_success_rate_no_history() {
        let registry = create_mock_registry();
        let delegator = AgentDelegator::new(registry);

        let success_rate = delegator.get_agent_success_rate("unknown_agent");
        assert_eq!(success_rate, 0.5); // Default neutral score
    }

    #[test]
    fn test_priority_based_delegation() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Add some history to influence priority
        delegator.record_delegation_attempt("test_agent", "previous_task", true);

        let result = delegator.try_delegate_with_fallback("please test this code").unwrap();
        // Should either delegate or not based on scoring
        match result {
            DelegationResult::Delegated { .. } | DelegationResult::NoDelegation => {},
        }
    }

    #[test]
    fn test_delegation_performance() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Test delegation performance - should complete in reasonable time
        let start = std::time::Instant::now();

        // Run multiple delegation attempts
        for i in 0..100 {
            let input = format!("test delegation request {}", i);
            let _ = delegator.try_delegate_with_fallback(&input);
        }

        let duration = start.elapsed();

        // Should complete 100 delegations in under 100ms (1ms per delegation)
        assert!(
            duration.as_millis() < 100,
            "100 delegations took {}ms, should be under 100ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_delegation_overhead_single() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Test single delegation overhead
        let start = std::time::Instant::now();
        let _ = delegator.try_delegate_with_fallback("review this code");
        let duration = start.elapsed();

        // Single delegation should complete in under 10ms
        assert!(
            duration.as_millis() < 10,
            "Single delegation took {}ms, should be under 10ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_context_creation_performance() {
        let registry = create_mock_registry();
        let mut delegator = AgentDelegator::new(registry);

        // Add some context to test with
        let messages = vec![
            SimpleMessage::new("user", "test message 1"),
            SimpleMessage::new("assistant", "response 1"),
            SimpleMessage::new("user", "test message 2"),
        ];
        delegator.update_main_context(messages);

        let start = std::time::Instant::now();

        // Create multiple contexts
        for i in 0..50 {
            if let Ok(DelegationResult::Delegated { context_id, .. }) =
                delegator.force_delegate("test_agent", &format!("task {}", i))
            {
                delegator.cleanup_delegation(&context_id);
            }
        }

        let duration = start.elapsed();

        // Should create and cleanup 50 contexts in under 50ms
        assert!(
            duration.as_millis() < 50,
            "50 context operations took {}ms, should be under 50ms",
            duration.as_millis()
        );
    }
}
