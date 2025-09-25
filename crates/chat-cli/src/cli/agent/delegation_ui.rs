use std::collections::VecDeque;
use std::io::Write;

use crossterm::{
    queue,
    style,
};

use super::delegator::DelegationHistory;

/// Display current agent status in conversation UI
#[allow(dead_code)]
pub fn display_agent_status(agent_name: &str, output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Cyan),
        style::Print("🤖 Active Agent: "),
        style::SetForegroundColor(style::Color::Green),
        style::Print(agent_name),
        style::ResetColor,
        style::Print("\n")
    )
}

/// Display delegation history
pub fn display_delegation_history(
    history: &VecDeque<DelegationHistory>,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Cyan),
        style::Print("📋 Delegation History:\n"),
        style::ResetColor
    )?;

    if history.is_empty() {
        queue!(
            output,
            style::SetForegroundColor(style::Color::Grey),
            style::Print("  No delegation history available.\n"),
            style::ResetColor
        )?;
        return Ok(());
    }

    for (i, entry) in history.iter().enumerate().rev().take(10) {
        let status_icon = if entry.success { "✅" } else { "❌" };
        let status_color = if entry.success {
            style::Color::Green
        } else {
            style::Color::Red
        };

        queue!(
            output,
            style::SetForegroundColor(style::Color::Grey),
            style::Print(format!("  {}. ", history.len() - i)),
            style::Print(status_icon),
            style::Print(" "),
            style::SetForegroundColor(status_color),
            style::Print(&entry.agent),
            style::ResetColor,
            style::Print(" - "),
            style::Print(&entry.task[..entry.task.len().min(50)]),
            style::Print(if entry.task.len() > 50 { "..." } else { "" }),
            style::Print("\n")
        )?;
    }

    Ok(())
}

/// Display delegation statistics
pub fn display_delegation_stats(history: &VecDeque<DelegationHistory>, output: &mut impl Write) -> std::io::Result<()> {
    let total = history.len();
    let successful = history.iter().filter(|h| h.success).count();
    let success_rate = if total > 0 {
        (successful as f32 / total as f32) * 100.0
    } else {
        0.0
    };

    queue!(
        output,
        style::SetForegroundColor(style::Color::Cyan),
        style::Print("📊 Delegation Statistics:\n"),
        style::ResetColor,
        style::Print(format!("  Total delegations: {}\n", total)),
        style::Print(format!("  Successful: {}\n", successful)),
        style::Print(format!("  Failed: {}\n", total - successful)),
        style::SetForegroundColor(style::Color::Green),
        style::Print(format!("  Success rate: {:.1}%\n", success_rate)),
        style::ResetColor
    )
}

/// Display debug information for delegation decision
pub fn display_delegation_debug(
    input: &str,
    intent: &str,
    candidates: &[String],
    selected: Option<&str>,
    reasoning: &str,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Magenta),
        style::Print("🔍 DEBUG: Delegation Analysis\n"),
        style::ResetColor,
        style::Print(format!("  Input: {}\n", input)),
        style::Print(format!("  Intent: {}\n", intent)),
        style::Print(format!("  Candidates: [{}]\n", candidates.join(", "))),
        style::Print(format!("  Selected: {}\n", selected.unwrap_or("None"))),
        style::Print(format!("  Reasoning: {}\n", reasoning)),
        style::Print("─".repeat(50)),
        style::Print("\n")
    )
}

/// Display debug information for agent scoring
pub fn display_agent_scoring_debug(
    agent_name: &str,
    keyword_score: f32,
    pattern_score: f32,
    priority_score: f32,
    success_rate: f32,
    final_score: f32,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Magenta),
        style::Print(format!("🔍 DEBUG: Agent Scoring - {}\n", agent_name)),
        style::ResetColor,
        style::Print(format!("  Keyword Score: {:.2}\n", keyword_score)),
        style::Print(format!("  Pattern Score: {:.2}\n", pattern_score)),
        style::Print(format!("  Priority Score: {:.2}\n", priority_score)),
        style::Print(format!("  Success Rate: {:.2}\n", success_rate)),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(format!("  Final Score: {:.2}\n", final_score)),
        style::ResetColor
    )
}

/// Display debug information for context creation
#[allow(dead_code)]
pub fn display_context_debug(
    agent_name: &str,
    context_size: usize,
    creation_time_ms: u64,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Magenta),
        style::Print(format!("🔍 DEBUG: Context Creation - {}\n", agent_name)),
        style::ResetColor,
        style::Print(format!("  Context Size: {} messages\n", context_size)),
        style::Print(format!("  Creation Time: {}ms\n", creation_time_ms)),
        style::ResetColor
    )
}

/// Display delegation notification when agent switches
pub fn display_delegation_notification(
    from_agent: &str,
    to_agent: &str,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Yellow),
        style::Print("🔄 Delegating from "),
        style::SetForegroundColor(style::Color::Blue),
        style::Print(from_agent),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(" to "),
        style::SetForegroundColor(style::Color::Green),
        style::Print(to_agent),
        style::ResetColor,
        style::Print("\n")
    )
}

/// Display delegation failure with clear error message
pub fn display_delegation_error(agent_name: &str, error_msg: &str, output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Red),
        style::Print("❌ Delegation failed to "),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(agent_name),
        style::SetForegroundColor(style::Color::Red),
        style::Print(": "),
        style::ResetColor,
        style::Print(error_msg),
        style::Print("\n")
    )
}

/// Display specific error for agent not found
pub fn display_agent_not_found_error(agent_name: &str, output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Red),
        style::Print("❌ Agent '"),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(agent_name),
        style::SetForegroundColor(style::Color::Red),
        style::Print("' not found. Use 'q agent list' to see available agents."),
        style::ResetColor,
        style::Print("\n")
    )
}

/// Display specific error for context creation failure
pub fn display_context_creation_error(agent_name: &str, reason: &str, output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Red),
        style::Print("❌ Failed to create context for "),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(agent_name),
        style::SetForegroundColor(style::Color::Red),
        style::Print(": "),
        style::ResetColor,
        style::Print(reason),
        style::Print("\n")
    )
}

/// Display specific error for delegation timeout
pub fn display_delegation_timeout_error(
    agent_name: &str,
    timeout_ms: u64,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Red),
        style::Print("⏱️  Delegation to "),
        style::SetForegroundColor(style::Color::Yellow),
        style::Print(agent_name),
        style::SetForegroundColor(style::Color::Red),
        style::Print(format!(" timed out after {}ms", timeout_ms)),
        style::ResetColor,
        style::Print("\n")
    )
}

/// Display graceful fallback notification
pub fn display_fallback_notification(
    failed_agent: &str,
    fallback_agent: &str,
    output: &mut impl Write,
) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::Yellow),
        style::Print("⚠️  Falling back from "),
        style::Print(failed_agent),
        style::Print(" to "),
        style::SetForegroundColor(style::Color::Green),
        style::Print(fallback_agent),
        style::ResetColor,
        style::Print("\n")
    )
}

/// Create visual indicator badge for active agent
#[allow(dead_code)]
pub fn create_agent_badge(agent_name: &str) -> String {
    format!("[{}]", agent_name)
}

/// Create colored agent indicator for conversation themes
#[allow(dead_code)]
pub fn create_colored_agent_indicator(agent_name: &str, output: &mut impl Write) -> std::io::Result<()> {
    queue!(
        output,
        style::SetForegroundColor(style::Color::White),
        style::SetBackgroundColor(style::Color::DarkBlue),
        style::Print(" "),
        style::Print(agent_name),
        style::Print(" "),
        style::ResetColor
    )
}

/// Display agent status with priority indicator
#[allow(dead_code)]
pub fn display_agent_with_priority(agent_name: &str, priority: u8, output: &mut impl Write) -> std::io::Result<()> {
    let priority_color = match priority {
        9..=10 => style::Color::Red,
        7..=8 => style::Color::Yellow,
        5..=6 => style::Color::Green,
        _ => style::Color::Grey,
    };

    queue!(
        output,
        style::SetForegroundColor(priority_color),
        style::Print("●"),
        style::ResetColor,
        style::Print(" "),
        style::SetForegroundColor(style::Color::Cyan),
        style::Print(agent_name),
        style::ResetColor,
        style::Print(format!(" (P{})", priority))
    )
}
