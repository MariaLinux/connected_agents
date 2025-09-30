use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub nodes: Vec<Node>,
    pub connections: HashMap<String, Vec<Connection>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub id: u32,
    pub name: String,
    pub plugin: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub parameters: serde_yaml::Value,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub to: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowDataType {
    Unknown,
    None,
    PlainText,
    Json,
    Xml,
    Html,
}

// Data that flows between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowData {
    pub data_type: WorkflowDataType,
    pub items: Vec<serde_json::Value>,
    pub text: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

impl WorkflowData {
    pub fn new() -> Self {
        Self {
            data_type: WorkflowDataType::Json,
            items: vec![serde_json::json!({"message": "workflow started"})],
            text: None,
            bytes: None,
        }
    }

    pub fn clear() -> Self {
        Self { data_type: WorkflowDataType::None, items: vec![], text: None, bytes: None }
    }
}