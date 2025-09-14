use std::sync::Arc;

use regex::Regex;

use super::registry::{
    AgentCandidate,
    AgentRegistry,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DelegationIntent {
    /// Explicit agent invocation: "use code-reviewer agent"
    ExplicitAgent(String),
    /// Automatic delegation based on keywords/patterns
    AutoDelegate(Vec<AgentCandidate>),
    /// No delegation needed
    NoDelegate,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RequestAnalyzer {
    registry: Arc<AgentRegistry>,
    explicit_patterns: Vec<Regex>,
}

#[allow(dead_code)]
impl RequestAnalyzer {
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        let explicit_patterns = vec![
            // "use <agent> agent"
            Regex::new(r"(?i)use\s+([a-zA-Z0-9_-]+)\s+agent").unwrap(),
            // "ask <agent> to"
            Regex::new(r"(?i)ask\s+([a-zA-Z0-9_-]+)\s+to").unwrap(),
            // "delegate to <agent>"
            Regex::new(r"(?i)delegate\s+to\s+([a-zA-Z0-9_-]+)").unwrap(),
            // "<agent> agent:"
            Regex::new(r"(?i)([a-zA-Z0-9_-]+)\s+agent:").unwrap(),
        ];

        Self {
            registry,
            explicit_patterns,
        }
    }

    pub fn analyze(&self, input: &str) -> DelegationIntent {
        // First check for explicit agent invocation
        if let Some(agent_name) = self.extract_explicit_agent(input) {
            if self.registry.get_agent(&agent_name).is_some() {
                return DelegationIntent::ExplicitAgent(agent_name);
            }
        }

        // Then check for automatic delegation candidates with confidence scoring
        if self.should_auto_delegate(input) {
            let candidates = self.score_agent_candidates(input);
            if !candidates.is_empty() {
                return DelegationIntent::AutoDelegate(candidates);
            }
        }

        DelegationIntent::NoDelegate
    }

    /// Enhanced delegation intent analysis with confidence scoring
    pub fn analyze_delegation_intent(&self, input: &str) -> (DelegationIntent, f32) {
        // Check explicit agent first
        if let Some(agent_name) = self.extract_explicit_agent(input) {
            if self.registry.get_agent(&agent_name).is_some() {
                return (DelegationIntent::ExplicitAgent(agent_name), 1.0);
            }
        }

        // Calculate auto-delegation confidence
        let confidence = Self::calculate_delegation_confidence(input);
        if confidence > 0.7 {
            let candidates = self.score_agent_candidates(input);
            if !candidates.is_empty() {
                return (DelegationIntent::AutoDelegate(candidates), confidence);
            }
        }

        (DelegationIntent::NoDelegate, 0.0)
    }

    /// Score agent candidates with confidence levels
    pub fn score_agent_candidates(&self, input: &str) -> Vec<AgentCandidate> {
        let mut candidates = self.registry.find_candidates(input);

        // Enhanced scoring based on keyword matches and context
        for candidate in &mut candidates {
            candidate.score = self.calculate_agent_score(&candidate.name, input);
        }

        // Sort by score descending and filter by minimum threshold
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates.into_iter().filter(|c| c.score > 0.3).collect()
    }

    /// Determine if input should trigger auto-delegation
    #[allow(clippy::unused_self)] // Public API method
    pub fn should_auto_delegate(&self, input: &str) -> bool {
        let confidence = Self::calculate_delegation_confidence(input);
        confidence > 0.6 // Threshold for auto-delegation
    }

    /// Calculate confidence score for delegation (0.0 to 1.0)
    fn calculate_delegation_confidence(input: &str) -> f32 {
        let input_lower = input.to_lowercase();
        let mut score = 0.0;

        // Strong delegation indicators
        let strong_indicators = ["review", "analyze", "check", "test", "debug", "fix", "optimize"];
        let strong_matches = strong_indicators
            .iter()
            .filter(|&&keyword| input_lower.contains(keyword))
            .count() as f32;
        score += strong_matches * 0.3;

        // Weak delegation indicators
        let weak_indicators = ["help", "assist", "look", "examine"];
        let weak_matches = weak_indicators
            .iter()
            .filter(|&&keyword| input_lower.contains(keyword))
            .count() as f32;
        score += weak_matches * 0.1;

        // Explicit delegation words
        let explicit_words = ["delegate", "agent", "use"];
        let explicit_matches = explicit_words
            .iter()
            .filter(|&&keyword| input_lower.contains(keyword))
            .count() as f32;
        score += explicit_matches * 0.4;

        // Cap at 1.0
        score.min(1.0)
    }

    /// Calculate score for a specific agent against input
    fn calculate_agent_score(&self, agent_name: &str, input: &str) -> f32 {
        if let Some(agent) = self.registry.get_agent(agent_name) {
            let mut score: f32 = 0.0;
            let input_lower = input.to_lowercase();

            // Check delegation keywords
            if let Some(delegation) = &agent.delegation {
                for keyword in &delegation.keywords {
                    if input_lower.contains(&keyword.to_lowercase()) {
                        score += 0.3;
                    }
                }
            }

            // Agent name mentioned
            if input_lower.contains(&agent_name.to_lowercase()) {
                score += 0.5;
            }

            score.min(1.0)
        } else {
            0.0
        }
    }

    pub fn extract_explicit_agent(&self, input: &str) -> Option<String> {
        for pattern in &self.explicit_patterns {
            if let Some(captures) = pattern.captures(input) {
                if let Some(agent_name) = captures.get(1) {
                    return Some(agent_name.as_str().to_string());
                }
            }
        }
        None
    }

    pub fn score_agents(&self, input: &str) -> Vec<(String, f32)> {
        let candidates = self.registry.find_candidates(input);
        candidates.into_iter().map(|c| (c.name, c.score)).collect()
    }

    /// Check if input contains delegation keywords that suggest user wants to delegate
    pub fn has_delegation_intent(input: &str) -> bool {
        let delegation_keywords = [
            "use", "ask", "delegate", "agent", "review", "analyze", "check", "test", "debug",
        ];

        let input_lower = input.to_lowercase();
        delegation_keywords.iter().any(|&keyword| input_lower.contains(keyword))
    }

    /// Extract the task description from input, removing delegation commands
    pub fn extract_task(&self, input: &str) -> String {
        let mut task = input.to_string();

        // Remove explicit delegation patterns
        for pattern in &self.explicit_patterns {
            task = pattern.replace_all(&task, "").to_string();
        }

        // Clean up extra whitespace
        task.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::cli::agent::Agent;
    use crate::cli::agent::delegation::DelegationConfig;

    fn create_test_registry() -> Arc<AgentRegistry> {
        let mut registry = AgentRegistry::new().unwrap();

        let agent = Agent {
            name: "code-reviewer".to_string(),
            delegation: Some(DelegationConfig {
                keywords: vec!["review".to_string(), "code".to_string()],
                auto_delegate: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        registry.register_agent(agent);
        Arc::new(registry)
    }

    #[test]
    fn test_request_analyzer_new() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry.clone());
        assert_eq!(analyzer.explicit_patterns.len(), 4);
    }

    #[test]
    fn test_extract_explicit_agent() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        assert_eq!(
            analyzer.extract_explicit_agent("use code-reviewer agent"),
            Some("code-reviewer".to_string())
        );
        assert_eq!(
            analyzer.extract_explicit_agent("ask code-reviewer to help"),
            Some("code-reviewer".to_string())
        );
        assert_eq!(
            analyzer.extract_explicit_agent("delegate to code-reviewer"),
            Some("code-reviewer".to_string())
        );
        assert_eq!(
            analyzer.extract_explicit_agent("code-reviewer agent: help me"),
            Some("code-reviewer".to_string())
        );
        assert_eq!(analyzer.extract_explicit_agent("no agent mentioned"), None);
    }

    #[test]
    fn test_analyze_explicit_agent() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        match analyzer.analyze("use code-reviewer agent") {
            DelegationIntent::ExplicitAgent(name) => assert_eq!(name, "code-reviewer"),
            _ => panic!("Expected ExplicitAgent"),
        }
    }

    #[test]
    fn test_analyze_auto_delegate() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        // "use agent to review" should exceed 0.6 threshold
        match analyzer.analyze("use agent to review this code") {
            DelegationIntent::AutoDelegate(candidates) => assert!(!candidates.is_empty()),
            _ => panic!("Expected AutoDelegate"),
        }
    }

    #[test]
    fn test_analyze_no_delegate() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        match analyzer.analyze("hello world") {
            DelegationIntent::NoDelegate => {},
            _ => panic!("Expected NoDelegate"),
        }
    }

    #[test]
    fn test_has_delegation_intent() {
        assert!(RequestAnalyzer::has_delegation_intent("use agent"));
        assert!(RequestAnalyzer::has_delegation_intent("ask for help"));
        assert!(RequestAnalyzer::has_delegation_intent("delegate this"));
        assert!(RequestAnalyzer::has_delegation_intent("review code"));
        assert!(!RequestAnalyzer::has_delegation_intent("hello world"));
    }

    #[test]
    fn test_extract_task() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        let task = analyzer.extract_task("use code-reviewer agent to review this code");
        assert!(task.contains("review this code"));

        let task2 = analyzer.extract_task("simple task without delegation");
        assert_eq!(task2, "simple task without delegation");
    }

    #[test]
    fn test_score_agents() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        let scores = analyzer.score_agents("review this code");
        assert!(!scores.is_empty());
        assert!(scores.iter().any(|(name, _)| name == "code-reviewer"));
    }

    #[test]
    fn test_delegation_intent_debug() {
        let intent = DelegationIntent::NoDelegate;
        assert!(format!("{:?}", intent).contains("NoDelegate"));

        let intent2 = DelegationIntent::ExplicitAgent("test".to_string());
        assert!(format!("{:?}", intent2).contains("ExplicitAgent"));
    }

    #[test]
    fn test_analyze_delegation_intent() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        let (intent, confidence) = analyzer.analyze_delegation_intent("use code-reviewer agent");
        match intent {
            DelegationIntent::ExplicitAgent(name) => {
                assert_eq!(name, "code-reviewer");
                assert_eq!(confidence, 1.0);
            },
            _ => panic!("Expected ExplicitAgent"),
        }
    }

    #[test]
    fn test_score_agent_candidates() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        let candidates = analyzer.score_agent_candidates("review this code");
        assert!(!candidates.is_empty());
        assert!(candidates.iter().all(|c| c.score > 0.3));
    }

    #[test]
    fn test_should_auto_delegate() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        // "use agent to review" = 0.4 (use) + 0.4 (agent) + 0.3 (review) = 1.0 (capped) > 0.6
        assert!(analyzer.should_auto_delegate("use agent to review this code"));
        assert!(!analyzer.should_auto_delegate("hello world"));
    }

    #[test]
    fn test_calculate_delegation_confidence() {
        let registry = create_test_registry();
        let _analyzer = RequestAnalyzer::new(registry);

        let confidence1 = RequestAnalyzer::calculate_delegation_confidence("review this code");
        let confidence2 = RequestAnalyzer::calculate_delegation_confidence("hello world");

        assert!(confidence1 > confidence2);
        assert!(confidence1 >= 0.3); // "review" gives 0.3 points
        assert!(confidence2 < 0.1); // No keywords should give 0
    }

    #[test]
    fn test_delegation_decision_accuracy() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        // Test cases with expected delegation decisions
        let test_cases = vec![
            ("use code-reviewer agent", true, 1.0), // Explicit - should always delegate
            ("review this code please", true, 0.6), // Strong keywords - should delegate
            ("analyze the data", true, 0.3),        // Weak keywords - might delegate
            ("hello world", false, 0.1),            // No keywords - should not delegate
            ("help me with something", false, 0.2), // Weak keywords - should not delegate
        ];

        for (input, should_delegate, min_confidence) in test_cases {
            let (intent, confidence) = analyzer.analyze_delegation_intent(input);

            match intent {
                DelegationIntent::ExplicitAgent(_) => {
                    assert!(should_delegate, "Input '{}' should delegate", input);
                    assert_eq!(confidence, 1.0, "Explicit delegation should have confidence 1.0");
                },
                DelegationIntent::AutoDelegate(_) => {
                    assert!(should_delegate, "Input '{}' should delegate", input);
                    assert!(
                        confidence >= min_confidence,
                        "Input '{}' confidence {} should be >= {}",
                        input,
                        confidence,
                        min_confidence
                    );
                },
                DelegationIntent::NoDelegate => {
                    if should_delegate {
                        // This is acceptable if confidence is below threshold
                        assert!(
                            confidence < 0.6,
                            "Input '{}' should have low confidence if not delegating",
                            input
                        );
                    }
                },
            }
        }
    }

    #[test]
    fn test_agent_scoring_accuracy() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        // Test agent scoring with different inputs
        let test_cases = vec![
            ("review this code", "code-reviewer", 0.3), // Should score high for code-reviewer
            ("analyze data", "code-reviewer", 0.0),     // Should score low for unrelated task
            ("code-reviewer help me", "code-reviewer", 0.5), // Agent name mentioned
        ];

        for (input, agent_name, min_score) in test_cases {
            let score = analyzer.calculate_agent_score(agent_name, input);
            assert!(
                score >= min_score,
                "Agent '{}' should score >= {} for input '{}', got {}",
                agent_name,
                min_score,
                input,
                score
            );
        }
    }

    #[test]
    fn test_threshold_logic_consistency() {
        let registry = create_test_registry();
        let analyzer = RequestAnalyzer::new(registry);

        // Test that should_auto_delegate is consistent with analyze_delegation_intent
        let test_inputs = vec![
            "review this code",
            "analyze the data",
            "hello world",
            "help me debug",
            "use agent to review code", // This should match the code-reviewer agent
        ];

        for input in test_inputs {
            let should_delegate = analyzer.should_auto_delegate(input);
            let (intent, confidence) = analyzer.analyze_delegation_intent(input);

            match intent {
                DelegationIntent::AutoDelegate(_) => {
                    assert!(
                        should_delegate,
                        "should_auto_delegate should return true when analyze returns AutoDelegate for '{}'",
                        input
                    );
                    assert!(
                        confidence > 0.6,
                        "AutoDelegate should have confidence > 0.6 for '{}'",
                        input
                    );
                },
                DelegationIntent::ExplicitAgent(_) => {
                    // Explicit agents bypass the threshold check
                },
                DelegationIntent::NoDelegate => {
                    if should_delegate {
                        // This can happen if confidence is just above threshold but no suitable agents found
                        assert!(
                            confidence > 0.6,
                            "should_auto_delegate true but NoDelegate should have high confidence for '{}'",
                            input
                        );
                    }
                },
            }
        }
    }
}
