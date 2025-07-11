pub mod clear;
pub mod commands; // NEW: Add commands module
pub mod compact;
pub mod context;
pub mod editor;
pub mod hooks;
pub mod knowledge;
pub mod mcp;
pub mod model;
pub mod persist;
pub mod profile;
pub mod prompts;
pub mod subscribe;
pub mod tools;
pub mod usage;

use clap::Parser;
use clear::ClearArgs;
use commands::CommandsSubcommand; // NEW: Add commands import
use compact::CompactArgs;
use context::ContextSubcommand;
use editor::EditorArgs;
use hooks::HooksArgs;
use knowledge::KnowledgeSubcommand;
use mcp::McpArgs;
use model::ModelArgs;
use persist::PersistSubcommand;
use profile::AgentSubcommand;
use prompts::PromptsArgs;
use tools::ToolsArgs;

use crate::cli::chat::cli::subscribe::SubscribeArgs;
use crate::cli::chat::cli::usage::UsageArgs;
use crate::cli::chat::{
    ChatError,
    ChatSession,
    ChatState,
    EXTRA_HELP,
};
use crate::cli::issue;
use crate::os::Os;

/// q (Amazon Q Chat)
#[derive(Debug, PartialEq, Parser)]
#[command(color = clap::ColorChoice::Always, term_width = 0, after_long_help = EXTRA_HELP)]
pub enum SlashCommand {
    /// Quit the application
    #[command(aliases = ["q", "exit"])]
    Quit,
    /// Clear the conversation history
    Clear(ClearArgs),
    /// Manage agents
    #[command(subcommand, aliases = ["profile"])]
    Agent(AgentSubcommand),
    /// Manage context files for the chat session
    #[command(subcommand)]
    Context(ContextSubcommand),
    /// (Beta) Manage knowledge base for persistent context storage. Requires "q settings
    /// chat.enableKnowledge true"
    #[command(subcommand, hide = true)]
    Knowledge(KnowledgeSubcommand),
    /// (Beta) Manage custom commands. Requires "q settings chat.enableCommands true"
    #[command(subcommand, hide = true)]
    Commands(CommandsSubcommand), // NEW: Add Commands subcommand
    /// Open $EDITOR (defaults to vi) to compose a prompt
    #[command(name = "editor")]
    PromptEditor(EditorArgs),
    /// Summarize the conversation to free up context space
    Compact(CompactArgs),
    /// View and manage tools and permissions
    Tools(ToolsArgs),
    /// Create a new Github issue or make a feature request
    Issue(issue::IssueArgs),
    /// View and retrieve prompts
    Prompts(PromptsArgs),
    /// View and manage context hooks
    Hooks(HooksArgs),
    /// Show current session's context window usage
    Usage(UsageArgs),
    /// See mcp server loaded
    Mcp(McpArgs),
    /// Select a model for the current conversation session
    Model(ModelArgs),
    /// Upgrade to a Q Developer Pro subscription for increased query limits
    Subscribe(SubscribeArgs),
    #[command(flatten)]
    Persist(PersistSubcommand),
    // #[command(flatten)]
    // Root(RootSubcommand),
}

impl SlashCommand {
    pub async fn execute(self, os: &mut Os, session: &mut ChatSession) -> Result<ChatState, ChatError> {
        match self {
            Self::Quit => Ok(ChatState::Exit),
            Self::Clear(args) => args.execute(session).await,
            Self::Agent(subcommand) => subcommand.execute(os, session).await,
            Self::Context(args) => args.execute(os, session).await,
            Self::Knowledge(subcommand) => subcommand.execute(os, session).await,
            Self::Commands(subcommand) => subcommand.execute(os, session).await, // NEW: Add Commands execution
            Self::PromptEditor(args) => args.execute(session).await,
            Self::Compact(args) => args.execute(os, session).await,
            Self::Tools(args) => args.execute(session).await,
            Self::Issue(args) => {
                if let Err(err) = args.execute(os).await {
                    return Err(ChatError::Custom(err.to_string().into()));
                }

                Ok(ChatState::PromptUser {
                    skip_printing_tools: true,
                })
            },
            Self::Prompts(args) => args.execute(session).await,
            Self::Hooks(args) => args.execute(os, session).await,
            Self::Usage(args) => args.execute(os, session).await,
            Self::Mcp(args) => args.execute(session).await,
            Self::Model(args) => args.execute(session).await,
            Self::Subscribe(args) => args.execute(os, session).await,
            Self::Persist(subcommand) => subcommand.execute(os, session).await,
            // Self::Root(subcommand) => {
            //     if let Err(err) = subcommand.execute(os, database, telemetry).await {
            //         return Err(ChatError::Custom(err.to_string().into()));
            //     }
            //
            //     Ok(ChatState::PromptUser {
            //         skip_printing_tools: true,
            //     })
            // },
        }
    }
}
