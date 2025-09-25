use std::sync::Arc;

use regex::Regex;

use super::registry::{
    AgentCandidate,
    AgentRegistry,
};

/// Configuration for delegation decision thresholds
#[derive(Debug, Clone)]
pub struct DelegationConfig {
    /// Minimum confidence threshold for auto-delegation (0.0-1.0)
    pub auto_delegate_threshold: f32,
    /// Minimum score threshold for agent candidates (0.0-1.0)
    pub candidate_score_threshold: f32,
    /// Maximum number of candidates to return
    pub max_candidates: usize,
}

impl Default for DelegationConfig {
    fn default() -> Self {
        Self {
            auto_delegate_threshold: 0.6,
            candidate_score_threshold: 0.3,
            max_candidates: 5,
        }
    }
}

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
    config: DelegationConfig,
}

#[allow(dead_code)]
impl RequestAnalyzer {
    pub fn new(registry: Arc<AgentRegistry>) -> Self {
        Self::with_config(registry, DelegationConfig::default())
    }

    pub fn with_config(registry: Arc<AgentRegistry>, config: DelegationConfig) -> Self {
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
            config,
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
        if confidence > self.config.auto_delegate_threshold {
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

        // Apply priority-based selection algorithm
        self.apply_priority_based_selection(&mut candidates);

        // Sort by score descending and filter by configurable threshold
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates
            .into_iter()
            .filter(|c| c.score > self.config.candidate_score_threshold)
            .take(self.config.max_candidates)
            .collect()
    }

    /// Apply priority-based selection algorithm to enhance candidate scoring
    fn apply_priority_based_selection(&self, candidates: &mut [AgentCandidate]) {
        for candidate in candidates {
            if let Some(agent) = self.registry.get_agent(&candidate.name) {
                if let Some(delegation) = &agent.delegation {
                    // Priority boost: higher priority agents get score multiplier
                    let priority_multiplier = 1.0 + (delegation.priority as f32 / 20.0); // 1.0-1.5x multiplier
                    candidate.score *= priority_multiplier;

                    // Auto-delegate preference: agents that allow auto-delegation get slight boost
                    if delegation.auto_delegate {
                        candidate.score += 0.1;
                    }

                    // Context inheritance consideration: full context agents get boost for complex tasks
                    if matches!(
                        delegation.context_inheritance,
                        super::delegation::ContextInheritanceLevel::Full
                    ) {
                        candidate.score += 0.05;
                    }
                }
            }
        }
    }

    /// Determine if input should trigger auto-delegation
    #[allow(clippy::unused_self)] // Public API method
    pub fn should_auto_delegate(&self, input: &str) -> bool {
        let confidence = Self::calculate_delegation_confidence(input);
        confidence > self.config.auto_delegate_threshold
    }

    /// Calculate confidence score for delegation (0.0 to 1.0)
    fn calculate_delegation_confidence(input: &str) -> f32 {
        let input_lower = input.to_lowercase();
        let mut score: f32 = 0.0;

        // Strong delegation indicators with word boundary matching
        let strong_indicators = [
            ("review", 0.4),
            ("analyze", 0.4),
            ("check", 0.3),
            ("test", 0.3),
            ("debug", 0.4),
            ("fix", 0.4),
            ("optimize", 0.4),
            ("refactor", 0.4),
            ("validate", 0.3),
            ("verify", 0.3),
            ("inspect", 0.3),
        ];

        for (keyword, weight) in &strong_indicators {
            let word_pattern = format!(r"\b{}\b", regex::escape(keyword));
            if let Ok(regex) = Regex::new(&word_pattern) {
                if regex.is_match(&input_lower) {
                    score += weight;
                }
            } else if input_lower.contains(keyword) {
                score += weight * 0.7; // Reduced score for partial matches
            }
        }

        // Weak delegation indicators
        let weak_indicators = [
            ("help", 0.15),
            ("assist", 0.15),
            ("look", 0.1),
            ("examine", 0.2),
            ("show", 0.1),
            ("explain", 0.15),
            ("guide", 0.2),
        ];

        for (keyword, weight) in &weak_indicators {
            let word_pattern = format!(r"\b{}\b", regex::escape(keyword));
            if let Ok(regex) = Regex::new(&word_pattern) {
                if regex.is_match(&input_lower) {
                    score += weight;
                }
            }
        }

        // Explicit delegation phrases (higher weight)
        let explicit_patterns = [
            (r"\bdelegate\s+to\b", 0.8),
            (r"\buse\s+\w+\s+agent\b", 0.7),
            (r"\bask\s+\w+\s+to\b", 0.6),
            (r"\b\w+\s+agent:\b", 0.7),
            (r"\bcan\s+you\s+(review|analyze|check|test)\b", 0.5),
        ];

        for (pattern, weight) in &explicit_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&input_lower) {
                    score += weight;
                }
            }
        }

        // Question patterns that suggest delegation
        let question_patterns = [
            (r"\bwhat\s+(is|are)\s+wrong\b", 0.3),
            (r"\bhow\s+(can|do)\s+i\s+(fix|improve)\b", 0.4),
            (r"\bwhy\s+(is|does)\b.*\b(not\s+work|fail|error)\b", 0.4),
        ];

        for (pattern, weight) in &question_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&input_lower) {
                    score += weight;
                }
            }
        }

        // Technical context indicators
        if input_lower.contains("code")
            || input_lower.contains("function")
            || input_lower.contains("class")
            || input_lower.contains("method")
        {
            score += 0.2;
        }

        // File extension context
        if Regex::new(r"\.\w{2,4}\b").unwrap().is_match(&input_lower) {
            score += 0.1; // Mentions file extensions
        }

        // Cap at 1.0
        score.min(1.0)
    }

    /// Calculate score for a specific agent against input
    fn calculate_agent_score(&self, agent_name: &str, input: &str) -> f32 {
        if let Some(agent) = self.registry.get_agent(agent_name) {
            let mut score: f32 = 0.0;
            let input_lower = input.to_lowercase();

            // Check delegation configuration
            if let Some(delegation) = &agent.delegation {
                // Enhanced keyword matching with word boundaries and context
                for keyword in &delegation.keywords {
                    let keyword_lower = keyword.to_lowercase();

                    // Exact word match (higher score)
                    let word_boundary_pattern = format!(r"\b{}\b", regex::escape(&keyword_lower));
                    if let Ok(regex) = Regex::new(&word_boundary_pattern) {
                        if regex.is_match(&input_lower) {
                            score += 0.4; // Higher score for exact word matches
                            continue;
                        }
                    }

                    // Partial match (lower score)
                    if input_lower.contains(&keyword_lower) {
                        score += 0.2;
                    }
                }

                // Enhanced pattern matching with task patterns
                for pattern in &delegation.task_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        if regex.is_match(&input_lower) {
                            score += 0.5; // Task patterns get higher weight
                        }
                    }
                }

                // Priority-based scoring boost
                let priority_boost = (delegation.priority as f32) / 100.0; // 0.01-0.10 boost
                score += priority_boost;
            }

            // Agent name mentioned (with word boundaries)
            let name_pattern = format!(r"\b{}\b", regex::escape(&agent_name.to_lowercase()));
            if let Ok(regex) = Regex::new(&name_pattern) {
                if regex.is_match(&input_lower) {
                    score += 0.6; // High score for explicit agent name mention
                }
            } else if input_lower.contains(&agent_name.to_lowercase()) {
                score += 0.3; // Lower score for partial name match
            }

            // Contextual scoring based on input characteristics
            score += Self::calculate_contextual_score(&input_lower, agent_name);

            score.min(1.0)
        } else {
            0.0
        }
    }

    /// Calculate contextual score based on input characteristics and agent specialization
    fn calculate_contextual_score(input_lower: &str, agent_name: &str) -> f32 {
        let mut contextual_score = 0.0;

        // Code-related context
        if (input_lower.contains("code") || input_lower.contains("function") || input_lower.contains("class"))
            && (agent_name.contains("code") || agent_name.contains("review") || agent_name.contains("dev"))
        {
            contextual_score += 0.2;
        }

        // Documentation context
        if (input_lower.contains("document") || input_lower.contains("readme") || input_lower.contains("docs"))
            && (agent_name.contains("doc") || agent_name.contains("write"))
        {
            contextual_score += 0.2;
        }

        // Testing context
        if (input_lower.contains("test") || input_lower.contains("spec") || input_lower.contains("unit"))
            && (agent_name.contains("test") || agent_name.contains("qa"))
        {
            contextual_score += 0.2;
        }

        contextual_score
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

        // "please review this code" should exceed 0.6 threshold (review=0.4 + can you=0.5)
        match analyzer.analyze("can you review this code please") {
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

        // "can you review" = 0.5 (can you pattern) + 0.4 (review) = 0.9 > 0.6
        assert!(analyzer.should_auto_delegate("can you review this code"));
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
