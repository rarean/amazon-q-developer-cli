# Crate Integration Points - ACP Implementation

## 1. chat-cli Crate Integration

### 1.1 CLI Module Extensions (`src/cli/`)

#### New ACP Module (`src/cli/acp.rs`)
```rust
use clap::Parser;
use eyre::Result;
use std::process::ExitCode;
use crate::os::Os;

#[derive(Debug, Parser, PartialEq)]
pub struct AcpArgs {
    /// Enable debug logging for ACP protocol
    #[arg(long)]
    debug: bool,
    
    /// Specify custom MCP server configurations
    #[arg(long, value_delimiter = ',')]
    mcp_servers: Option<Vec<String>>,
    
    /// Override default session timeout (seconds)
    #[arg(long, default_value = "3600")]
    session_timeout: u64,
}

impl AcpArgs {
    pub async fn execute(self, os: &mut Os) -> Result<ExitCode> {
        // Initialize ACP agent with existing infrastructure
        let agent = acp::QCliAcpAgent::new(
            os.database.clone(),
            os.telemetry.clone(),
            self.mcp_servers,
        ).await?;
        
        // Run ACP server on stdio
        agent.run_stdio().await?;
        Ok(ExitCode::SUCCESS)
    }
}
```

#### CLI Root Integration (`src/cli/mod.rs`)
```rust
// Add to RootSubcommand enum
#[derive(Debug, PartialEq, Subcommand)]
pub enum RootSubcommand {
    // ... existing variants
    /// Agent Client Protocol server
    Acp(AcpArgs),
}

// Add to execute method
impl RootSubcommand {
    pub async fn execute(self, os: &mut Os) -> Result<ExitCode> {
        // ... existing auth check
        match self {
            // ... existing cases
            Self::Acp(args) => args.execute(os).await,
        }
    }
    
    pub fn requires_auth(&self) -> bool {
        matches!(self, Self::Chat(_) | Self::Profile | Self::Acp(_))
    }
}
```

### 1.2 API Client Integration (`src/api_client/`)

#### AWS Service Facade (`src/api_client/acp_facade.rs`)
```rust
use crate::api_client::{
    codewhisperer_client::CodeWhispererClient,
    qdeveloper_client::QDeveloperClient,
    consolas_client::ConsolasClient,
};
use semantic_search_client::SemanticSearchClient;

pub struct AwsServiceFacade {
    qdeveloper: QDeveloperClient,
    codewhisperer: CodeWhispererClient,
    consolas: ConsolasClient,
    semantic_search: SemanticSearchClient,
}

impl AwsServiceFacade {
    pub async fn new(os: &Os) -> Result<Self> {
        Ok(Self {
            qdeveloper: QDeveloperClient::new(&os.aws_config).await?,
            codewhisperer: CodeWhispererClient::new(&os.aws_config).await?,
            consolas: ConsolasClient::new(&os.aws_config).await?,
            semantic_search: SemanticSearchClient::new(&os.config).await?,
        })
    }
    
    pub async fn process_chat_request(
        &self,
        prompt: &str,
        context: &ConversationContext,
    ) -> Result<ChatResponse> {
        // Route to appropriate service based on request type
        match self.classify_request(prompt) {
            RequestType::Chat => self.qdeveloper.send_message(prompt, context).await,
            RequestType::Code => self.codewhisperer.generate_completion(prompt, context).await,
            RequestType::Analysis => self.consolas.analyze_request(prompt, context).await,
        }
    }
}
```

### 1.3 MCP Client Bridge (`src/mcp_client/acp_bridge.rs`)
```rust
use crate::mcp_client::{McpClient, ToolExecutor};
use agent_client_protocol as acp;

pub struct McpAcpBridge {
    mcp_client: McpClient,
    tool_registry: HashMap<String, acp::ToolInfo>,
}

impl McpAcpBridge {
    pub async fn new(mcp_servers: Option<Vec<String>>) -> Result<Self> {
        let mcp_client = McpClient::new(mcp_servers).await?;
        let mut bridge = Self {
            mcp_client,
            tool_registry: HashMap::new(),
        };
        bridge.discover_tools().await?;
        Ok(bridge)
    }
    
    pub async fn execute_acp_tool(
        &self,
        tool_call: &acp::ToolCall,
    ) -> Result<acp::ToolResult> {
        // Translate ACP tool call to MCP format
        let mcp_request = self.translate_to_mcp(tool_call)?;
        
        // Execute via existing MCP client
        let mcp_result = self.mcp_client.execute_tool(mcp_request).await?;
        
        // Translate result back to ACP format
        self.translate_to_acp(mcp_result)
    }
}
```

### 1.4 Database Extensions (`src/database/`)

#### ACP Session Storage (`src/database/acp_sessions.rs`)
```rust
use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use agent_client_protocol::SessionId;

#[derive(Debug, Serialize, Deserialize)]
pub struct AcpSession {
    pub id: SessionId,
    pub created_at: i64,
    pub last_active: i64,
    pub client_info: Option<String>,
    pub context_data: Vec<u8>,
}

impl AcpSession {
    pub fn create_table(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS acp_sessions (
                id TEXT PRIMARY KEY,
                created_at INTEGER NOT NULL,
                last_active INTEGER NOT NULL,
                client_info TEXT,
                context_data BLOB
            )",
            [],
        )?;
        Ok(())
    }
    
    pub fn insert(&self, conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "INSERT INTO acp_sessions (id, created_at, last_active, client_info, context_data)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &self.id.0.as_str(),
                &self.created_at.to_string(),
                &self.last_active.to_string(),
                &self.client_info.as_deref().unwrap_or(""),
                &base64::encode(&self.context_data),
            ],
        )?;
        Ok(())
    }
}
```

### 1.5 Telemetry Integration (`src/telemetry/`)

#### ACP Telemetry Events (`src/telemetry/acp_events.rs`)
```rust
use crate::telemetry::{TelemetryClient, TelemetryEvent};
use agent_client_protocol::SessionId;

pub struct AcpTelemetryEvents;

impl AcpTelemetryEvents {
    pub async fn session_created(
        telemetry: &TelemetryClient,
        session_id: &SessionId,
        client_info: Option<&str>,
    ) -> Result<()> {
        let event = TelemetryEvent::new("acp_session_created")
            .with_property("session_id", session_id.0.as_str())
            .with_property("client_info", client_info.unwrap_or("unknown"));
        
        telemetry.send_event(event).await
    }
    
    pub async fn tool_executed(
        telemetry: &TelemetryClient,
        tool_name: &str,
        execution_time_ms: u64,
        success: bool,
    ) -> Result<()> {
        let event = TelemetryEvent::new("acp_tool_executed")
            .with_property("tool_name", tool_name)
            .with_property("execution_time_ms", &execution_time_ms.to_string())
            .with_property("success", &success.to_string());
        
        telemetry.send_event(event).await
    }
}
```

## 2. ACP Crate Implementation

### 2.1 Core Agent (`crates/acp/src/agent.rs`)
```rust
use agent_client_protocol as acp;
use crate::{aws_facade::AwsServiceFacade, mcp_bridge::McpAcpBridge};
use chat_cli::{
    database::Database,
    telemetry::TelemetryClient,
    auth::is_logged_in,
};

pub struct QCliAcpAgent {
    sessions: Arc<Mutex<HashMap<acp::SessionId, SessionState>>>,
    aws_facade: AwsServiceFacade,
    mcp_bridge: Option<McpAcpBridge>,
    database: Database,
    telemetry: TelemetryClient,
}

impl acp::Agent for QCliAcpAgent {
    async fn initialize(
        &self,
        req: acp::InitializeRequest,
    ) -> Result<acp::InitializeResponse, acp::Error> {
        // Validate authentication using existing system
        if !is_logged_in(&self.database).await {
            return Err(acp::Error::authentication_required());
        }
        
        // Return capabilities based on available services
        Ok(acp::InitializeResponse {
            protocol_version: acp::V1,
            agent_capabilities: acp::AgentCapabilities {
                supports_streaming: true,
                supports_tools: self.mcp_bridge.is_some(),
                supports_file_operations: true,
            },
            auth_methods: vec![],
        })
    }
    
    async fn prompt(
        &self,
        req: acp::PromptRequest,
    ) -> Result<acp::PromptResponse, acp::Error> {
        // Process through AWS services
        let response = self.aws_facade
            .process_chat_request(&req.prompt, &req.session_id)
            .await
            .map_err(|e| acp::Error::internal_error(e.to_string()))?;
        
        // Handle tool calls if present
        if let Some(tool_calls) = response.tool_calls {
            for tool_call in tool_calls {
                if let Some(bridge) = &self.mcp_bridge {
                    let result = bridge.execute_acp_tool(&tool_call).await?;
                    // Send tool result back to client
                    self.send_tool_result(&req.session_id, result).await?;
                }
            }
        }
        
        Ok(acp::PromptResponse {
            stop_reason: acp::StopReason::EndTurn,
        })
    }
}
```

### 2.2 AWS Service Integration (`crates/acp/src/aws_facade.rs`)
```rust
use amzn_qdeveloper_streaming_client::QDeveloperStreamingClient;
use amzn_codewhisperer_streaming_client::CodeWhispererStreamingClient;
use amzn_consolas_client::ConsolasClient;

pub struct AwsServiceFacade {
    qdeveloper: QDeveloperStreamingClient,
    codewhisperer: CodeWhispererStreamingClient,
    consolas: ConsolasClient,
}

impl AwsServiceFacade {
    pub async fn process_chat_request(
        &self,
        prompt: &str,
        session_id: &acp::SessionId,
    ) -> Result<ChatResponse> {
        // Use Q Developer for general chat
        let request = self.build_qdeveloper_request(prompt, session_id)?;
        let mut stream = self.qdeveloper.send_message(request).await?;
        
        let mut response = ChatResponse::new();
        while let Some(event) = stream.next().await {
            match event? {
                QDeveloperEvent::MessageChunk(chunk) => {
                    response.add_content(chunk.content);
                },
                QDeveloperEvent::ToolCall(tool_call) => {
                    response.add_tool_call(tool_call);
                },
                QDeveloperEvent::Complete => break,
            }
        }
        
        Ok(response)
    }
}
```

### 2.3 MCP Bridge (`crates/acp/src/mcp_bridge.rs`)
```rust
use chat_cli::mcp_client::{McpClient, ToolCall as McpToolCall};
use agent_client_protocol as acp;

pub struct McpAcpBridge {
    mcp_client: McpClient,
}

impl McpAcpBridge {
    pub async fn execute_acp_tool(
        &self,
        tool_call: &acp::ToolCall,
    ) -> Result<acp::ToolResult> {
        // Convert ACP tool call to MCP format
        let mcp_call = McpToolCall {
            name: tool_call.name.clone(),
            arguments: tool_call.arguments.clone(),
        };
        
        // Execute via existing MCP client
        let mcp_result = self.mcp_client.execute_tool(mcp_call).await?;
        
        // Convert result back to ACP format
        Ok(acp::ToolResult {
            success: mcp_result.success,
            content: mcp_result.output,
            error: mcp_result.error,
        })
    }
}
```

## 3. Shared Infrastructure Integration

### 3.1 Authentication Flow
```rust
// Reuse existing authentication in ACP agent
impl QCliAcpAgent {
    async fn validate_authentication(&self) -> Result<(), acp::Error> {
        if !chat_cli::auth::is_logged_in(&self.database).await {
            return Err(acp::Error::authentication_required());
        }
        Ok(())
    }
}
```

### 3.2 Configuration Sharing
```rust
// Extend existing configuration for ACP
#[derive(Serialize, Deserialize)]
pub struct AcpConfig {
    pub enabled: bool,
    pub session_timeout: u64,
    pub max_concurrent_sessions: usize,
    pub mcp_servers: Vec<String>,
}

impl Default for AcpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            session_timeout: 3600,
            max_concurrent_sessions: 10,
            mcp_servers: vec![],
        }
    }
}
```

### 3.3 Error Handling Integration
```rust
// Unified error handling across crates
#[derive(Debug, thiserror::Error)]
pub enum AcpIntegrationError {
    #[error("Authentication error: {0}")]
    Auth(#[from] chat_cli::auth::AuthError),
    
    #[error("AWS service error: {0}")]
    AwsService(#[from] aws_smithy_runtime_api::client::result::SdkError<Box<dyn std::error::Error>>),
    
    #[error("MCP error: {0}")]
    Mcp(#[from] chat_cli::mcp_client::McpError),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
}

impl From<AcpIntegrationError> for acp::Error {
    fn from(err: AcpIntegrationError) -> Self {
        match err {
            AcpIntegrationError::Auth(_) => acp::Error::authentication_required(),
            AcpIntegrationError::Protocol(msg) => acp::Error::invalid_request(msg),
            _ => acp::Error::internal_error(err.to_string()),
        }
    }
}
```

## 4. Testing Integration Points

### 4.1 Shared Test Utilities
```rust
// Extend existing test utilities for ACP testing
pub mod test_utils {
    use chat_cli::util::test::{TestOs, create_test_database};
    
    pub async fn create_acp_test_environment() -> (TestOs, acp::QCliAcpAgent) {
        let mut os = TestOs::new().await;
        let agent = acp::QCliAcpAgent::new(
            os.database.clone(),
            os.telemetry.clone(),
            None, // No MCP servers for basic tests
        ).await.unwrap();
        
        (os, agent)
    }
}
```

### 4.2 Integration Test Framework
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use chat_cli::util::test::*;
    
    #[tokio::test]
    async fn test_acp_with_existing_auth() {
        let (mut os, agent) = create_acp_test_environment().await;
        
        // Use existing auth test utilities
        login_test_user(&mut os).await;
        
        // Test ACP functionality with authenticated user
        let init_response = agent.initialize(acp::InitializeRequest::default()).await;
        assert!(init_response.is_ok());
    }
}
```

## 5. Deployment Integration

### 5.1 Binary Integration
- Single binary with ACP support included
- No additional dependencies for existing users
- ACP functionality available via `q acp` subcommand

### 5.2 Configuration Integration
- Extend existing configuration files
- ACP settings in existing config directory
- Backward compatibility maintained

### 5.3 Documentation Integration
- Add ACP documentation to existing docs structure
- Integration examples with existing CLI features
- Troubleshooting guides referencing existing systems
