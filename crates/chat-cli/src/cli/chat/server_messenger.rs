use tokio::sync::mpsc::{
    Receiver,
    Sender,
    channel,
};

use crate::mcp_client::{
    Messenger,
    MessengerError,
    PromptsListResult,
    ResourceTemplatesListResult,
    ResourcesListResult,
    ToolsListResult,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum UpdateEventMessage {
    ToolsListResult {
        server_name: String,
        result: eyre::Result<ToolsListResult>,
        pid: Option<u32>,
    },
    PromptsListResult {
        server_name: String,
        result: eyre::Result<PromptsListResult>,
        pid: Option<u32>,
    },
    ResourcesListResult {
        server_name: String,
        result: eyre::Result<ResourcesListResult>,
        pid: Option<u32>,
    },
    ResourceTemplatesListResult {
        server_name: String,
        result: eyre::Result<ResourceTemplatesListResult>,
        pid: Option<u32>,
    },
    InitStart {
        server_name: String,
        pid: Option<u32>,
    },
    Deinit {
        server_name: String,
        pid: Option<u32>,
    },
}

#[derive(Clone, Debug)]
pub struct ServerMessengerBuilder {
    pub update_event_sender: Sender<UpdateEventMessage>,
}

impl ServerMessengerBuilder {
    pub fn new(capacity: usize) -> (Receiver<UpdateEventMessage>, Self) {
        let (tx, rx) = channel::<UpdateEventMessage>(capacity);
        let this = Self {
            update_event_sender: tx,
        };
        (rx, this)
    }

    pub fn build_with_name(&self, server_name: String) -> ServerMessenger {
        ServerMessenger {
            server_name,
            update_event_sender: self.update_event_sender.clone(),
            pid: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerMessenger {
    pub server_name: String,
    pub update_event_sender: Sender<UpdateEventMessage>,
    pub pid: Option<u32>,
}

#[async_trait::async_trait]
impl Messenger for ServerMessenger {
    async fn send_tools_list_result(&self, result: eyre::Result<ToolsListResult>) -> Result<(), MessengerError> {
        Ok(self
            .update_event_sender
            .send(UpdateEventMessage::ToolsListResult {
                server_name: self.server_name.clone(),
                result,
                pid: self.pid,
            })
            .await
            .map_err(|e| MessengerError::Custom(e.to_string()))?)
    }

    async fn send_prompts_list_result(&self, result: eyre::Result<PromptsListResult>) -> Result<(), MessengerError> {
        Ok(self
            .update_event_sender
            .send(UpdateEventMessage::PromptsListResult {
                server_name: self.server_name.clone(),
                result,
                pid: self.pid,
            })
            .await
            .map_err(|e| MessengerError::Custom(e.to_string()))?)
    }

    async fn send_resources_list_result(
        &self,
        result: eyre::Result<ResourcesListResult>,
    ) -> Result<(), MessengerError> {
        Ok(self
            .update_event_sender
            .send(UpdateEventMessage::ResourcesListResult {
                server_name: self.server_name.clone(),
                result,
                pid: self.pid,
            })
            .await
            .map_err(|e| MessengerError::Custom(e.to_string()))?)
    }

    async fn send_resource_templates_list_result(
        &self,
        result: eyre::Result<ResourceTemplatesListResult>,
    ) -> Result<(), MessengerError> {
        Ok(self
            .update_event_sender
            .send(UpdateEventMessage::ResourceTemplatesListResult {
                server_name: self.server_name.clone(),
                result,
                pid: self.pid,
            })
            .await
            .map_err(|e| MessengerError::Custom(e.to_string()))?)
    }

    async fn send_init_msg(&self) -> Result<(), MessengerError> {
        Ok(self
            .update_event_sender
            .send(UpdateEventMessage::InitStart {
                server_name: self.server_name.clone(),
                pid: self.pid,
            })
            .await
            .map_err(|e| MessengerError::Custom(e.to_string()))?)
    }

    fn send_deinit_msg(&self) {
        let sender = self.update_event_sender.clone();
        let server_name = self.server_name.clone();
        let pid = self.pid;
        tokio::spawn(async move {
            let _ = sender.send(UpdateEventMessage::Deinit { server_name, pid }).await;
        });
    }

    fn duplicate(&self) -> Box<dyn Messenger> {
        Box::new(self.clone())
    }
}
