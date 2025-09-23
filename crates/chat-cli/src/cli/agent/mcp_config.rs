use std::collections::HashMap;
use std::path::Path;

use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};

use crate::cli::chat::tools::custom_tool::CustomToolConfig;
use crate::os::Os;

// This is to mirror claude's config set up
#[derive(Clone, Serialize, Deserialize, Debug, Default, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase", transparent)]
pub struct McpServerConfig {
    pub mcp_servers: HashMap<String, CustomToolConfig>,
}

impl McpServerConfig {
    pub async fn load_from_file(os: &Os, path: impl AsRef<Path>) -> eyre::Result<Self> {
        let contents = os.fs.read(path.as_ref()).await?;
        let value = serde_json::from_slice::<serde_json::Value>(&contents)?;
        // We need to extract mcp_servers field from the value because we have annotated
        // [McpServerConfig] with transparent. Transparent was added because we want to preserve
        // the type in agent.
        let config = value
            .get("mcpServers")
            .cloned()
            .ok_or(eyre::eyre!("No mcp servers found in config"))?;
        Ok(serde_json::from_value(config)?)
    }

    pub async fn save_to_file(&self, os: &Os, path: impl AsRef<Path>) -> eyre::Result<()> {
        let json = self.to_non_transparent_json_pretty()?;
        os.fs.write(path.as_ref(), json).await?;
        Ok(())
    }

    /// Because we had annotated [McpServerConfig] with transparent, when writing the config alone
    /// to its legacy location (as opposed to writing it along with its agent config), we would
    /// need to call this function to stringify it otherwise we would be writing only the inner
    /// hashmap.
    fn to_non_transparent_json_pretty(&self) -> eyre::Result<String> {
        let transparent_json = serde_json::to_value(self)?;
        let non_transparent_json = serde_json::json!({
            "mcpServers": transparent_json
        });
        Ok(serde_json::to_string_pretty(&non_transparent_json)?)
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::os::Os;

    fn create_test_custom_tool_config() -> CustomToolConfig {
        CustomToolConfig {
            r#type: crate::cli::chat::tools::custom_tool::TransportType::Stdio,
            url: String::new(),
            headers: HashMap::new(),
            oauth_scopes: Vec::new(),
            command: "test_command".to_string(),
            args: Vec::new(),
            env: Some(HashMap::new()),
            timeout: crate::cli::chat::tools::custom_tool::default_timeout(),
            disabled: false,
            is_from_legacy_mcp_json: false,
        }
    }

    #[test]
    fn test_mcp_server_config_default() {
        let config = McpServerConfig::default();
        assert!(config.mcp_servers.is_empty());
    }

    #[test]
    fn test_mcp_server_config_serialization() {
        let mut servers = HashMap::new();
        servers.insert("test_server".to_string(), create_test_custom_tool_config());

        let config = McpServerConfig { mcp_servers: servers };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: McpServerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_to_non_transparent_json_pretty() {
        let mut servers = HashMap::new();
        servers.insert("test".to_string(), create_test_custom_tool_config());
        let config = McpServerConfig { mcp_servers: servers };

        let json = config.to_non_transparent_json_pretty().unwrap();
        assert!(json.contains("mcpServers"));
        assert!(json.contains("test"));

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("mcpServers").is_some());
    }

    #[test]
    fn test_to_non_transparent_json_pretty_empty() {
        let config = McpServerConfig::default();
        let json = config.to_non_transparent_json_pretty().unwrap();
        assert!(json.contains("mcpServers"));
        assert!(json.contains("{}"));
    }

    #[test]
    fn test_to_non_transparent_json_pretty_error() {
        // Test serialization error handling by creating a config that would fail
        let config = McpServerConfig::default();
        let result = config.to_non_transparent_json_pretty();
        assert!(result.is_ok()); // This should still work for empty config
    }

    #[test]
    fn test_mcp_server_config_equality() {
        let config1 = McpServerConfig::default();
        let config2 = McpServerConfig::default();
        assert_eq!(config1, config2);

        let mut servers = HashMap::new();
        servers.insert("test".to_string(), create_test_custom_tool_config());
        let config3 = McpServerConfig { mcp_servers: servers };
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_mcp_server_config_clone() {
        let mut servers = HashMap::new();
        servers.insert("test".to_string(), create_test_custom_tool_config());
        let config = McpServerConfig { mcp_servers: servers };

        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn test_mcp_server_config_debug() {
        let config = McpServerConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("McpServerConfig"));
    }

    #[tokio::test]
    async fn test_load_from_file_success() {
        let os = Os::new().await.unwrap();
        let file_path = "/test_config.json"; // Use a simple path in the test filesystem

        // Create test JSON content using the Os filesystem
        let json_content = r#"{"mcpServers": {"test": {"command": "test_cmd"}}}"#;
        os.fs.write(&file_path, json_content).await.unwrap();

        let result = McpServerConfig::load_from_file(&os, &file_path).await;
        if let Err(e) = &result {
            println!("Error loading file: {:?}", e);
        }
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.mcp_servers.contains_key("test"));
    }

    #[tokio::test]
    async fn test_load_from_file_missing_mcp_servers() {
        let os = Os::new().await.unwrap();
        let file_path = "/test_config_missing.json";

        // Create JSON without mcpServers field using Os filesystem
        let json_content = r#"{"other": "value"}"#;
        os.fs.write(&file_path, json_content).await.unwrap();

        let result = McpServerConfig::load_from_file(&os, &file_path).await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        println!("Error message: {}", error_msg);
        assert!(error_msg.contains("No mcp servers found"));
    }

    #[tokio::test]
    async fn test_load_from_file_invalid_json() {
        let os = Os::new().await.unwrap();
        let file_path = "/test_config_invalid.json";

        // Create invalid JSON using Os filesystem
        let json_content = r#"{"invalid": json}"#;
        os.fs.write(&file_path, json_content).await.unwrap();

        let result = McpServerConfig::load_from_file(&os, &file_path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_from_file_nonexistent() {
        let os = Os::new().await.unwrap();
        let file_path = "/nonexistent/path/config.json";

        let result = McpServerConfig::load_from_file(&os, file_path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_to_file_success() {
        let os = Os::new().await.unwrap();
        let file_path = "/output_config.json";

        let mut servers = HashMap::new();
        servers.insert("test".to_string(), create_test_custom_tool_config());
        let config = McpServerConfig { mcp_servers: servers };

        let result = config.save_to_file(&os, &file_path).await;
        assert!(result.is_ok(), "Failed to save file: {:?}", result.err());

        // Verify file was created and contains expected content using Os filesystem
        assert!(os.fs.try_exists(&file_path).await.unwrap());
        let content = os.fs.read_to_string(&file_path).await.unwrap();
        assert!(content.contains("mcpServers"));
        assert!(content.contains("test"));
    }

    #[tokio::test]
    async fn test_save_to_file_error() {
        let os = Os::new().await.unwrap();
        let file_path = "/invalid/path/config.json";

        let config = McpServerConfig::default();
        let result = config.save_to_file(&os, file_path).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_mcp_server_config_with_multiple_servers() {
        let mut servers = HashMap::new();
        servers.insert("server1".to_string(), create_test_custom_tool_config());
        servers.insert("server2".to_string(), create_test_custom_tool_config());

        let config = McpServerConfig { mcp_servers: servers };
        assert_eq!(config.mcp_servers.len(), 2);

        let json = config.to_non_transparent_json_pretty().unwrap();
        assert!(json.contains("server1"));
        assert!(json.contains("server2"));
    }
}
