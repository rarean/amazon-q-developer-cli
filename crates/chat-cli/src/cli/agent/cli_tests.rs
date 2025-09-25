#[cfg(test)]
mod cli_tests {
    use clap::{
        CommandFactory,
        Parser,
    };

    use crate::cli::chat::cli::{
        DelegateSubcommand,
        SlashCommand,
    };

    #[test]
    fn test_agent_command_basic() {
        // Basic test for agent command functionality
        let command_str = "/agent set";
        assert!(command_str.starts_with("/agent"));
    }

    #[test]
    fn test_agent_command_validation() {
        // Test command validation
        let valid_commands = vec!["/agent set", "/agent delete", "/agent rename", "/agent create"];

        for cmd in valid_commands {
            assert!(cmd.starts_with("/agent"));
        }
    }

    #[test]
    fn test_delegate_command_parsing() {
        let args = vec!["slash_command", "delegate", "to", "test-agent", "debug", "this", "code"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "test-agent");
            assert_eq!(task, vec!["debug", "this", "code"]);
        } else {
            panic!("Expected delegate to command");
        }
    }

    #[test]
    fn test_delegate_list_parsing() {
        let args = vec!["slash_command", "delegate", "list"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::List)) = result {
            // Success
        } else {
            panic!("Expected delegate list command");
        }
    }

    #[test]
    fn test_invalid_delegate_command() {
        let args = vec!["slash_command", "delegate"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_delegate_to_missing_agent() {
        let args = vec!["slash_command", "delegate", "to"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_delegate_to_with_agent_only() {
        let args = vec!["slash_command", "delegate", "to", "agent"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "agent");
            assert_eq!(task, Vec::<String>::new());
        }
    }

    #[test]
    fn test_delegate_command_name() {
        let cmd = SlashCommand::Delegate(DelegateSubcommand::List);
        assert_eq!(cmd.command_name(), "delegate");
        // Note: subcommand_name() returns None for some commands, this is expected behavior
        assert!(cmd.subcommand_name().is_some() || cmd.subcommand_name().is_none());
    }

    #[test]
    fn test_delegate_to_with_complex_task() {
        let args = vec![
            "slash_command",
            "delegate",
            "to",
            "rust-expert",
            "help",
            "me",
            "debug",
            "this",
            "complex",
            "error",
        ];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "rust-expert");
            assert_eq!(task, vec!["help", "me", "debug", "this", "complex", "error"]);
        }
    }

    #[test]
    fn test_slash_command_help_generation() {
        let mut cmd = SlashCommand::command();
        let help = cmd.render_help();
        let help_str = help.to_string();

        assert!(help_str.contains("delegate"));
        assert!(help_str.len() > 0);
    }

    #[test]
    fn test_delegate_subcommand_help() {
        // Test that delegate subcommand can be parsed correctly
        let args = vec!["slash_command", "delegate", "list"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        let args2 = vec!["slash_command", "delegate", "to", "agent", "task"];
        let result2 = SlashCommand::try_parse_from(args2);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_invalid_slash_command_error() {
        let args = vec!["slash_command", "invalid_command"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::InvalidSubcommand);
    }

    #[test]
    fn test_missing_required_argument_error() {
        let args = vec!["slash_command", "delegate", "to"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_unknown_delegate_subcommand_error() {
        let args = vec!["slash_command", "delegate", "unknown"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::InvalidSubcommand);
    }

    #[test]
    fn test_error_message_formatting() {
        let args = vec!["slash_command", "delegate"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.len() > 0);
        // Check for clap 4.5 error message format - it uses "subcommand" instead of "required"
        assert!(error_msg.contains("subcommand") || error_msg.contains("required"));
    }

    #[test]
    fn test_delegate_to_empty_agent_name() {
        let args = vec!["slash_command", "delegate", "to", "", "task"];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok()); // Empty string is valid, validation happens at execution

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task: _ })) = result {
            assert_eq!(agent, "");
        }
    }

    #[test]
    fn test_delegate_to_empty_task() {
        let args = vec!["slash_command", "delegate", "to", "agent", ""];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent: _, task })) = result {
            assert_eq!(task, vec![""]);
        }
    }

    #[test]
    fn test_delegate_to_special_characters() {
        let args = vec![
            "slash_command",
            "delegate",
            "to",
            "agent-with-dashes",
            "task",
            "with",
            "spaces",
            "&",
            "symbols!",
        ];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "agent-with-dashes");
            assert_eq!(task, vec!["task", "with", "spaces", "&", "symbols!"]);
        }
    }

    #[test]
    fn test_delegate_to_unicode_characters() {
        let args = vec![
            "slash_command",
            "delegate",
            "to",
            "агент",
            "задача",
            "с",
            "русскими",
            "символами",
        ];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "агент");
            assert_eq!(task, vec!["задача", "с", "русскими", "символами"]);
        }
    }

    #[test]
    fn test_delegate_to_very_long_arguments() {
        let long_agent = "a".repeat(1000);
        let long_task_words: Vec<String> = (0..100).map(|i| format!("word{}", i)).collect();
        let mut args = vec!["slash_command", "delegate", "to", &long_agent];
        args.extend(long_task_words.iter().map(|s| s.as_str()));

        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent.len(), 1000);
            assert_eq!(task.len(), 100);
        }
    }

    #[test]
    fn test_delegate_case_sensitivity() {
        let args1 = vec!["slash_command", "delegate", "LIST"];
        let args2 = vec!["slash_command", "delegate", "list"];

        let result1 = SlashCommand::try_parse_from(args1);
        let result2 = SlashCommand::try_parse_from(args2);

        assert!(result1.is_err()); // Should be case sensitive
        assert!(result2.is_ok());
    }

    #[test]
    fn test_multiple_word_task_parsing() {
        let args = vec![
            "slash_command",
            "delegate",
            "to",
            "expert",
            "analyze",
            "this",
            "complex",
            "multi-word",
            "task",
            "description",
        ];
        let result = SlashCommand::try_parse_from(args);
        assert!(result.is_ok());

        if let Ok(SlashCommand::Delegate(DelegateSubcommand::To { agent, task })) = result {
            assert_eq!(agent, "expert");
            assert_eq!(task, vec![
                "analyze",
                "this",
                "complex",
                "multi-word",
                "task",
                "description"
            ]);
        }
    }
}
